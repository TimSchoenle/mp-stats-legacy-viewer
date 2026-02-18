use anyhow::Result;
use std::fs;
use std::path::Path;

/// Safely copy directory recursively with error handling
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

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

/// Finalize output by moving staging to target
pub fn finalize_output(staging_dir: &Path, output_dir: &Path) -> Result<()> {
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }

    copy_dir_all(staging_dir, output_dir)?;
    fs::remove_dir_all(staging_dir)?;

    Ok(())
}
