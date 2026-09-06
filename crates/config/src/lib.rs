//! The typed configuration surface of the platform, plus the MP Stats dialect of the layered
//! loader.
//!
//! The layering is [`terrace_config`]'s. Lowest precedence first: struct defaults, TOML at
//! `$MP_STATS_CONFIG` (a file, or every `*.toml` in it when it names a directory),
//! `MP_STATS_`-prefixed `__`-nested environment variables, `$MP_STATS_SECRETS_DIR`, and
//! `MP_STATS_<KEY>_FILE` indirection. The last three are mutually exclusive per key: a key
//! supplied by two of them is refused at boot rather than resolved by precedence.
//!
//! Each binary owns the aggregate it deserialises and reads only the blocks it uses, so one
//! `config.toml` can describe the whole platform without either binary having to parse the
//! other's section. `[telemetry]` is the one block that is not a single binary's: it describes
//! the process rather than the job, and the server reads it alongside `[server]`.
//! `MP_STATS_EXPLAIN=1` makes either of them report which layer supplied each key before it does
//! anything with the values.
//!
//! Every block below is closed — `#[serde(deny_unknown_fields)]` — so a key inside one that no
//! binary reads fails the boot instead of being ignored. The *aggregates* stay open, and have to:
//! one `config.toml` describes the whole platform, so each binary has to tolerate the other's
//! block. Closing the blocks themselves is what turns a misspelt `dist_dr` from a silent default
//! into a message naming it. It also makes the published contract true rather than optimistic:
//! that document already reports `additionalProperties: false` for every level, from the
//! generator's `closed` option, while the loader accepted the undeclared key anyway. No layer can
//! supply a key that is not a field — the environment layer filters the reserved variables and
//! every `MP_STATS_<KEY>_FILE` out before splitting, and the indirection and secrets layers key
//! on the value's own path.
//!
//! The blocks below are also the source the documentation is generated from. Under the
//! `config-schema` feature every struct here derives `Describe`, and
//! `examples/config-schema.rs` renders the tables in `README.md` and `docs/CONFIGURATION.md`
//! and the whole of `config.example.toml` out of them — which is why each field's `///` comment
//! opens with the sentence an operator needs: the first paragraph is the cell.

mod converter;
mod csp;
mod loader;
mod server;
mod telemetry;

pub use converter::{CacheConfig, ConverterConfig};
pub use csp::{CloudflareConfig, CspConfig};
pub use loader::{ConfigError, load, terrace};
pub use server::ServerConfig;
pub use telemetry::{SentryConfig, SentryLevel, TelemetryConfig};
