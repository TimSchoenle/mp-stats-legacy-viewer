use crate::Route;
use crate::components::search_bar::SearchBar;
use crate::hooks::use_theme;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let route = use_route::<Route>().unwrap_or(Route::Home);
    let theme_color = use_theme();

    html! {
        <header class={classes!(theme_color, "sticky", "top-0", "z-50", "w-full", "border-b", "border-rule", "bg-ink-0/90", "backdrop-blur-md")}>
            <div class="container mx-auto px-6 h-14 flex items-center gap-8">
                // Brand
                <Link<Route> to={Route::Home} classes="flex items-baseline gap-2.5 group">
                    <span class="serif text-xl text-paper-1">{"MP Stats"}</span>
                    <span class="font-mono text-[10px] tracking-[0.15em] uppercase text-theme-500 border border-theme-500/60 px-1.5 py-0.5 rounded-sm">{"Legacy"}</span>
                </Link<Route>>

                // Edition Links
                <nav class="hidden md:flex items-center gap-6">
                    { for PlatformEdition::iter().map(|edition| {
                        let is_active = matches!(
                            &route,
                            Route::Landing { edition: current } if current == edition
                        ) || matches!(
                            &route,
                            Route::Game { edition: current, .. } if current == edition
                        ) || matches!(
                            &route,
                            Route::Leaderboard { edition: current, .. } if current == edition
                        ) || matches!(
                            &route,
                            Route::Player { edition: current, .. } if current == edition
                        );
                        html! {
                            <Link<Route>
                                to={Route::Landing { edition: edition.clone() }}
                                classes={classes!("nav-link", if is_active { "active" } else { "" })}
                            >
                                { edition.display_name() }
                            </Link<Route>>
                        }
                    }) }
                </nav>

                // Search Bar (right)
                <div class="flex-1 max-w-sm ml-auto">
                    <SearchBar />
                </div>
            </div>
        </header>
    }
}
