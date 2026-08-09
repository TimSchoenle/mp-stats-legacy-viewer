//! The MP Stats dialect of [`terrace_config`].
//!
//! The layering itself — the TOML fragments, the `MP_STATS_*` environment layer, the
//! secrets-directory provider, the `_FILE` indirection and the shadow-key rejection — belongs
//! to `terrace-config`. What stays here is the one thing that is ours: which environment names
//! this deployment spells.

use serde::de::DeserializeOwned;
use terrace_config::Terrace;

pub use terrace_config::Error as ConfigError;

/// The prefix every configuration variable carries.
const PREFIX: &str = "MP_STATS_";

/// The loader both binaries boot through.
///
/// Layers, lowest precedence first: struct defaults, TOML at `$MP_STATS_CONFIG` (a file, or
/// every `*.toml` in it if it names a directory, merged in file-name order), `MP_STATS_`-prefixed
/// `__`-nested environment variables, `$MP_STATS_SECRETS_DIR`, and `MP_STATS_<KEY>_FILE`
/// indirection. The last three are mutually exclusive per key: a key supplied by two of them is
/// refused at boot rather than resolved by precedence.
///
/// A missing config file is not an error — running with none at all is the normal development
/// case, where every value comes from the struct defaults.
///
/// Both names below are the ones `Terrace::new(PREFIX)` would have derived anyway, spelled out
/// as literals on purpose: they are the two variables an operator sets before any other layer
/// exists, and `docs/CONFIGURATION.md` documents them against these lines rather than against a
/// derivation inside a dependency.
#[must_use]
pub fn terrace() -> Terrace {
    Terrace::new(PREFIX)
        .config_var("MP_STATS_CONFIG")
        .secrets_dir_var("MP_STATS_SECRETS_DIR")
}

/// Load a typed configuration aggregate.
///
/// # Errors
/// Returns [`ConfigError`] if a value fails to parse, a file-backed source cannot be read, or
/// one key is supplied by more than one of the last three layers.
pub fn load<T: DeserializeOwned>() -> Result<T, ConfigError> {
    terrace().load()
}

#[cfg(test)]
mod tests {
    use crate::{ConverterConfig, ServerConfig, load};
    use serde::Deserialize;
    use std::path::Path;

    /// Stands in for the per-binary aggregates: the same two blocks, deserialised the same way.
    #[derive(Debug, Deserialize)]
    struct Sample {
        #[serde(default)]
        server: ServerConfig,
        #[serde(default)]
        converter: ConverterConfig,
    }

    /// The dialect, end to end: the prefix, the `__` nesting, and the defaults that fill in
    /// around what the environment supplied. `terrace-config` owns the layering and tests it;
    /// what this pins is that this crate wires it to the names an operator actually sets.
    #[test]
    // `figment::Jail::expect_with` fixes the closure's error type to the large `figment::Error`.
    #[expect(
        clippy::result_large_err,
        reason = "figment::Jail::expect_with fixes the closure's error type"
    )]
    fn env_overrides_and_defaults_apply() {
        figment::Jail::expect_with(|jail| {
            jail.clear_env();
            jail.set_env("MP_STATS_SERVER__BIND_ADDR", "127.0.0.1:9000");
            jail.set_env("MP_STATS_CONVERTER__CACHE__ENABLED", "false");

            let config: Sample = load().map_err(|e| e.to_string()).unwrap();

            assert_eq!(config.server.bind_addr.to_string(), "127.0.0.1:9000");
            assert!(!config.converter.cache.enabled);
            // Untouched fields keep the defaults compiled into the structs.
            assert_eq!(config.server.dist_dir, Path::new("dist"));
            assert_eq!(config.converter.input_dir, Path::new("data"));
            Ok(())
        });
    }

    /// A `config.toml` next to the binary is read without anything being set, which is the
    /// whole point of the migration: the deployment describes itself in a file rather than in
    /// the process environment.
    #[test]
    // `figment::Jail::expect_with` fixes the closure's error type to the large `figment::Error`.
    #[expect(
        clippy::result_large_err,
        reason = "figment::Jail::expect_with fixes the closure's error type"
    )]
    fn the_default_config_file_is_read() {
        figment::Jail::expect_with(|jail| {
            jail.clear_env();
            jail.create_file(
                "config.toml",
                "[server]\nbind_addr = \"0.0.0.0:8081\"\ndata_dir = \"/srv/data\"\n",
            )?;

            let config: Sample = load().map_err(|e| e.to_string()).unwrap();

            assert_eq!(config.server.bind_addr.to_string(), "0.0.0.0:8081");
            assert_eq!(config.server.data_dir, Path::new("/srv/data"));
            Ok(())
        });
    }

    /// The environment outranks the TOML layer, so a container image can ship a baked config
    /// file and still be re-pointed at deploy time without rebuilding it.
    #[test]
    // `figment::Jail::expect_with` fixes the closure's error type to the large `figment::Error`.
    #[expect(
        clippy::result_large_err,
        reason = "figment::Jail::expect_with fixes the closure's error type"
    )]
    fn the_environment_outranks_the_toml_layer() {
        figment::Jail::expect_with(|jail| {
            jail.clear_env();
            jail.create_file("baked.toml", "[server]\ndist_dir = \"/dist\"\n")?;
            jail.set_env(
                "MP_STATS_CONFIG",
                jail.directory().join("baked.toml").display(),
            );
            jail.set_env("MP_STATS_SERVER__DIST_DIR", "/srv/dist");

            let config: Sample = load().map_err(|e| e.to_string()).unwrap();

            assert_eq!(config.server.dist_dir, Path::new("/srv/dist"));
            Ok(())
        });
    }

    /// One key supplied by both the environment and a mounted file fails the boot instead of
    /// being resolved by precedence — the layer that makes a half-migrated deployment loud.
    #[test]
    // `figment::Jail::expect_with` fixes the closure's error type to the large `figment::Error`.
    #[expect(
        clippy::result_large_err,
        reason = "figment::Jail::expect_with fixes the closure's error type"
    )]
    fn a_key_supplied_twice_is_refused() {
        figment::Jail::expect_with(|jail| {
            jail.clear_env();
            jail.create_dir("secrets")?;
            jail.create_file("secrets/server__data_dir", "/srv/data\n")?;
            jail.set_env(
                "MP_STATS_SECRETS_DIR",
                jail.directory().join("secrets").display(),
            );
            jail.set_env("MP_STATS_SERVER__DATA_DIR", "/other/data");

            let error = load::<Sample>().expect_err("a doubly-supplied key must fail the load");
            assert!(
                error.to_string().contains("data_dir"),
                "the error must name the key: {error}"
            );
            Ok(())
        });
    }
}
