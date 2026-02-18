use crate::Route;
use crate::components::search_bar::SearchBar;
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
                        <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-emerald-500 to-teal-600 shadow-lg shadow-emerald-500/20 flex items-center justify-center text-white font-bold text-lg group-hover:scale-105 transition-transform">
                            {"M"}
                        </div>
                        <span class="text-xl font-bold tracking-tight text-white group-hover:text-emerald-400 transition-colors">
                            {"Legacy Viewer"}
                        </span>
                    </Link<Route>>

                    // Navigation Links (Desktop)
                    <nav class="hidden md:flex items-center gap-1">
                        <Link<Route> to={Route::Home} classes="nav-link">{"Home"}</Link<Route>>
                        <Link<Route> to={Route::JavaLanding} classes="nav-link">{"Java"}</Link<Route>>
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
