//! Postcard inside LZMA, which is what every `.bin.xz` in the converted tree is.
//!
//! Only [`uncompress_lzma`] is reachable from the browser. Everything else takes a `Path`, and a
//! path on `wasm32-unknown-unknown` compiles and then fails at every call.

use crate::error::{DataError, Result};
use lzma_rust2::{XzOptions, XzReader, XzWriter};
use std::fs::File;
use std::io::{BufReader, BufWriter, Cursor, Read};
use std::path::Path;
use std::{fs, io};

/// Encodes `data` as Postcard and writes it compressed to `path`, creating the parent directories.
///
/// # Errors
///
/// If the value does not encode, a directory cannot be created, or the write fails part way.
pub fn write_lzma_bin<T: serde::Serialize>(path: &Path, data: &T) -> Result<()> {
    let bytes = postcard::to_stdvec(data)?;
    write_lzma_raw(path, &bytes)
}

/// Writes `data` compressed to `path`, creating the parent directories.
///
/// The compressor runs at the library's default preset. An existing file at `path` is truncated.
///
/// # Errors
///
/// If a directory cannot be created, or the write fails part way.
pub fn write_lzma_raw(path: &Path, data: &[u8]) -> Result<()> {
    let mut reader = Cursor::new(data);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path).map_err(DataError::Io)?;
    let writer = BufWriter::new(file);

    let mut writer = XzWriter::new(writer, XzOptions::default())?;
    io::copy(&mut reader, &mut writer).map_err(DataError::Io)?;
    writer.finish().map_err(DataError::Io)?;

    Ok(())
}

/// Reads `path`, decompresses it and decodes the Postcard value inside.
///
/// # Errors
///
/// [`DataError::FileNotFound`] when the file cannot be opened, and
/// [`DataError::Deserialization`] when the bytes are not the type `T` asks for — which is what
/// reading a page written by an older converter looks like.
pub fn read_lzma_bin<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let decompressed = read_lzma_raw(path)?;
    postcard::from_bytes(&decompressed)
        .map_err(|e| DataError::Deserialization(format!("Postcard deserialization failed: {e}")))
}

/// Reads `path` and returns its decompressed bytes.
///
/// # Errors
///
/// [`DataError::FileNotFound`] when the file cannot be opened, and [`DataError::Io`] when the
/// stream is truncated or is not LZMA.
pub fn read_lzma_raw(path: &Path) -> Result<Vec<u8>> {
    let file = File::open(path)
        .map_err(|e| DataError::FileNotFound(format!("{}: {}", path.display(), e)))?;

    let reader = BufReader::new(file);

    uncompress_lzma(reader)
}

/// Decompresses an LZMA stream into memory, whole.
///
/// The reader is drained, so the output is as large as the file expands to. This is the one
/// function in this module the browser calls, over the bytes of a fetched response.
///
/// # Errors
///
/// [`DataError::Io`] when the stream is truncated or is not LZMA.
pub fn uncompress_lzma(reader: impl Read) -> Result<Vec<u8>> {
    let mut decompressed = Vec::new();

    let mut reader = XzReader::new(reader, true);
    io::copy(&mut reader, &mut decompressed)?;

    Ok(decompressed)
}

/// Reads a compressed file whatever its container. There is one container, so this is
/// [`read_lzma_raw`] under another name.
///
/// # Errors
///
/// As [`read_lzma_raw`].
pub fn decompress_file_auto(path: &Path) -> Result<Vec<u8>> {
    read_lzma_raw(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_lzma_roundtrip() {
        let test_data: HashMap<String, u32> =
            vec![("test1".to_string(), 100), ("test2".to_string(), 200)]
                .into_iter()
                .collect();

        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_lzma.bin");

        write_lzma_bin(&temp_file, &test_data).unwrap();
        let loaded: HashMap<String, u32> = read_lzma_bin(&temp_file).unwrap();

        assert_eq!(test_data, loaded);
        std::fs::remove_file(temp_file).ok();
    }
}
