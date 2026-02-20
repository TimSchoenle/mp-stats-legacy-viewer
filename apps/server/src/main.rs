use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Parser, Debug)]
struct Opt {
    /// Directory where static assets (dist) are located
    #[clap(short, long, default_value = "dist")]
    dir: PathBuf,
    /// Directory where data is located
    #[clap(long, default_value = "data")]
    data_dir: PathBuf,
}

struct AppState {
    index_path: PathBuf,
}

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(8 * 1024 * 1024)
        .build()
        .expect("Failed to build Tokio runtime");

    runtime.block_on(async_main());
}

async fn async_main() {
    let opt = Opt::parse();

    let dist_dir = opt.dir.clone();

    let index_path = dist_dir.join("index.html");
    std::fs::metadata(&index_path).unwrap_or_else(|_| {
        panic!(
            "CRITICAL ERROR: Failed to start server! index.html was not found in {:?}",
            dist_dir
        )
    });

    let spa_service = ServeDir::new(&dist_dir).not_found_service(ServeFile::new(&index_path));

    let state = Arc::new(AppState { index_path });

    let app = Router::new()
        .route("/health/startup", get(startup_probe))
        .route("/health/live", get(liveness_probe))
        .route("/health/ready", get(readiness_probe))
        .nest_service("/data", ServeDir::new(opt.data_dir))
        .fallback_service(spa_service)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn startup_probe() -> StatusCode {
    StatusCode::OK
}

async fn liveness_probe(State(state): State<Arc<AppState>>) -> StatusCode {
    match tokio::fs::metadata(&state.index_path).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            eprintln!(
                "Liveness probe failed to read {:?}: {}",
                state.index_path, e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn readiness_probe(State(state): State<Arc<AppState>>) -> StatusCode {
    liveness_probe(State(state)).await
}
