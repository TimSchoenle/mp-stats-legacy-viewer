//! Reusing a previous run's output when the dumps it was made from have not moved.

use crate::io::{copy_dir_all, link_or_copy_dir_all};
use anyhow::{Context, Result};
use mp_stats_config::CacheConfig;
use rayon::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;

/// A directory of previous outputs, one entry per edition, each beside a fingerprint of the input
/// it was made from.
///
/// Reuse is sound because the conversion is deterministic: the same dumps produce the same bytes,
/// which the integration tests assert by comparing a cached run against an uncached one. A
/// disabled cache answers every restore with a miss and stores nothing, so the pipeline around it
/// is the same code either way.
pub struct ConversionCache {
    root: PathBuf,
    enabled: bool,
}

/// Bumped by hand whenever the written records change shape.
///
/// It is hashed into every fingerprint, so raising it invalidates every stored output even though
/// no dump moved. Nothing derives it: a field added to a record the converter writes is a change
/// this constant has to be told about, and forgetting serves the old bytes to a reader that
/// cannot decode them.
const OUTPUT_SCHEMA_VERSION: u64 = 3;

impl ConversionCache {
    /// An enabled cache storing its entries under `root`, which is created on the first store.
    #[must_use]
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            enabled: true,
        }
    }

    /// A cache that misses on every restore and keeps nothing, so a run always converts in full.
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            root: PathBuf::new(),
            enabled: false,
        }
    }

    /// Builds the cache the `[converter.cache]` block asks for.
    #[must_use]
    pub fn from_config(config: &CacheConfig) -> Self {
        if config.enabled {
            Self::new(config.dir.clone())
        } else {
            Self::disabled()
        }
    }

    /// Whether restores and stores do anything.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Hashes an input directory into the number a stored output is matched against.
    ///
    /// It covers the relative path, byte length and whole-second modification time of every file,
    /// sorted so filesystem traversal order cannot change it, plus the output schema version.
    /// Contents are never read, so a file edited within the same second and back to the same
    /// length fingerprints the same as before it was touched.
    ///
    /// # Errors
    ///
    /// Never. An unreadable file is skipped rather than failing the walk, and the result type is
    /// here for the callers that thread one.
    pub fn fingerprint_dir(input: &Path) -> Result<u64> {
        // Collect the file paths in a single walk, then read their metadata in
        // parallel. The per-file `stat` syscalls dominate the cost for large
        // input trees (thousands of files) and are especially slow over
        // bind-mounted/networked filesystems, so fanning them out across cores
        // is a large win over a sequential walk.
        let paths: Vec<PathBuf> = WalkDir::new(input)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|e| e.file_type().is_file())
            .map(walkdir::DirEntry::into_path)
            .collect();

        let mut files: Vec<(String, u64, u64)> = paths
            .par_iter()
            .filter_map(|path| {
                let rel = path
                    .strip_prefix(input)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .replace('\\', "/");

                let meta = std::fs::metadata(path).ok()?;
                let len = meta.len();
                let mtime = meta
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                    .map_or(0, |d| d.as_secs());

                Some((rel, len, mtime))
            })
            .collect();

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

    /// Materializes the stored output for `key` at `dest` and returns `true`, or returns `false`
    /// and writes nothing.
    ///
    /// A miss is the answer for a disabled cache, an absent entry and a fingerprint that does not
    /// match, and the caller cannot tell which — all three mean the same thing to it.
    ///
    /// # Errors
    ///
    /// If the entry matches and cannot be linked or copied to `dest`.
    pub fn restore(&self, key: &str, fingerprint: u64, dest: &Path) -> Result<bool> {
        if !self.enabled {
            return Ok(false);
        }

        let output = self.output_path(key);
        if !output.exists() || self.stored_fingerprint(key) != Some(fingerprint) {
            return Ok(false);
        }

        link_or_copy_dir_all(&output, dest)
            .with_context(|| format!("restoring cached output for '{key}'"))?;
        Ok(true)
    }

    /// Replaces the stored output for `key` with a copy of `src`, and records `fingerprint`
    /// beside it.
    ///
    /// # Errors
    ///
    /// If the copy fails or the fingerprint cannot be written. The caller treats both as a
    /// warning: a run whose output could not be cached has still produced its output.
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

    /// Recomputing the hash without the schema version gives a different number, which is what
    /// makes a schema change miss the cache rather than restore a tree in the previous shape.
    #[test]
    fn fingerprint_includes_schema_version() {
        use std::hash::{Hash, Hasher};

        let input = unique_dir("fp");
        std::fs::create_dir_all(&input).unwrap();
        std::fs::write(input.join("a.bin"), b"hello").unwrap();

        let with_version = ConversionCache::fingerprint_dir(&input).unwrap();

        // Recompute the same hash but without the version component.
        let mut files: Vec<(String, u64, u64)> = Vec::new();
        for entry in WalkDir::new(&input)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
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
                .map_or(0, |d| d.as_secs());
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
