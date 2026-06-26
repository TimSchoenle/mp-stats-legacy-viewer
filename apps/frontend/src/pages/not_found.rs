use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-[calc(100vh-8rem)] p-6">
            <div class="card p-12 text-center max-w-md w-full">
                <div class="eyebrow mb-6">{"Error · page not found"}</div>
                <h1 class="serif page-title text-7xl mb-6 text-paper-1">{ "404" }</h1>
                <p class="text-sm text-paper-3 mb-8 leading-relaxed">
                    { "The page you're looking for isn't in the archive." }
                </p>
                <Link<Route>
                    to={Route::Home}
                    classes="btn"
                >
                    { "← Return home" }
                </Link<Route>>
            </div>
        </div>
    }
}
