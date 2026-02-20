use mp_stats_frontend::Api;
use mp_stats_frontend::app::App;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Root)]
fn root() -> Html {
    let api_context = Api;

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
