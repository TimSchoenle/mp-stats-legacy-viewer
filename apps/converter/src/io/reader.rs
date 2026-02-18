use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Safely read and parse JSON file with validation
pub fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let file =
        File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;

    let metadata = file
        .metadata()
        .with_context(|| format!("Failed to get file metadata: {}", path.display()))?;

    // Basic validation: check if file is not empty
    if metadata.len() == 0 {
        anyhow::bail!("File is empty: {}", path.display());
    }

    // Check file size (prevent loading extremely large files)
    const MAX_JSON_SIZE: u64 = 100 * 1024 * 1024; // 100 MB
    if metadata.len() > MAX_JSON_SIZE {
        anyhow::bail!(
            "File too large: {} ({} bytes, max {} bytes)",
            path.display(),
            metadata.len(),
            MAX_JSON_SIZE
        );
    }

    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)
        .with_context(|| format!("Failed to parse JSON from: {}", path.display()))?;

    Ok(data)
}

/// Check if path exists and is a valid directory
pub fn validate_directory(path: &Path, description: &str) -> Result<()> {
    if !path.exists() {
        anyhow::bail!(
            "{} directory does not exist: {}",
            description,
            path.display()
        );
    }

    if !path.is_dir() {
        anyhow::bail!("{} is not a directory: {}", description, path.display());
    }

    Ok(())
}

/// Check if two paths are different (safety check for in-place operations)
pub fn validate_different_paths(in_path: &Path, out_path: &Path) -> Result<()> {
    if let (Ok(in_canon), Ok(out_canon)) = (in_path.canonicalize(), out_path.canonicalize()) {
        if in_canon == out_canon {
            anyhow::bail!(
                "Input and output directories must be different for safety: {} == {}",
                in_canon.display(),
                out_canon.display()
            );
        }
    }
    Ok(())
}
