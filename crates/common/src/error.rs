//! What can go wrong while reading or writing the converted tree.

use thiserror::Error;

/// A failure in reading, writing, compressing or decoding one file of the converted tree.
///
/// Every variant carries the path or the underlying message, because the converter runs over
/// thousands of files in parallel and a failure without one names nothing.
#[derive(Error, Debug)]
pub enum DataError {
    /// The filesystem refused the read or the write.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// A value could not be encoded.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Decompressed bytes did not decode into the type the caller asked for, which is what a
    /// reader older or newer than the tree it is reading looks like.
    #[error("Deserialization error: {0}")]
    Deserialization(String),

    /// Compression failed after the value encoded.
    #[error("Compression error: {0}")]
    Compression(String),

    /// The bytes are not the LZMA stream the path claimed they were.
    #[error("Decompression error: {0}")]
    Decompression(String),

    /// An input the conversion refuses, such as a UUID too short to name a shard.
    #[error("Validation error: {0}")]
    Validation(String),

    /// The file is not there. For a client fetching a page past the end of a board this is the
    /// ordinary outcome rather than a fault.
    #[error("File not found: {0}")]
    FileNotFound(String),

    /// The file exists and is not in the shape its path claims.
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// A consistency check over already-decoded data failed.
    #[error("Data integrity check failed: {0}")]
    IntegrityCheckFailed(String),
}

// `Compression`, `Decompression`, `InvalidFormat` and `IntegrityCheckFailed` have no constructor
// in this workspace. The lzma and Postcard paths both land in `Io`, `Serialization` or
// `Deserialization` instead.

/// [`Result`](std::result::Result) with [`DataError`] already filled in.
pub type Result<T> = std::result::Result<T, DataError>;

impl From<postcard::Error> for DataError {
    fn from(err: postcard::Error) -> Self {
        DataError::Serialization(err.to_string())
    }
}

impl From<serde_json::Error> for DataError {
    fn from(err: serde_json::Error) -> Self {
        DataError::Serialization(err.to_string())
    }
}
