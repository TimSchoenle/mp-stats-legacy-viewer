//! Togglable Sentry error reporting and performance tracing.
//!
//! Two switches, and both have to be on. The `sentry` cargo feature decides whether the SDK is
//! *linked* — it is a default feature, and turning it off is worth roughly two megabytes of
//! transport and TLS in a `scratch` image that otherwise carries one static binary. The
//! `telemetry.sentry.enabled` key decides whether a client is *installed*, and it is off by
//! default because a DSN is an egress destination for whatever a log line happens to carry.
//!
//! A binary built without the feature and configured with the key set refuses to boot. It is the
//! same refusal as `enabled` without a DSN, for the same reason: a reporter that reports nowhere
//! is indistinguishable from a service with nothing to report, and the difference is only
//! noticed during the incident it was meant to surface.
//!
//! Three sinks, all fed by the one client [`init`] installs:
//!
//! - **`tracing`** — [`tracing_layer`] turns records into issues and breadcrumbs, under the
//!   thresholds in `[telemetry.sentry]`.
//! - **panics** — the SDK's own hook, added by `sentry::init`. Worth having precisely because
//!   `axum` absorbs a panicking handler by dropping the connection, which leaves nothing behind
//!   in an image nobody is attached to.
//! - **HTTP** — [`attach`], which wraps the router in a per-request hub and, optionally, one
//!   transaction per matched route.
//!
//! The extern crate is always spelled `::sentry`; the bare path is ambiguous with this module.

#[cfg(feature = "sentry")]
mod enabled {
    use std::sync::OnceLock;
    use std::time::Duration;

    use ::sentry::integrations::tracing::{EventFilter, SentryLayer, default_span_filter};
    use anyhow::{Result, bail};
    use axum::Router;
    use mp_stats_config::{SentryConfig, SentryLevel};
    use secrecy::ExposeSecret as _;
    use tracing::Level;
    use tracing_subscriber::registry::LookupSpan;

    /// The client guard the process holds for its lifetime.
    pub(crate) type Guard = ::sentry::ClientInitGuard;

    /// What [`attach`] mounts, decided once at boot.
    ///
    /// Process-global because the client it describes is: `sentry::init` binds one client to
    /// `Hub::main()` for the lifetime of the process, so a copy threaded through the router
    /// builder would be a second source of truth for a single global. Unset until [`init`] runs,
    /// which `main` does before it builds a router.
    static HTTP: OnceLock<HttpOptions> = OnceLock::new();

    /// The two independent halves of the HTTP integration.
    #[derive(Debug, Clone, Copy)]
    struct HttpOptions {
        /// A client is bound, so requests get their own hub and their request metadata.
        active: bool,
        /// Additionally start one transaction per request. Whether that transaction is *kept* is
        /// the sampler's decision, not this one.
        transactions: bool,
    }

    /// Install the process-wide client, or nothing when the section is switched off.
    ///
    /// # Errors
    ///
    /// When `enabled` is set without a DSN, when the DSN does not parse, or when a sample rate
    /// falls outside `0.0..=1.0`. All three are configuration mistakes whose only other outcome
    /// is a server that reports nothing and says so nowhere.
    pub(crate) fn init(config: &SentryConfig) -> Result<Option<Guard>> {
        if !config.enabled {
            record_http(HttpOptions {
                active: false,
                transactions: false,
            });
            return Ok(None);
        }

        // Empty is absent, not a value. `MP_STATS_TELEMETRY__SENTRY__DSN=""` is what an
        // unfilled chart value or a compose pass-through produces, and it has to land on the
        // message below rather than on the parse error, which would send an operator looking at
        // a URL that is not the problem.
        let dsn = config
            .dsn
            .as_ref()
            .map(|dsn| dsn.expose_secret().trim())
            .filter(|dsn| !dsn.is_empty());
        let Some(dsn) = dsn else {
            bail!(
                "`telemetry.sentry.enabled` is set but `telemetry.sentry.dsn` is empty, so \
                 nothing would be reported. Set the DSN, or turn the section off."
            );
        };

        // Parsed here rather than through `ClientOptions::dsn`, which panics on a malformed
        // value. The error deliberately does not quote the DSN: it embeds a credential, and this
        // message reaches the log stream.
        let dsn = dsn.parse::<::sentry::types::Dsn>().map_err(|error| {
            anyhow::anyhow!(
                "`telemetry.sentry.dsn` is not a valid Sentry DSN ({error}); expected \
                 https://<key>@<host>/<project>"
            )
        })?;

        check_rate("sample_rate", config.sample_rate)?;
        check_rate("traces_sample_rate", config.traces_sample_rate)?;

        // The transport is taken as `rustls-no-provider` so that the choice of crypto backend is
        // this repository's rather than a transitive default - see the workspace manifest - and
        // the cost of that is having to make it here. Without an installed provider, reqwest
        // panics when it builds its connector, which would turn a misconfigured DSN host into a
        // crash rather than a queued event that never sends.
        //
        // The result is ignored deliberately: `Err` means a provider is already installed, which
        // is a second `init` in one process, and the first one's choice is the one in force.
        let _ = rustls::crypto::ring::default_provider().install_default();

        let environment = config.environment.clone().unwrap_or_else(|| {
            // Read off the build rather than off a profile variable this platform does not have.
            // Every published image is a release build, so the common case is right without an
            // operator setting anything; a staging cluster running the same image is the case
            // that has to say so explicitly.
            if cfg!(debug_assertions) {
                "development".to_owned()
            } else {
                "production".to_owned()
            }
        });

        // Spelled as the image tag spells it, `v` included, so a regression in Sentry names the
        // same string as the release it came from.
        let release = config.release.clone().unwrap_or_else(|| {
            concat!("mp-stats-legacy-viewer@v", env!("CARGO_PKG_VERSION")).to_owned()
        });

        let mut options = ::sentry::ClientOptions::new()
            .debug(config.debug)
            .sample_rate(config.sample_rate)
            .traces_sample_rate(config.traces_sample_rate)
            .max_breadcrumbs(config.max_breadcrumbs)
            .attach_stacktrace(config.attach_stacktraces)
            .send_default_pii(config.send_default_pii)
            .shutdown_timeout(Duration::from_secs(config.shutdown_timeout_secs))
            .environment(environment)
            .release(release)
            // Marks this workspace's own frames as application code, so a stack trace opens on
            // the handler rather than on an axum internal.
            .in_app_include(vec!["mp_stats"]);
        options.dsn = Some(dsn);
        if let Some(server_name) = config.server_name.clone() {
            options = options.server_name(server_name);
        }

        // Every field `apply_defaults` would otherwise fill from `SENTRY_DSN`, `SENTRY_RELEASE`
        // or `SENTRY_ENVIRONMENT` is set above, and that is the point: those variables are a
        // second configuration channel bypassing the layered loader and its shadow-key
        // rejection, and an already-set field is one they cannot reach.
        let guard = ::sentry::init(options);

        record_http(HttpOptions {
            active: true,
            transactions: config.http_transactions,
        });

        Ok(Some(guard))
    }

    /// The `tracing` layer feeding the client, or `None` when the section is off.
    pub(crate) fn tracing_layer<S>(config: &SentryConfig) -> Option<SentryLayer<S>>
    where
        S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    {
        if !config.enabled {
            return None;
        }

        let capture = config.capture_level;
        let breadcrumb = config.breadcrumb_level;

        let mut layer = ::sentry::integrations::tracing::layer()
            .event_filter(move |metadata| {
                let level = *metadata.level();
                if accepts(capture, level) {
                    EventFilter::Event
                } else if accepts(breadcrumb, level) {
                    EventFilter::Breadcrumb
                } else {
                    EventFilter::Ignore
                }
            })
            // Not additionally gated on `traces_sample_rate`. Whether a span is *recorded* is
            // the sampler's decision, and it is the one that can honour an inherited trace: a
            // process at rate `0.0` starts none of its own but still continues one it was
            // handed. Gating span creation here would cut that trace at this server.
            .span_filter(default_span_filter);

        if config.span_attributes {
            layer = layer.enable_span_attributes();
        }

        Some(layer)
    }

    /// Wrap `router` in the per-request hub and the request-metadata layer.
    ///
    /// The hub layer is not optional decoration: without a hub per request, breadcrumbs from
    /// concurrently served requests all land on the main hub, and every issue arrives carrying a
    /// trail that belongs to whoever else was in flight.
    pub(crate) fn attach(router: Router) -> Router {
        let Some(options) = HTTP.get().copied().filter(|options| options.active) else {
            return router;
        };

        // `SentryHttpLayer::new` reads `send_default_pii` off the bound client to decide whether
        // to redact sensitive request headers, so it has to be built after `init`.
        let http = ::sentry::integrations::tower::SentryHttpLayer::new();
        let http = if options.transactions {
            http.enable_transaction()
        } else {
            http
        };

        // The hub layer is the outer of the two: the metadata layer writes onto the hub, so the
        // hub has to be bound before it runs.
        router
            .layer(http)
            .layer(::sentry::integrations::tower::NewSentryLayer::<
                axum::extract::Request,
            >::new_from_top())
    }

    /// Whether a record at `level` is at least as severe as `threshold`.
    ///
    /// `tracing::Level` orders `ERROR` lowest, so "at least as severe" is `<=`.
    fn accepts(threshold: SentryLevel, level: Level) -> bool {
        let threshold = match threshold {
            SentryLevel::Off => return false,
            SentryLevel::Error => Level::ERROR,
            SentryLevel::Warn => Level::WARN,
            SentryLevel::Info => Level::INFO,
            SentryLevel::Debug => Level::DEBUG,
            SentryLevel::Trace => Level::TRACE,
        };
        level <= threshold
    }

    /// Refuse a rate the SDK would otherwise clamp or read as "send nothing".
    fn check_rate(name: &str, rate: f32) -> Result<()> {
        if (0.0..=1.0).contains(&rate) {
            Ok(())
        } else {
            bail!("`telemetry.sentry.{name}` must be between 0.0 and 1.0, got {rate}")
        }
    }

    /// First writer wins, matching the client itself: a second `init` in one process is a test
    /// harness, not a reconfiguration.
    fn record_http(options: HttpOptions) {
        let _ = HTTP.set(options);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// `tracing::Level` sorts `ERROR` *below* `TRACE`, so a severity threshold reads as `<=`
        /// and not `>=`. Inverting it turns `capture_level = "error"` into "capture everything",
        /// which arrives as a bill rather than as a compile error.
        #[test]
        fn a_threshold_accepts_only_levels_at_least_as_severe() {
            assert!(accepts(SentryLevel::Error, Level::ERROR));
            assert!(!accepts(SentryLevel::Error, Level::WARN));
            assert!(!accepts(SentryLevel::Error, Level::TRACE));

            assert!(accepts(SentryLevel::Info, Level::ERROR));
            assert!(accepts(SentryLevel::Info, Level::WARN));
            assert!(accepts(SentryLevel::Info, Level::INFO));
            assert!(!accepts(SentryLevel::Info, Level::DEBUG));

            for level in [
                Level::ERROR,
                Level::WARN,
                Level::INFO,
                Level::DEBUG,
                Level::TRACE,
            ] {
                assert!(!accepts(SentryLevel::Off, level));
                assert!(accepts(SentryLevel::Trace, level));
            }
        }

        #[test]
        fn a_sample_rate_outside_the_unit_interval_is_refused() {
            assert!(check_rate("sample_rate", 0.0).is_ok());
            assert!(check_rate("sample_rate", 1.0).is_ok());
            assert!(check_rate("sample_rate", -0.1).is_err());
            assert!(check_rate("sample_rate", 1.1).is_err());
        }

        /// The disabled path installs no client at all — not a client with an empty DSN, which
        /// still starts a transport thread and still queues events nobody collects.
        #[test]
        fn the_default_section_installs_no_layer() {
            let config = SentryConfig::default();

            assert!(!config.enabled);
            assert!(tracing_layer::<tracing_subscriber::Registry>(&config).is_none());
        }

        /// `enabled` without a DSN is the mistake this whole module is arranged around: the
        /// alternative outcome is a server that starts, serves, and reports nowhere.
        #[test]
        fn enabled_without_a_dsn_is_a_boot_failure() {
            let config = SentryConfig {
                enabled: true,
                ..SentryConfig::default()
            };

            let Err(error) = init(&config) else {
                panic!("a client with no DSN reports nowhere and must not be installed")
            };
            assert!(error.to_string().contains("dsn"), "{error}");
        }

        /// A pass-through that resolved to nothing — an unfilled chart value,
        /// `MP_STATS_TELEMETRY__SENTRY__DSN=""` — has to read as *absent* rather than as a DSN
        /// that fails to parse. The two produce very different messages, and only one of them
        /// sends the operator to the right place.
        #[test]
        fn an_empty_dsn_reads_as_absent_rather_than_malformed() {
            let config = SentryConfig {
                enabled: true,
                dsn: Some("   ".into()),
                ..SentryConfig::default()
            };

            let Err(error) = init(&config) else {
                panic!("a blank DSN reports nowhere either")
            };
            assert!(error.to_string().contains("is empty"), "{error}");
        }

        /// The DSN never reaches the log stream, not even when it is the thing that is wrong.
        #[test]
        fn a_malformed_dsn_is_not_quoted_back() {
            let config = SentryConfig {
                enabled: true,
                dsn: Some("https://the-key@not a dsn".into()),
                ..SentryConfig::default()
            };

            let Err(error) = init(&config) else {
                panic!("a malformed DSN cannot be used")
            };
            assert!(
                !error.to_string().contains("the-key"),
                "the credential must not be echoed: {error}"
            );
        }
    }
}

#[cfg(not(feature = "sentry"))]
mod disabled {
    use anyhow::{Result, bail};
    use axum::Router;
    use mp_stats_config::SentryConfig;
    use tracing_subscriber::layer::Identity;

    /// The client guard, in a build that cannot have one.
    ///
    /// Uninhabited on purpose: [`init`] below returns `Ok(None)` or an error and there is no
    /// third case, so "this build reports nothing" is checked by the compiler rather than
    /// asserted in a comment.
    pub(crate) enum Guard {}

    /// Refuse a configuration this binary cannot honour, and otherwise do nothing.
    ///
    /// # Errors
    ///
    /// When `telemetry.sentry.enabled` is set. Ignoring it would leave an operator with a
    /// deployment that looks configured, a Sentry project that stays empty, and no line anywhere
    /// connecting the two.
    pub(crate) fn init(config: &SentryConfig) -> Result<Option<Guard>> {
        if config.enabled {
            bail!(
                "`telemetry.sentry.enabled` is set, but this binary was built without its \
                 `sentry` feature and can report nothing. Rebuild `mp-stats-server` with default \
                 features, or turn the section off."
            );
        }
        Ok(None)
    }

    /// No layer, spelled as the no-op `Layer` so the subscriber build stays one expression.
    ///
    /// Deliberately not generic over the subscriber the way its counterpart is: `Identity` is a
    /// `Layer<S>` for every `S`, so a parameter here would appear in no argument and no return
    /// type, and every call site would have to name a type the caller does not otherwise know.
    pub(crate) fn tracing_layer(_config: &SentryConfig) -> Option<Identity> {
        None
    }

    /// The router, unchanged.
    pub(crate) fn attach(router: Router) -> Router {
        router
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// The whole point of compiling this module rather than deleting the keys: a deployment
        /// that has switched Sentry on is told that this build cannot honour it, instead of
        /// running for a quarter with an empty Sentry project and no explanation.
        #[test]
        fn an_enabled_section_is_refused_by_a_build_that_cannot_honour_it() {
            let config = SentryConfig {
                enabled: true,
                dsn: Some("https://key@sentry.example/42".into()),
                ..SentryConfig::default()
            };

            let Err(error) = init(&config) else {
                panic!("a build without the feature reports nothing and must say so")
            };
            assert!(
                error.to_string().contains("sentry"),
                "the error must name the feature: {error}"
            );
        }

        /// And the default section still boots, because that is every deployment that has never
        /// heard of this block.
        #[test]
        fn the_default_section_boots() {
            assert!(init(&SentryConfig::default()).is_ok());
        }
    }
}

// One of the two module bodies above is compiled, and its three items are this module's. The
// alternative — a wrapper per item, dispatching on the same `cfg` — would put the feature
// condition in six places rather than two, and would need an `impl Trait` return whose concrete
// type differs between the builds.
#[cfg(feature = "sentry")]
use enabled as backend;

#[cfg(not(feature = "sentry"))]
use disabled as backend;

pub(crate) use backend::{Guard, attach, init, tracing_layer};
