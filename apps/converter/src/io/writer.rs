use anyhow::{Context, Result};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Safely copy a directory tree.
///
/// The converter's output (and therefore the conversion cache) consists of many
/// thousands of small files, for which a naive sequential recursive copy is
/// extremely slow. Instead we walk the tree once - creating every destination
/// directory up-front - and then copy all files in parallel via rayon, which is
/// dramatically faster on multi-core machines and over networked/bind-mounted
/// filesystems (e.g. Docker build mounts).
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

/// Materialize a directory tree at `dst` from `src`, preferring hard links over
/// byte copies.
///
/// Restoring a cache hit only needs the cached files to *appear* at the
/// destination; their contents are never mutated in place afterwards. A hard
/// link makes the destination entry point at the same inode as the cached file,
/// which is essentially free regardless of file size. When hard linking isn't
/// possible (most commonly because `src` and `dst` are on different filesystems,
/// e.g. a dedicated Docker cache mount) we transparently fall back to a normal
/// copy on a per-file basis.
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

/// Setup staging directory (clean slate)
pub fn setup_staging_directory(staging_dir: &Path) -> Result<()> {
    if staging_dir.exists() {
        fs::remove_dir_all(staging_dir)?;
    }
    fs::create_dir_all(staging_dir)?;
    Ok(())
}

/// Finalize output by moving staging to target.
///
/// Prefer an atomic `rename`, which is effectively instant and performs no data
/// copy at all. This works whenever staging and output live on the same
/// filesystem (the common case). When they don't (e.g. crossing a device/mount
/// boundary, which surfaces as an OS error), fall back to a recursive copy plus
/// cleanup.
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
