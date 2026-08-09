mod config;

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
    let config: Config = mp_stats_config::load().context("loading configuration")?;
    let server = config.server;

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

    let listener = tokio::net::TcpListener::bind(server.bind_addr)
        .await
        .with_context(|| format!("binding {}", server.bind_addr))?;
    println!("Listening on http://{}", server.bind_addr);

    axum::serve(listener, router(&server, &index_path))
        .await
        .context("serving")
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

async fn startup_probe() -> StatusCode {
    StatusCode::OK
}

async fn liveness_probe() -> StatusCode {
    StatusCode::OK
}

async fn readiness_probe() -> StatusCode {
    StatusCode::OK
}
