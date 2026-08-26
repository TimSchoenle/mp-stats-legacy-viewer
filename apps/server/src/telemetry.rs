//! The global `tracing` subscriber, and the guard that keeps the reporter alive behind it.
//!
//! This is the only place the process decides how it writes a record. Everything else in the
//! binary emits through `tracing` macros, which is what lets one configuration key redirect the
//! whole stream to JSON, and lets a second sink — [`crate::sentry`] — read the same records
//! without any call site knowing it exists.

use anyhow::{Context, Result};
use mp_stats_config::TelemetryConfig;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

/// Keeps the Sentry client alive, and flushes what it has queued when it is dropped.
///
/// Returned by [`init`] rather than parked in a static, because a static is never dropped: the
/// flush that gets the last events of a terminating process out is this drop, bounded by
/// `telemetry.sentry.shutdown_timeout_secs`. Bind it for the lifetime of the run — `let
/// _telemetry = init(…)?` — since `let _ = init(…)?` drops it immediately and closes the client
/// before the server has served anything.
///
/// Always `None` when the binary was built without the `sentry` feature: [`crate::sentry::Guard`]
/// is uninhabited there, so "no client" is a fact of the type rather than of this value.
#[must_use = "dropping the guard closes the Sentry client and stops reporting"]
pub(crate) struct TelemetryGuard(Option<crate::sentry::Guard>);

// Hand-written because `sentry::ClientInitGuard` is not `Debug`, and reporting only whether a
// client is bound is what anyone would want from it anyway: the client itself holds the DSN.
impl std::fmt::Debug for TelemetryGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TelemetryGuard")
            .field(&self.reporting())
            .finish()
    }
}

impl TelemetryGuard {
    /// Whether a Sentry client is bound to this process.
    pub(crate) fn reporting(&self) -> bool {
        self.0.is_some()
    }
}

/// Install the global subscriber, and the Sentry client when one is configured.
///
/// Emits one JSON object per record when [`TelemetryConfig::json_logs`] is set and
/// human-readable lines otherwise. The filter is [`TelemetryConfig::log_filter`], overridden by
/// `RUST_LOG` when that is set to something parseable — and it governs the Sentry sink too, so
/// tightening the filter to `warn` silently removes every `info` breadcrumb.
///
/// # Errors
///
/// If the filter does not parse, if a subscriber is already installed, or if
/// `[telemetry.sentry]` is switched on and unusable — see [`crate::sentry::init`].
pub(crate) fn init(config: &TelemetryConfig) -> Result<TelemetryGuard> {
    let filter = match EnvFilter::try_from_default_env() {
        Ok(filter) => filter,
        // Not a fallback for *every* error: `try_from_default_env` reports an unset `RUST_LOG`
        // and an unparseable one the same way, and silently ignoring the second means an
        // operator debugging a container gets the configured filter back with no hint that the
        // directive they typed was rejected.
        Err(error) if std::env::var_os("RUST_LOG").is_some() => {
            return Err(anyhow::Error::new(error).context(
                "RUST_LOG is set but is not a valid filter; unset it to fall back to \
                 `telemetry.log_filter`",
            ));
        }
        Err(_) => EnvFilter::try_new(&config.log_filter).with_context(|| {
            format!(
                "`telemetry.log_filter` is not a valid filter: {}",
                config.log_filter
            )
        })?,
    };

    // Before the subscriber is installed, not after: the layer below reports onto the client
    // this binds, and the SDK's panic hook should already be in place for anything the
    // subscriber build itself does.
    let guard = crate::sentry::init(&config.sentry)?;

    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(crate::sentry::tracing_layer(&config.sentry));

    if config.json_logs {
        registry
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_current_span(true)
                    .with_span_list(false),
            )
            .try_init()
    } else {
        registry
            .with(tracing_subscriber::fmt::layer().with_target(true))
            .try_init()
    }
    .map_err(|error| anyhow::anyhow!("{error}"))
    .context("installing the global tracing subscriber")?;

    Ok(TelemetryGuard(guard))
}
