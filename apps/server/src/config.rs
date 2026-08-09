//! The aggregate this binary deserialises out of the layered configuration.

use serde::Deserialize;

/// Everything the server reads.
///
/// One block, `[server]`, so the same `config.toml` can also carry the converter's
/// `[converter]` block without either binary having to know about the other's keys — an
/// unknown key is ignored by the layer, not rejected.
#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub(crate) server: mp_stats_config::ServerConfig,
}
