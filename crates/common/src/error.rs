use thiserror::Error;

/// Custom error type for data operations with enhanced context
#[derive(Error, Debug)]
pub enum DataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Compression error: {0}")]
    Compression(String),

    #[error("Decompression error: {0}")]
    Decompression(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Data integrity check failed: {0}")]
    IntegrityCheckFailed(String),
}

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
