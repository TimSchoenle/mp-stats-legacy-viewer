//! A static file server with a `Content-Security-Policy` on it.
//!
//! There is no API. The converted tree is mounted at `/data` and the built frontend everywhere
//! else, and the client does its own querying by fetching files out of the tree, so nothing here
//! knows what a leaderboard is. The three health probes are the only routes this binary registers,
//! and everything else is [`ServeDir`].
//!
//! Two things are checked before the listener binds, because both fail as a working server serving
//! a broken site: the shell has to exist, and its inline scripts have to be hashable into the
//! policy. See [`csp`].
//!
//! Everything the process says about itself goes through `tracing` — see [`telemetry`] — and
//! optionally to Sentry as well, which is the same stream read a second time. See [`sentry`].

mod config;
mod csp;
mod sentry;
mod telemetry;

use crate::config::Config;
use anyhow::{Context, Result};
use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;
use mp_stats_config::ServerConfig;
use std::path::Path;
use tower_http::services::{ServeDir, ServeFile};

fn main() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(8 * 1024 * 1024)
        .build()
        .context("building the Tokio runtime")?;

    runtime.block_on(serve())
}

async fn serve() -> Result<()> {
    // Layered: struct defaults, then `$MP_STATS_CONFIG`, then `MP_STATS_*`. See
    // `docs/CONFIGURATION.md`.
    //
    // Before the subscriber exists, so a failure here is reported by `main`'s `Result` and not by
    // a logger this line is what configures. It is the one thing this binary can be wrong about
    // that no log line will ever describe.
    let config: Config = mp_stats_config::load().context("loading configuration")?;
    let Config {
        server,
        telemetry: observability,
    } = config;

    // Held for the rest of `main`: dropping it closes the Sentry client, so it is what turns the
    // graceful shutdown below into a flush of whatever is still queued.
    let telemetry = telemetry::init(&observability).context("installing telemetry")?;
    if telemetry.reporting() {
        // After the subscriber is installed, not beside the client: a record emitted before it
        // exists goes nowhere, and "is Sentry actually on in this container" is the first
        // question an operator asks.
        tracing::info!(
            traces_sample_rate = observability.sentry.traces_sample_rate,
            send_default_pii = observability.sentry.send_default_pii,
            "Sentry reporting enabled"
        );
    }

    // Checked before anything binds: a dist directory without an entry point answers every
    // route with a 404 that looks like a routing bug, so refusing to start names the cause
    // once instead.
    let index_path = server.index_path();
    std::fs::metadata(&index_path).with_context(|| {
        format!(
            "no index.html at {} - `server.dist_dir` must point at a built frontend",
            index_path.display()
        )
    })?;

    // Before the bind, because it reads that same index.html: a shell whose inline scripts
    // cannot be hashed would otherwise take the port and then serve a blank page.
    let router = csp::attach(router(&server, &index_path), &server.csp, &index_path)
        .context("assembling the Content-Security-Policy")?;

    // Outside the policy middleware, so a request that panics inside it is still reported and
    // still carries its own hub. A no-op unless a client was bound above.
    let router = sentry::attach(router);

    let listener = tokio::net::TcpListener::bind(server.bind_addr)
        .await
        .with_context(|| format!("binding {}", server.bind_addr))?;
    tracing::info!(address = %server.bind_addr, "Listening on http://{}", server.bind_addr);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("serving")?;

    // Reached only through the signal handler, which is the whole reason it exists: `axum::serve`
    // without it runs until the process is killed, and a killed process drops nothing — so the
    // events queued by whatever went wrong last are lost exactly when they matter.
    tracing::info!("Shutting down");

    // Explicit rather than left to the end of the scope: this drop is the flush, and a later
    // edit that adds a line after it would silently move the flush past it.
    drop(telemetry);
    Ok(())
}

/// Resolves on the first termination signal the platform has.
///
/// `SIGTERM` is what an orchestrator sends before it escalates, and Ctrl-C is what a developer
/// sends. Both mean the same thing here: stop accepting, finish what is in flight, and let the
/// telemetry guard flush on the way out.
async fn shutdown_signal() {
    let interrupt = async {
        tokio::signal::ctrl_c()
            .await
            .expect("the Ctrl-C handler installs on every platform this runs on");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("SIGTERM is a valid signal to listen for")
            .recv()
            .await;
    };

    // Windows has no `SIGTERM`, and this binary is developed there as well as deployed on Linux.
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = interrupt => {}
        () = terminate => {}
    }
}

/// Health probes, the converter's output under `/data`, and the SPA everywhere else.
fn router(config: &ServerConfig, index_path: &Path) -> Router {
    let spa_service = ServeDir::new(&config.dist_dir).not_found_service(ServeFile::new(index_path));

    Router::new()
        .route("/health/startup", get(startup_probe))
        .route("/health/live", get(liveness_probe))
        .route("/health/ready", get(readiness_probe))
        .nest_service("/data", ServeDir::new(&config.data_dir))
        .fallback_service(spa_service)
}

// All three answer unconditionally. A process that has bound the port has already proved the two
// things this server can be wrong about — the shell is readable and the policy renders — and it
// holds no connection, cache or upstream whose health could change afterwards.
async fn startup_probe() -> StatusCode {
    StatusCode::OK
}

async fn liveness_probe() -> StatusCode {
    StatusCode::OK
}

async fn readiness_probe() -> StatusCode {
    StatusCode::OK
}
