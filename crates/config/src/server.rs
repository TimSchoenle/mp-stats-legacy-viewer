//! The HTTP server's block: where it listens and which directories it serves.

use crate::CspConfig;
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::PathBuf;

/// What the server binds and what it serves.
///
/// Every field has a working default, so a checkout with no `config.toml` at all runs against the
/// layout the frontend build and the converter produce.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(serde::Serialize, terrace_config::schema::Describe)
)]
pub struct ServerConfig {
    /// Address the HTTP listener binds.
    ///
    /// Deserialised as a `SocketAddr`, so an unparseable value fails at boot rather than at
    /// `bind` time.
    #[serde(default = "ServerConfig::default_bind_addr")]
    pub bind_addr: SocketAddr,
    /// Directory holding the built frontend.
    ///
    /// Its `index.html` is both the SPA entry point and the fallback for unknown routes; the
    /// server refuses to start without it.
    #[serde(default = "ServerConfig::default_dist_dir")]
    pub dist_dir: PathBuf,
    /// Directory holding the converter's output, mounted at `/data`.
    #[serde(default = "ServerConfig::default_data_dir")]
    pub data_dir: PathBuf,
    /// The `Content-Security-Policy` attached to every document the server answers with. The
    /// policy is derived from `dist_dir`'s `index.html` at startup; these keys only decide what
    /// it makes room for.
    #[serde(default)]
    #[cfg_attr(feature = "config-schema", config(nested))]
    pub csp: CspConfig,
}

impl ServerConfig {
    fn default_bind_addr() -> SocketAddr {
        SocketAddr::from(([0, 0, 0, 0], 8080))
    }

    fn default_dist_dir() -> PathBuf {
        PathBuf::from("dist")
    }

    fn default_data_dir() -> PathBuf {
        PathBuf::from("data")
    }

    /// Path the SPA entry point is expected at.
    #[must_use]
    pub fn index_path(&self) -> PathBuf {
        self.dist_dir.join("index.html")
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: Self::default_bind_addr(),
            dist_dir: Self::default_dist_dir(),
            data_dir: Self::default_data_dir(),
            csp: CspConfig::default(),
        }
    }
}
