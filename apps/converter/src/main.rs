//! The converter's entry point: load the configuration, run one conversion, exit.
//!
//! Everything it does is [`mp_stats_converter`]. This binary exists so the pipeline can be linked
//! by the integration tests without being run.

use anyhow::{Context, Result};
use mp_stats_config::ConverterConfig;
use mp_stats_converter::Converter;
use serde::Deserialize;

/// Everything the converter reads.
///
/// One block, `[converter]`, so the same `config.toml` can also carry the server's `[server]`
/// block without either binary having to know about the other's keys.
#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default)]
    converter: ConverterConfig,
}

fn main() -> Result<()> {
    // Layered: struct defaults, then `$MP_STATS_CONFIG`, then `MP_STATS_*`. See
    // `docs/CONFIGURATION.md`.
    let config: Config = mp_stats_config::load().context("loading configuration")?;

    Converter::from_config(&config.converter)?.convert()
}
