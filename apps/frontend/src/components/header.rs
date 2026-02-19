use crate::Route;
use crate::components::search_bar::SearchBar;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="sticky top-0 z-50 w-full border-b border-white/5 bg-dark-950/80 backdrop-blur-md transition-all">
            <div class="container mx-auto px-4 h-16 flex items-center justify-between">
                // Logo / Brand
                <div class="flex items-center gap-6">
                    <Link<Route> to={Route::Home} classes="flex items-center gap-2 group">
                        <span class="text-xl font-bold tracking-tight text-white group-hover:text-emerald-400 transition-colors">
                            {"MP Stats Legacy Viewer"}
                        </span>
                    </Link<Route>>

                    // Navigation Links (Desktop)
                    <nav class="hidden md:flex items-center gap-1">
                        <Link<Route> to={Route::Home} classes="nav-link">{"Home"}</Link<Route>>
                        <Link<Route> to={Route::Landing { edition: PlatformEdition::Java }} classes="nav-link">{"Java"}</Link<Route>>
                        <Link<Route> to={Route::Landing { edition: PlatformEdition::Bedrock }} classes="nav-link">{"Bedrock"}</Link<Route>>
                    </nav>
                </div>

                // Search Bar
                <div class="flex-1 max-w-sm ml-4">
                    <SearchBar />
                </div>
            </div>
        </header>
    }
}
