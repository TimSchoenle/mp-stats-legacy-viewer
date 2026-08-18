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

/// Set to `1` or `true` to have [`load`] report which layer supplied each key.
///
/// Read straight from the process environment rather than out of the configuration, because it
/// decides whether to describe the layers and so is answered before they exist. Declaring it
/// [reserved](Terrace::reserve) makes an attempt to supply it from a file or a mounted secret an
/// error at boot instead of a variable that is quietly never read.
const EXPLAIN_VAR: &str = "MP_STATS_EXPLAIN";

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
        .reserve(EXPLAIN_VAR)
}

/// Load a typed configuration aggregate.
///
/// With `MP_STATS_EXPLAIN` set, a report of every layer and the key each one supplied is written
/// to stderr — including when the load fails, which is the case it exists for: a key refused for
/// being supplied twice names the key, and the report names both of the sources holding it.
///
/// # Errors
/// Returns [`ConfigError`] if a value fails to parse, a file-backed source cannot be read, or
/// one key is supplied by more than one of the last three layers.
pub fn load<T: DeserializeOwned>() -> Result<T, ConfigError> {
    let terrace = terrace();
    let loaded = terrace.load();

    if explain_requested() {
        match terrace.explain() {
            Ok(explanation) => eprintln!("{explanation}"),
            // The load's own outcome is what the caller asked for; a report that cannot be
            // assembled says so and is not allowed to replace it.
            Err(error) => eprintln!("{EXPLAIN_VAR} is set, but the layers cannot be read: {error}"),
        }
    }

    loaded
}

/// Whether the boot-time layer report was asked for.
///
/// Two exact spellings rather than "anything non-empty": `MP_STATS_EXPLAIN=0` reads as off to
/// everyone who types it, and a variable left at `0` in a deployment must not go on printing the
/// shape of the configuration into the log forever.
fn explain_requested() -> bool {
    matches!(std::env::var(EXPLAIN_VAR).as_deref(), Ok("1" | "true"))
}

#[cfg(test)]
mod tests {
    use crate::{ConverterConfig, ServerConfig, loader::terrace};
    use serde::Deserialize;
    use std::path::Path;
    use terrace_config::explain::Layer;
    use terrace_config::testing::Harness;

    /// Stands in for the per-binary aggregates: the same two blocks, deserialised the same way.
    #[derive(Debug, Deserialize)]
    struct Sample {
        #[serde(default)]
        server: ServerConfig,
        #[serde(default)]
        converter: ConverterConfig,
    }

    /// A sandbox over the loader this crate hands the binaries: a scratch working directory, an
    /// empty environment, and both restored when the test returns.
    ///
    /// Every variable a test arranges is derived from that loader rather than typed out, so a
    /// renamed variable fails these tests instead of leaving them passing against a name nothing
    /// reads any more.
    fn harness() -> Harness {
        Harness::over(terrace())
    }

    /// The dialect, end to end: the prefix, the `__` nesting, and the defaults that fill in
    /// around what the environment supplied. `terrace-config` owns the layering and tests it;
    /// what this pins is that this crate wires it to the names an operator actually sets.
    #[test]
    fn env_overrides_and_defaults_apply() {
        harness().run(|jail| {
            jail.env_key("server.bind_addr", "127.0.0.1:9000");
            jail.env_key("converter.cache.enabled", false);

            let config: Sample = jail.load()?;

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
    ///
    /// Written into the sandbox's working directory rather than through `jail.config`, which
    /// would point `MP_STATS_CONFIG` at it — the default path is the thing under test.
    #[test]
    fn the_default_config_file_is_read() {
        harness().run(|jail| {
            jail.write(
                "config.toml",
                "[server]\nbind_addr = \"0.0.0.0:8081\"\ndata_dir = \"/srv/data\"\n",
            )?;

            let config: Sample = jail.load()?;

            assert_eq!(config.server.bind_addr.to_string(), "0.0.0.0:8081");
            assert_eq!(config.server.data_dir, Path::new("/srv/data"));
            Ok(())
        });
    }

    /// The environment outranks the TOML layer, so a container image can ship a baked config
    /// file and still be re-pointed at deploy time without rebuilding it.
    #[test]
    fn the_environment_outranks_the_toml_layer() {
        harness().run(|jail| {
            jail.config("[server]\ndist_dir = \"/dist\"\n")?;
            jail.env_key("server.dist_dir", "/srv/dist");

            let config: Sample = jail.load()?;

            assert_eq!(config.server.dist_dir, Path::new("/srv/dist"));
            Ok(())
        });
    }

    /// The `[server.csp.cloudflare]` block is the deepest nesting this configuration has, and
    /// `__` is the separator at every level of it — the spelling `docs/CONFIGURATION.md`
    /// documents, and the one an operator enabling a Cloudflare product actually types.
    #[test]
    fn the_csp_block_nests_three_deep() {
        harness().run(|jail| {
            jail.config("[server.csp.cloudflare]\nturnstile = true\n")?;
            jail.env_key("server.csp.cloudflare.script_nonce", true);

            let config: Sample = jail.load()?;

            assert!(config.server.csp.cloudflare.script_nonce);
            assert!(config.server.csp.cloudflare.turnstile);
            // A partial table leaves the rest of the block on its defaults: the policy is sent,
            // and no concession is made that was not asked for.
            assert!(config.server.csp.enabled);
            assert!(!config.server.csp.cloudflare.web_analytics);
            Ok(())
        });
    }

    /// One key supplied by both the environment and a mounted file fails the boot instead of
    /// being resolved by precedence — the layer that makes a half-migrated deployment loud.
    #[test]
    fn a_key_supplied_twice_is_refused() {
        harness().run(|jail| {
            jail.secret_key("server.data_dir", "/srv/data\n")?;
            jail.env_key("server.data_dir", "/other/data");

            let error = jail
                .load::<Sample>()
                .expect_err("a doubly-supplied key must fail the load");

            assert!(
                error.to_string().contains("data_dir"),
                "the error must name the key: {error}"
            );
            Ok(())
        });
    }

    /// What `MP_STATS_EXPLAIN` prints at boot, asserted on the report rather than on the value:
    /// a value that a deployment reads from a mounted file and a value it reads from a stale
    /// variable are the same value, and only the layer says which one is being run on.
    #[test]
    fn the_report_names_the_layer_a_value_came_from() {
        harness().run(|jail| {
            jail.secret_key("server.data_dir", "/srv/data\n")?;

            let explanation = jail.explain()?;
            let origin = explanation
                .origin("server.data_dir")
                .expect("the mounted key is reported");

            assert!(
                matches!(origin.effective(), Layer::SecretsFile(_)),
                "the mounted file is the effective layer, not {}",
                origin.effective()
            );
            Ok(())
        });
    }
}
