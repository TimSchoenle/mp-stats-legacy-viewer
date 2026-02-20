use crate::error::{DataError, Result};
use lzma_rust2::{XzOptions, XzReader, XzWriter};
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, Cursor, Read};
use std::path::Path;

/// Write data as LZMA-compressed Postcard binary
pub fn write_lzma_bin<T: serde::Serialize>(path: &Path, data: &T) -> Result<()> {
    let bytes = postcard::to_stdvec(data)?;
    write_lzma_raw(path, &bytes)
}

/// Write raw bytes with LZMA compression
pub fn write_lzma_raw(path: &Path, data: &[u8]) -> Result<()> {
    let mut reader = Cursor::new(data);

    let file = File::create(path).map_err(|e| DataError::Io(e))?;
    let writer = BufWriter::new(file);

    // TODO: SHOULD PROBABLY RUN in with_preset(9) mode for prod release
    let mut writer = XzWriter::new(writer, XzOptions::default())?;
    io::copy(&mut reader, &mut writer).map_err(|e| DataError::Io(e))?;
    writer.finish().map_err(|e| DataError::Io(e))?;

    Ok(())
}

/// Read and decompress LZMA-compressed Postcard binary
pub fn read_lzma_bin<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let decompressed = read_lzma_raw(path)?;
    postcard::from_bytes(&decompressed)
        .map_err(|e| DataError::Deserialization(format!("Postcard deserialization failed: {}", e)))
}

/// Read and decompress LZMA-compressed raw bytes
pub fn read_lzma_raw(path: &Path) -> Result<Vec<u8>> {
    let file = File::open(path)
        .map_err(|e| DataError::FileNotFound(format!("{}: {}", path.display(), e)))?;

    let reader = BufReader::new(file);

    uncompress_lzma(reader)
}

pub fn uncompress_lzma(reader: impl Read) -> Result<Vec<u8>> {
    let mut decompressed = Vec::new();

    let mut reader = XzReader::new(reader, true);
    io::copy(&mut reader, &mut decompressed)?;

    Ok(decompressed)
}

/// Attempt to read and decompress file with multiple formats
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
