use axum::Router;
use clap::Parser;
use std::path::PathBuf;
use std::net::SocketAddr;
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
    let spa_service = ServeDir::new(&dist_dir)
        .not_found_service(ServeFile::new(dist_dir.join("index.html")));

    let app = Router::new()
        .nest_service("/data", ServeDir::new(opt.data_dir))
        .fallback_service(spa_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}