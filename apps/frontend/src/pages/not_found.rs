use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-[calc(100vh-8rem)] text-white p-6">
            <div class="glass-panel p-12 text-center max-w-md w-full border border-white/10 shadow-2xl relative overflow-hidden">
                <div class="absolute inset-0 bg-red-500/5 pointer-events-none"></div>
                <h1 class="text-8xl font-bold mb-4 font-mono text-white tracking-tighter drop-shadow-sm">{ "404" }</h1>
                <p class="text-xl text-gray-400 mb-8">{ "The page you're looking for doesn't exist." }</p>
                <Link<Route>
                    to={Route::Home}
                    classes="inline-flex items-center justify-center px-6 py-3 rounded-lg bg-white/10 hover:bg-white/20 text-white font-medium transition-colors border border-white/5"
                >
                    { "Return Home" }
                </Link<Route>>
            </div>
        </div>
    }
}
