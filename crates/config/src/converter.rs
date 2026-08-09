//! The converter's block: what it reads, where it writes, and how it caches.

use serde::Deserialize;
use std::path::PathBuf;

/// Input, output and cache settings for one converter run.
#[derive(Debug, Clone, Deserialize)]
pub struct ConverterConfig {
    /// Directory holding the raw per-edition data dumps.
    #[serde(default = "ConverterConfig::default_input_dir")]
    pub input_dir: PathBuf,
    /// Directory the optimized output is written to. Must differ from [`Self::input_dir`],
    /// which the converter verifies before it starts.
    #[serde(default = "ConverterConfig::default_output_dir")]
    pub output_dir: PathBuf,
    /// Incremental output cache.
    #[serde(default)]
    pub cache: CacheConfig,
}

impl ConverterConfig {
    fn default_input_dir() -> PathBuf {
        PathBuf::from("data")
    }

    fn default_output_dir() -> PathBuf {
        PathBuf::from("target/converted_data")
    }
}

impl Default for ConverterConfig {
    fn default() -> Self {
        Self {
            input_dir: Self::default_input_dir(),
            output_dir: Self::default_output_dir(),
            cache: CacheConfig::default(),
        }
    }
}

/// The converter's incremental output cache.
///
/// The pipeline is deterministic for a given input directory, so an edition whose input is
/// unchanged can be restored verbatim instead of re-converted. This is what keeps a Docker
/// rebuild that touched only unrelated source code from re-running the whole pipeline.
#[derive(Debug, Clone, Deserialize)]
pub struct CacheConfig {
    /// Restore from and store into [`Self::dir`]. When `false` every run is a full conversion.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Where cached output and its input fingerprints live.
    #[serde(default = "CacheConfig::default_dir")]
    pub dir: PathBuf,
}

impl CacheConfig {
    fn default_dir() -> PathBuf {
        PathBuf::from("target/converter_cache")
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            dir: Self::default_dir(),
        }
    }
}

fn default_true() -> bool {
    true
}
