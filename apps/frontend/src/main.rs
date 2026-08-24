//! Mounts the client. Everything it mounts is [`mp_stats_frontend`].

use mp_stats_frontend::Api;
use mp_stats_frontend::app::App;
use yew::prelude::*;
use yew_router::prelude::*;

/// The one place the [`Api`] context and the router are created, so every component below shares
/// a single fetch cache for the lifetime of the tab.
#[function_component(Root)]
fn root() -> Html {
    let api_context = Api::default();

    html! {
        <ContextProvider<Api> context={api_context}>
            <BrowserRouter>
                <App />
            </BrowserRouter>
        </ContextProvider<Api>>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
