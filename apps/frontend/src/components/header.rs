use crate::components::search_bar::SearchBar;
use crate::hooks::use_theme;
use crate::Route;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let route = use_route::<Route>().unwrap_or(Route::Home);
    let theme_color = use_theme();

    html! {
        <header class="sticky top-0 z-50 w-full border-b border-white/10 bg-dark-900/80 backdrop-blur-md transition-all shadow-sm">
            <div class="container mx-auto px-4 h-16 flex items-center justify-between">
                // Logo / Brand
                <div class="flex items-center gap-6">
                    <Link<Route> to={Route::Home} classes="flex items-center gap-2 group">
                        <span class={classes!(theme_color, "text-xl", "font-bold", "tracking-tight", "text-white", "group-hover:text-theme-400", "transition-colors")}>
                            {"MP Stats Legacy Viewer"}
                        </span>
                    </Link<Route>>

                    // Navigation Links (Desktop)
                    <nav class="hidden md:flex items-center gap-2">
        {for PlatformEdition::iter().map(|edition| {
                let is_active = matches!(
                    &route,
                    Route::Landing { edition: current_edition } if current_edition == edition
                );
            html! {
                <Link<Route> to={Route::Landing { edition: edition.clone() }} classes={classes!("nav-link", if is_active { "active" } else { "" })}>{edition.display_name()}</Link<Route>>
            }
        })}
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
