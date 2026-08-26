//! The observability block: how the process logs, and where it reports what went wrong.
//!
//! One block rather than a key under `[server]`, because none of it is a property of the HTTP
//! listener. A log filter and an error reporter belong to the *process*, and the second binary
//! this configuration already describes would read the same keys without either of them having
//! to learn about the other's block.
//!
//! The two halves are deliberately one stream. `tracing` is the only thing either binary writes
//! records to; [`SentryConfig`] is a second sink attached to that same stream, under thresholds
//! of its own. That is what makes [`TelemetryConfig::log_filter`] the surprise worth documenting
//! twice: a record the filter drops never reaches Sentry either.

use secrecy::SecretString;
use serde::Deserialize;

/// How the process logs, and whether it reports.
///
/// Every field has a working default, so a deployment that says nothing about telemetry logs at
/// `info` to a terminal and sends nothing anywhere.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(serde::Serialize, terrace_config::schema::Describe)
)]
pub struct TelemetryConfig {
    /// `RUST_LOG`-style filter deciding which records are emitted at all, for example
    /// `info,mp_stats_server=debug`.
    ///
    /// `RUST_LOG` itself outranks this key when it is set, which is the one place in this
    /// configuration where a bare environment variable wins over the layered loader: it is the
    /// variable every Rust operator already reaches for while a container is misbehaving, and a
    /// filter is not a value a deployment can be wrong about for long.
    ///
    /// It governs the Sentry sink too — see [`SentryConfig`].
    #[serde(default = "TelemetryConfig::default_log_filter")]
    pub log_filter: String,
    /// Emit one JSON object per record instead of human-readable lines.
    ///
    /// Off by default because the default deployment of this repository is `docker run` on a
    /// terminal. A cluster shipping logs to a collector wants it on.
    #[serde(default)]
    pub json_logs: bool,
    /// Sentry error reporting and performance tracing. Off unless configured; see
    /// [`SentryConfig`].
    #[serde(default)]
    #[cfg_attr(feature = "config-schema", config(nested))]
    pub sentry: SentryConfig,
}

impl TelemetryConfig {
    fn default_log_filter() -> String {
        "info".to_owned()
    }
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            log_filter: Self::default_log_filter(),
            json_logs: false,
            sentry: SentryConfig::default(),
        }
    }
}

/// How much of the `tracing` stream one Sentry sink takes.
///
/// Ordered by severity, so a threshold names the *least* severe record it accepts: `warn` means
/// `error` and `warn`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(serde::Serialize, terrace_config::schema::Describe)
)]
#[serde(rename_all = "lowercase")]
pub enum SentryLevel {
    /// Take nothing.
    Off,
    /// `error` only.
    #[default]
    Error,
    /// `error` and `warn`.
    Warn,
    /// Down to `info`.
    Info,
    /// Down to `debug`.
    Debug,
    /// Everything.
    Trace,
}

/// Sentry error reporting and performance tracing.
///
/// Off by default, and off in the `config.toml` the image ships: a DSN is an egress destination
/// for whatever a log line happens to carry, so switching it on is an operator's decision made
/// once per deployment. When [`Self::enabled`] is set the server refuses to boot without a
/// usable [`Self::dsn`] rather than starting with a reporter that reports nowhere — the same
/// posture the missing `index.html` and the unscannable shell already have.
///
/// The keys below are compiled into the binary only under its `sentry` feature, which is on by
/// default. A binary built with `--no-default-features` reads them and refuses to start while
/// [`Self::enabled`] is set, rather than ignoring a section an operator believes is working.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(serde::Serialize, terrace_config::schema::Describe)
)]
// `clippy::struct_excessive_bools` is not enabled in this workspace and so is not silenced
// here: the six flags below are independent operator toggles, one per
// `MP_STATS_TELEMETRY__SENTRY__*` variable, and collapsing them into a mode enum would mean
// inventing names for combinations nobody asked for.
pub struct SentryConfig {
    /// Initialise the Sentry client. `false` installs no client, no panic hook, no `tracing`
    /// layer and no HTTP middleware, so every other key here is inert and nothing leaves the
    /// process.
    #[serde(default)]
    pub enabled: bool,
    /// Ingest URL, `https://<key>@<host>/<project>`.
    ///
    /// A [`SecretString`]: the embedded key is a bearer credential for
    /// the project's ingest endpoint, and this struct is nested in a block that is logged with
    /// `?`. Prefer supplying it through the secrets directory or `_FILE` indirection rather than
    /// through the environment.
    ///
    /// Absent while [`Self::enabled`] is set is a boot failure, not a silent no-op.
    #[serde(default)]
    #[cfg_attr(feature = "config-schema", serde(serialize_with = "redact"))]
    #[cfg_attr(feature = "config-schema", config(secret))]
    pub dsn: Option<SecretString>,
    /// Environment tag on every event.
    ///
    /// Left unset it is derived from the build rather than guessed: `production` for a release
    /// binary — which every published image is — and `development` for a debug one. Set it
    /// explicitly for anything in between, such as a staging cluster running a release build.
    #[serde(default)]
    pub environment: Option<String>,
    /// Release tag on every event.
    ///
    /// Defaults to `mp-stats-legacy-viewer@v<version>`, spelled as the image tag spells it, so a
    /// regression is attributable to a deploy without an operator having to remember to set it.
    #[serde(default)]
    pub release: Option<String>,
    /// Host tag on every event.
    ///
    /// Left unset, Sentry reports none: the hostname of a replica is infrastructure detail that
    /// [`Self::send_default_pii`] would otherwise gate, and a `scratch` container's hostname
    /// names a pod that no longer exists by the time anyone reads the issue.
    #[serde(default)]
    pub server_name: Option<String>,
    /// Fraction of captured events actually sent, `0.0`–`1.0`.
    ///
    /// A blunt volume cap — it drops whole issues, not repetitions of one — so leave it at `1.0`
    /// unless a quota forces otherwise. A value outside the range fails the boot.
    #[serde(default = "SentryConfig::default_sample_rate")]
    pub sample_rate: f32,
    /// Fraction of traces this process **starts** that are recorded, `0.0`–`1.0`.
    ///
    /// `0.0` (the default) means it starts none of its own, which is the right figure for a
    /// static file server: a trace per asset request is volume without a question it answers.
    /// It does **not** mean this process is absent from a trace — a request arriving with a
    /// trace already sampled is continued regardless, which is what keeps one reader action
    /// readable across whatever sits in front of this server.
    #[serde(default)]
    pub traces_sample_rate: f32,
    /// Least severe `tracing` level reported as a Sentry **issue**.
    #[serde(default)]
    #[cfg_attr(feature = "config-schema", config(values))]
    pub capture_level: SentryLevel,
    /// Least severe `tracing` level kept as a **breadcrumb** — the trail attached to the next
    /// issue.
    ///
    /// Records at or above [`Self::capture_level`] become issues instead, so this threshold only
    /// ever describes the band below it.
    #[serde(default = "SentryConfig::default_breadcrumb_level")]
    #[cfg_attr(feature = "config-schema", config(values))]
    pub breadcrumb_level: SentryLevel,
    /// How many breadcrumbs one event carries.
    #[serde(default = "SentryConfig::default_max_breadcrumbs")]
    pub max_breadcrumbs: usize,
    /// Attach a stack trace to events that carry none of their own.
    #[serde(default = "default_true")]
    pub attach_stacktraces: bool,
    /// Send personally identifying data with every event: the client IP, the full request header
    /// set (`Cookie` included) and the resolved user.
    ///
    /// **Off, and worth leaving off.** A reader's IP address is exactly what a crash report does
    /// not need in order to be actionable, and Sentry is a third party for the purposes of
    /// whatever data policy this deployment publishes. On, it also widens what the HTTP
    /// middleware records, because the Sentry tower layer reads this same flag to decide whether
    /// to redact sensitive headers.
    #[serde(default)]
    pub send_default_pii: bool,
    /// Record request spans: one Sentry transaction per request, named by the *matched route*
    /// rather than the URI, so a `/data` path does not become its own transaction name.
    ///
    /// Whether a started transaction is *kept* is [`Self::traces_sample_rate`]'s decision, and
    /// that rate is `0.0` by default — so this key on its own costs one span per request and
    /// sends nothing. It is the switch for a deployment that should stay out of traces entirely.
    #[serde(default = "default_true")]
    pub http_transactions: bool,
    /// Copy `tracing` span fields onto the Sentry span as attributes.
    ///
    /// Off: a transaction is stored under a longer retention than a log line, and the span
    /// fields this server records name paths on disk.
    #[serde(default)]
    pub span_attributes: bool,
    /// How long process exit waits for queued events to drain.
    ///
    /// Spent only on a graceful shutdown, which this server performs on `SIGTERM`; a `SIGKILL`
    /// takes the queue with it whatever this says.
    #[serde(default = "SentryConfig::default_shutdown_timeout_secs")]
    pub shutdown_timeout_secs: u64,
    /// Print the SDK's own diagnostics to stderr. For proving a DSN works, not for running.
    #[serde(default)]
    pub debug: bool,
}

impl SentryConfig {
    fn default_sample_rate() -> f32 {
        1.0
    }

    fn default_breadcrumb_level() -> SentryLevel {
        SentryLevel::Info
    }

    fn default_max_breadcrumbs() -> usize {
        100
    }

    fn default_shutdown_timeout_secs() -> u64 {
        2
    }
}

impl Default for SentryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            dsn: None,
            environment: None,
            release: None,
            server_name: None,
            sample_rate: Self::default_sample_rate(),
            traces_sample_rate: 0.0,
            capture_level: SentryLevel::Error,
            breadcrumb_level: Self::default_breadcrumb_level(),
            max_breadcrumbs: Self::default_max_breadcrumbs(),
            attach_stacktraces: true,
            send_default_pii: false,
            http_transactions: true,
            span_attributes: false,
            shutdown_timeout_secs: Self::default_shutdown_timeout_secs(),
            debug: false,
        }
    }
}

fn default_true() -> bool {
    true
}

/// Serialise a DSN as its presence and nothing else.
///
/// Only the generator serialises this struct, and only to read the *defaults* out of a
/// `Default` value — where the DSN is `None`. The arm that renders `Some` therefore never runs
/// today, and it is written this way rather than as an `unreachable!` so that it cannot become
/// the line that prints a credential into `config.example.toml` if it ever does.
#[cfg(feature = "config-schema")]
fn redact<S: serde::Serializer>(
    dsn: &Option<SecretString>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match dsn {
        Some(_) => serializer.serialize_some("<redacted>"),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{SentryLevel, TelemetryConfig, loader::terrace};
    use secrecy::ExposeSecret as _;
    use serde::Deserialize;
    use terrace_config::testing::Harness;

    /// Stands in for the aggregate a binary deserialises: the one block, read the same way.
    #[derive(Debug, Deserialize)]
    struct Sample {
        #[serde(default)]
        telemetry: TelemetryConfig,
    }

    fn harness() -> Harness {
        Harness::over(terrace())
    }

    /// A deployment that says nothing about telemetry logs and reports nothing. The whole block
    /// is `#[serde(default)]` twice over — once on `telemetry`, once per key — so a missing
    /// section has to materialise rather than fail the boot of a deployment that predates it.
    #[test]
    fn an_unmentioned_section_is_off() {
        harness().run(|jail| {
            let config: Sample = jail.load()?;

            assert_eq!(config.telemetry.log_filter, "info");
            assert!(!config.telemetry.json_logs);
            assert!(!config.telemetry.sentry.enabled);
            assert!(config.telemetry.sentry.dsn.is_none());
            assert!(config.telemetry.sentry.traces_sample_rate.abs() < f32::EPSILON);
            assert!(!config.telemetry.sentry.send_default_pii);
            assert_eq!(config.telemetry.sentry.capture_level, SentryLevel::Error);
            assert_eq!(config.telemetry.sentry.breadcrumb_level, SentryLevel::Info);
            Ok(())
        });
    }

    /// The keys are two levels deep, which the `[server.csp.cloudflare]` block already proves
    /// the loader reaches. What this pins is the level spelling: `capture_level = "warn"` is
    /// lowercase in the file because the enum is `rename_all = "lowercase"`, and the tables
    /// generated from it print those variants.
    #[test]
    fn the_nested_keys_resolve_through_the_dialect() {
        harness().run(|jail| {
            jail.config(
                "[telemetry]\njson_logs = true\n\n[telemetry.sentry]\nenabled = true\n\
                 capture_level = \"warn\"\ntraces_sample_rate = 0.25\n",
            )?;
            jail.env_key("telemetry.sentry.max_breadcrumbs", 30);

            let config: Sample = jail.load()?;
            let sentry = &config.telemetry.sentry;

            assert!(config.telemetry.json_logs);
            assert!(sentry.enabled);
            assert_eq!(sentry.capture_level, SentryLevel::Warn);
            assert!((sentry.traces_sample_rate - 0.25).abs() < f32::EPSILON);
            assert_eq!(sentry.max_breadcrumbs, 30);
            // A partial table leaves the rest of the block on its defaults.
            assert!(sentry.attach_stacktraces);
            assert!(sentry.http_transactions);
            Ok(())
        });
    }

    /// The DSN is the first secret this configuration has ever carried, so this is also the
    /// first test that the file-backed layers do something a deployment depends on: a mounted
    /// `Secret` supplies it, and the trailing newline a `kubectl create secret` leaves behind is
    /// not part of the value.
    #[test]
    fn the_dsn_is_read_from_a_mounted_secret() {
        harness().run(|jail| {
            jail.secret_key("telemetry.sentry.dsn", "https://key@sentry.example/42\n")?;

            let config: Sample = jail.load()?;

            assert_eq!(
                config
                    .telemetry
                    .sentry
                    .dsn
                    .as_ref()
                    .expect("the mounted DSN is read")
                    .expose_secret(),
                "https://key@sentry.example/42"
            );
            Ok(())
        });
    }

    /// A DSN in the environment and a DSN on a volume is the half-migrated deployment the
    /// shadow-key rule exists for, and it is worth pinning on the one key here that is a
    /// credential: the two values are indistinguishable in a log, and only the layer says which
    /// project the events are actually landing in.
    #[test]
    fn a_dsn_supplied_twice_is_refused() {
        harness().run(|jail| {
            jail.secret_key("telemetry.sentry.dsn", "https://key@sentry.example/42\n")?;
            jail.env_key("telemetry.sentry.dsn", "https://other@sentry.example/7");

            let error = jail
                .load::<Sample>()
                .expect_err("a doubly-supplied DSN must fail the load");

            assert!(
                error.to_string().contains("dsn"),
                "the error must name the key: {error}"
            );
            Ok(())
        });
    }
}
