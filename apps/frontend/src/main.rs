use mp_stats_core::DataProviderWrapper;
use mp_stats_data_client::ClientDataProvider;
use mp_stats_frontend::app::App;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Root)]
fn root() -> Html {
    let provider = DataProviderWrapper(Rc::new(ClientDataProvider));

    html! {
        <ContextProvider<DataProviderWrapper> context={provider}>
            <BrowserRouter>
                <App />
            </BrowserRouter>
        </ContextProvider<DataProviderWrapper>>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
