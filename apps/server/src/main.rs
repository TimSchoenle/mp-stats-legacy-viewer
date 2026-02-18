use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get};
use clap::Parser;
use mp_stats_core::DataProviderWrapper;
use mp_stats_data_server::ServerDataProvider;
use mp_stats_frontend::app::App;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;
use yew::ServerRenderer;
use yew::prelude::*;
use yew_router::Router as YewRouter;
use yew_router::history::History;
use yew_router::history::{AnyHistory, MemoryHistory};
use yew_router::prelude::*;

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
        .thread_stack_size(8 * 1024 * 1024) // 8 MB — musl defaults to 128KB which causes segfaults
        .build()
        .expect("Failed to build Tokio runtime");

    runtime.block_on(async_main());
}

async fn async_main() {
    let opt = Opt::parse();
    let data_dir = opt.data_dir.clone();

    let template = std::fs::read_to_string(opt.dir.join("index.html"))
        .expect("Failed to read dist/index.html — run `trunk build` first");

    let state = AppState {
        data_dir,
        dist_dir: opt.dir.clone(),
        template,
    };

    let app = axum::Router::new()
        .route("/", get(render_app))
        .route("/java/*path", get(render_app))
        .route("/bedrock/*path", get(render_app))
        .nest_service("/data", ServeDir::new(&opt.data_dir))
        .fallback_service(ServeDir::new(&opt.dir).append_index_html_on_directories(false))
        .with_state(state);

    println!("Listening on http://localhost:8080");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    data_dir: PathBuf,
    dist_dir: PathBuf,
    template: String,
}

#[derive(Properties, PartialEq, Clone)]
struct ServerAppProps {
    pub provider: DataProviderWrapper,
    pub history: AnyHistory,
}

#[function_component(ServerApp)]
fn server_app(props: &ServerAppProps) -> Html {
    html! {
        <ContextProvider<DataProviderWrapper> context={props.provider.clone()}>
            <YewRouter history={props.history.clone()}>
                <App />
            </YewRouter>
        </ContextProvider<DataProviderWrapper>>
    }
}

async fn render_app(
    State(state): State<AppState>,
    url: axum::extract::OriginalUri,
) -> impl IntoResponse {
    let provider = DataProviderWrapper(Arc::new(ServerDataProvider::new(state.data_dir)));
    let url = url.to_string();

    let renderer = ServerRenderer::<ServerApp>::with_props(move || {
        let history = AnyHistory::from(MemoryHistory::new());
        history.push(&url);

        ServerAppProps { provider, history }
    });

    let html = renderer.render().await;

    let full_html = state
        .template
        .replace("<body></body>", &format!("<body>{}</body>", html));

    (StatusCode::OK, axum::response::Html(full_html))
}
