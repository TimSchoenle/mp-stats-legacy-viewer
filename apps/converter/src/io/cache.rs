use crate::io::copy_dir_all;
use anyhow::{Context, Result};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;

/// Incremental conversion cache.
///
/// The converter is fully deterministic for a given input directory, so its
/// output can be reused verbatim whenever the input is unchanged. This avoids
/// re-running the expensive decompression/parsing/recompression pipeline (for
/// example on Docker rebuilds where only unrelated source code changed).
///
/// Outputs are cached per logical key (one per platform edition) alongside a
/// fingerprint of the corresponding input directory.
pub struct ConversionCache {
    root: PathBuf,
    enabled: bool,
}

/// Version of the converter's *output* schema/format.
///
/// This is mixed into every input fingerprint so that a change to the
/// serialized data model (e.g. adding fields to `GameLeaderboardData` or
/// `LeaderboardMeta`) invalidates previously cached output, even when the raw
/// input data is byte-for-byte unchanged. Bump this whenever the produced
/// binaries change in a way that older readers/newer code cannot consume.
const OUTPUT_SCHEMA_VERSION: u64 = 2;

impl ConversionCache {
    /// Create an enabled cache rooted at `root`.
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            enabled: true,
        }
    }

    /// Create a disabled cache (no-op restore/store).
    pub fn disabled() -> Self {
        Self {
            root: PathBuf::new(),
            enabled: false,
        }
    }

    /// Resolve the cache configuration from the environment.
    ///
    /// * `CONVERTER_NO_CACHE` (any non-empty value) disables caching entirely.
    /// * `CONVERTER_CACHE_DIR` overrides the cache location.
    /// * Otherwise defaults to `target/converter_cache`.
    pub fn from_env() -> Self {
        if std::env::var_os("CONVERTER_NO_CACHE").is_some_and(|v| !v.is_empty()) {
            return Self::disabled();
        }

        let root = std::env::var_os("CONVERTER_CACHE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("target/converter_cache"));

        Self::new(root)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Compute a stable fingerprint of an input directory from the relative
    /// path, byte length and modification time of every file it contains.
    pub fn fingerprint_dir(input: &Path) -> Result<u64> {
        let mut files: Vec<(String, u64, u64)> = Vec::new();

        for entry in WalkDir::new(input).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }

            let rel = entry
                .path()
                .strip_prefix(input)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .replace('\\', "/");

            let meta = entry.metadata()?;
            let len = meta.len();
            let mtime = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            files.push((rel, len, mtime));
        }

        // Sort for a deterministic order independent of filesystem traversal.
        files.sort();

        let mut hasher = DefaultHasher::new();
        // Bind the fingerprint to the output schema version so that changes to
        // the serialized data model invalidate previously cached output even
        // when the raw input is unchanged.
        OUTPUT_SCHEMA_VERSION.hash(&mut hasher);
        files.len().hash(&mut hasher);
        for file in &files {
            file.hash(&mut hasher);
        }

        Ok(hasher.finish())
    }

    fn fingerprint_path(&self, key: &str) -> PathBuf {
        self.root.join(format!("{key}.fingerprint"))
    }

    fn output_path(&self, key: &str) -> PathBuf {
        self.root.join(key)
    }

    fn stored_fingerprint(&self, key: &str) -> Option<u64> {
        std::fs::read_to_string(self.fingerprint_path(key))
            .ok()
            .and_then(|raw| raw.trim().parse::<u64>().ok())
    }

    /// Restore a cached output for `key` into `dest` when the stored
    /// fingerprint matches `fingerprint`. Returns `true` on a cache hit.
    pub fn restore(&self, key: &str, fingerprint: u64, dest: &Path) -> Result<bool> {
        if !self.enabled {
            return Ok(false);
        }

        let output = self.output_path(key);
        if !output.exists() || self.stored_fingerprint(key) != Some(fingerprint) {
            return Ok(false);
        }

        copy_dir_all(&output, dest)
            .with_context(|| format!("restoring cached output for '{key}'"))?;
        Ok(true)
    }

    /// Persist `src` as the cached output for `key`, recording `fingerprint`.
    pub fn store(&self, key: &str, fingerprint: u64, src: &Path) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let output = self.output_path(key);
        if output.exists() {
            let _ = std::fs::remove_dir_all(&output);
        }
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)?;
        }

        copy_dir_all(src, &output).with_context(|| format!("caching output for '{key}'"))?;
        std::fs::write(self.fingerprint_path(key), fingerprint.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unique_dir(tag: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("mp_stats_cache_test_{tag}_{nanos}"))
    }

    /// A cached output is reused only when the stored fingerprint matches.
    /// A different fingerprint (e.g. after the output schema changed) must be
    /// treated as a cache miss so that stale binaries are never restored.
    #[test]
    fn restore_misses_on_fingerprint_mismatch() {
        let root = unique_dir("root");
        let src = unique_dir("src");
        let dest = unique_dir("dest");

        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("game.bin.xz"), b"payload").unwrap();

        let cache = ConversionCache::new(root.clone());
        cache.store("java", 111, &src).unwrap();

        // Same fingerprint -> hit.
        assert!(cache.restore("java", 111, &dest).unwrap());
        assert!(dest.join("game.bin.xz").exists());

        // Different fingerprint (schema/version changed) -> miss.
        let dest2 = unique_dir("dest2");
        assert!(!cache.restore("java", 222, &dest2).unwrap());
        assert!(!dest2.exists());

        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(&src);
        let _ = std::fs::remove_dir_all(&dest);
    }

    /// The schema version is part of the fingerprint, so two otherwise-identical
    /// inputs must hash differently across schema versions. We verify the
    /// version contributes by recomputing the hash with and without it.
    #[test]
    fn fingerprint_includes_schema_version() {
        use std::hash::{Hash, Hasher};

        let input = unique_dir("fp");
        std::fs::create_dir_all(&input).unwrap();
        std::fs::write(input.join("a.bin"), b"hello").unwrap();

        let with_version = ConversionCache::fingerprint_dir(&input).unwrap();

        // Recompute the same hash but without the version component.
        let mut files: Vec<(String, u64, u64)> = Vec::new();
        for entry in WalkDir::new(&input).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() {
                continue;
            }
            let rel = entry
                .path()
                .strip_prefix(&input)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .replace('\\', "/");
            let meta = entry.metadata().unwrap();
            let mtime = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            files.push((rel, meta.len(), mtime));
        }
        files.sort();
        let mut hasher = DefaultHasher::new();
        files.len().hash(&mut hasher);
        for file in &files {
            file.hash(&mut hasher);
        }
        let without_version = hasher.finish();

        assert_ne!(
            with_version, without_version,
            "schema version must contribute to the fingerprint"
        );

        let _ = std::fs::remove_dir_all(&input);
    }
}
