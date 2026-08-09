//! The typed configuration surface of the platform, plus the MP Stats dialect of the layered
//! loader.
//!
//! The layering is [`terrace_config`]'s. Lowest precedence first: struct defaults, TOML at
//! `$MP_STATS_CONFIG` (a file, or every `*.toml` in it when it names a directory),
//! `MP_STATS_`-prefixed `__`-nested environment variables, `$MP_STATS_SECRETS_DIR`, and
//! `MP_STATS_<KEY>_FILE` indirection. The last three are mutually exclusive per key: a key
//! supplied by two of them is refused at boot rather than resolved by precedence.
//!
//! Each binary owns the aggregate it deserialises and reads only its own block, so one
//! `config.toml` can describe the whole platform without either binary having to parse the
//! other's section.

mod converter;
mod csp;
mod loader;
mod server;

pub use converter::{CacheConfig, ConverterConfig};
pub use csp::{CloudflareConfig, CspConfig};
pub use loader::{ConfigError, load, terrace};
pub use server::ServerConfig;
