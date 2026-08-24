//! Reading a dump file, and the checks a run makes before it reads anything.

use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Parses `path` as JSON into `T`.
///
/// # Errors
///
/// If the file cannot be opened, is empty, is larger than `MAX_JSON_SIZE`, or does not parse. The
/// size ceiling is what stops a corrupt dump from being read into memory whole before it fails.
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

/// Checks that `path` is an existing directory, naming it `description` if it is not.
///
/// # Errors
///
/// If the path does not exist, or exists and is not a directory.
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

/// Refuses an input and output directory that resolve to the same place.
///
/// Passing is not proof they differ: a path that does not exist yet cannot be canonicalized, and
/// an unresolvable pair is accepted rather than guessed at.
///
/// # Errors
///
/// If both paths resolve and resolve to the same directory.
pub fn validate_different_paths(in_path: &Path, out_path: &Path) -> Result<()> {
    if let (Ok(in_canon), Ok(out_canon)) = (in_path.canonicalize(), out_path.canonicalize())
        && in_canon == out_canon
    {
        anyhow::bail!(
            "Input and output directories must be different for safety: {} == {}",
            in_canon.display(),
            out_canon.display()
        );
    }
    Ok(())
}
