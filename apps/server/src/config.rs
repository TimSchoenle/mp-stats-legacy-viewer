//! The aggregate this binary deserialises out of the layered configuration.

use serde::Deserialize;

/// Everything the server reads.
///
/// Two blocks. `[server]` is this binary's alone; `[telemetry]` describes the process rather
/// than the job, and is spelled outside `[server]` so that a second binary reading it does not
/// have to read a table named after this one.
///
/// The converter's `[converter]` block can sit in the same file without either binary having to
/// know about the other's keys — an unknown key is ignored by the layer, not rejected.
#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub(crate) server: mp_stats_config::ServerConfig,
    #[serde(default)]
    pub(crate) telemetry: mp_stats_config::TelemetryConfig,
}
