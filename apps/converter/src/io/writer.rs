//! Moving trees of small files around, which is most of what a run's wall time goes on.

use anyhow::{Context, Result};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Copies the tree at `src` to `dst`, one walk to create the directories and then every file in
/// parallel.
///
/// Existing files at `dst` are overwritten and files not in `src` are left alone, so this merges
/// rather than replaces. The output is tens of thousands of small files, where the per-file
/// syscall dominates and a sequential copy is slow enough to notice on a Docker bind mount.
///
/// # Errors
///
/// If the walk fails, or if any directory or file cannot be created.
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    fs::create_dir_all(dst)?;

    // Single pass: create directories eagerly and queue files for a parallel copy.
    let mut files: Vec<(PathBuf, PathBuf)> = Vec::new();
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let rel = entry.path().strip_prefix(src).unwrap_or(entry.path());
        let target = dst.join(rel);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)
                .with_context(|| format!("creating directory {target:?}"))?;
        } else {
            files.push((entry.path().to_path_buf(), target));
        }
    }

    files.par_iter().try_for_each(|(from, to)| -> Result<()> {
        fs::copy(from, to).with_context(|| format!("copying {from:?} -> {to:?}"))?;
        Ok(())
    })?;

    Ok(())
}

/// Materializes the tree at `src` under `dst`, hard-linking each file and copying only where the
/// link fails.
///
/// A link costs the same whatever the file weighs, which is what makes restoring a cache hit
/// nearly free. It is safe here because a restored file is read and replaced, never written
/// through. Linking fails whenever the two sides are on different filesystems, which is the usual
/// case for a dedicated Docker cache mount, and each such file falls back to a copy on its own.
///
/// # Errors
///
/// If the walk fails, or if a file can neither be linked nor copied.
pub fn link_or_copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    fs::create_dir_all(dst)?;

    let mut files: Vec<(PathBuf, PathBuf)> = Vec::new();
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let rel = entry.path().strip_prefix(src).unwrap_or(entry.path());
        let target = dst.join(rel);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)
                .with_context(|| format!("creating directory {target:?}"))?;
        } else {
            files.push((entry.path().to_path_buf(), target));
        }
    }

    files.par_iter().try_for_each(|(from, to)| -> Result<()> {
        // A stale destination (e.g. from a previous partial run) would make
        // `hard_link` fail with AlreadyExists, so clear it first.
        let _ = fs::remove_file(to);
        if fs::hard_link(from, to).is_ok() {
            return Ok(());
        }
        fs::copy(from, to).with_context(|| format!("copying {from:?} -> {to:?}"))?;
        Ok(())
    })?;

    Ok(())
}

/// Empties `staging_dir` and recreates it, so a run never inherits a previous one's leftovers.
///
/// # Errors
///
/// If the existing directory cannot be removed or the new one cannot be created.
pub fn setup_staging_directory(staging_dir: &Path) -> Result<()> {
    if staging_dir.exists() {
        fs::remove_dir_all(staging_dir)?;
    }
    fs::create_dir_all(staging_dir)?;
    Ok(())
}

/// Replaces `output_dir` with `staging_dir`, by rename where the two share a filesystem and by
/// copy where they do not.
///
/// The existing `output_dir` is removed first, so the window in which the site has no data to
/// serve is the length of the rename in the common case and the length of the copy otherwise.
/// `staging_dir` does not exist afterwards either way.
///
/// # Errors
///
/// If the old output cannot be removed, or the staging tree can neither be renamed nor copied
/// into place.
pub fn finalize_output(staging_dir: &Path, output_dir: &Path) -> Result<()> {
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    if let Some(parent) = output_dir.parent() {
        fs::create_dir_all(parent)?;
    }

    if fs::rename(staging_dir, output_dir).is_ok() {
        return Ok(());
    }

    copy_dir_all(staging_dir, output_dir)?;
    fs::remove_dir_all(staging_dir)?;

    Ok(())
}
