use crate::error::{DataError, Result};
use std::fs::File;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};
use std::path::Path;

/// Compression format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionFormat {
    Lzma,
    Zlib,
    None,
}

/// Write data as LZMA-compressed Postcard binary
pub fn write_lzma_bin<T: serde::Serialize>(path: &Path, data: &T) -> Result<()> {
    let bytes = postcard::to_stdvec(data)?;
    write_lzma_raw(path, &bytes)
}

/// Write raw bytes with LZMA compression
pub fn write_lzma_raw(path: &Path, data: &[u8]) -> Result<()> {
    let file = File::create(path).map_err(|e| DataError::Io(e))?;
    let mut writer = BufWriter::new(file);

    lzma_rs::xz_compress(&mut Cursor::new(data), &mut writer)
        .map_err(|e| DataError::Compression(format!("LZMA compression failed: {}", e)))?;

    writer.flush().map_err(|e| DataError::Io(e))?;

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

    let mut reader = BufReader::new(file);
    let mut decompressed = Vec::new();

    lzma_rs::xz_decompress(&mut reader, &mut decompressed)
        .map_err(|e| DataError::Decompression(format!("LZMA decompression failed: {}", e)))?;

    Ok(decompressed)
}

/// Attempt to decompress with multiple formats (LZMA, then Zlib fallback)
pub fn decompress_auto(data: &[u8]) -> Result<Vec<u8>> {
    // Try LZMA first
    let mut decompressed = Vec::new();
    if lzma_rs::xz_decompress(&mut Cursor::new(data), &mut decompressed).is_ok() {
        return Ok(decompressed);
    }

    // Fallback to Zlib
    decompressed.clear();
    let mut decoder = flate2::read::ZlibDecoder::new(Cursor::new(data));
    decoder
        .read_to_end(&mut decompressed)
        .map_err(|e| DataError::Decompression(format!("Auto-decompression failed: {}", e)))?;

    Ok(decompressed)
}

/// Attempt to read and decompress file with multiple formats
pub fn decompress_file_auto(path: &Path) -> Result<Vec<u8>> {
    let file = File::open(path)
        .map_err(|e| DataError::FileNotFound(format!("{}: {}", path.display(), e)))?;

    let mut reader = BufReader::new(file);
    let mut decompressed = Vec::new();

    // Try LZMA first
    if lzma_rs::xz_decompress(&mut reader, &mut decompressed).is_ok() {
        return Ok(decompressed);
    }

    // Fallback to Zlib
    decompressed.clear();
    let file = File::open(path)?;
    let mut decoder = flate2::read::ZlibDecoder::new(file);
    decoder.read_to_end(&mut decompressed).map_err(|e| {
        DataError::Decompression(format!(
            "Auto-decompression of {} failed: {}",
            path.display(),
            e
        ))
    })?;

    Ok(decompressed)
}

/// Validate that data is properly formatted (basic sanity checks)
pub fn validate_compressed_data(data: &[u8], min_size: usize) -> Result<()> {
    if data.is_empty() {
        return Err(DataError::Validation(
            "Compressed data is empty".to_string(),
        ));
    }

    if data.len() < min_size {
        return Err(DataError::Validation(format!(
            "Compressed data too small: {} bytes (minimum: {})",
            data.len(),
            min_size
        )));
    }

    Ok(())
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
