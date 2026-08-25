//! The bar across the top of every page.

use crate::Route;
use crate::components::search_bar::SearchBar;
use crate::hooks::use_theme;
use dioxus::prelude::*;
use mp_stats_core::models::PlatformEdition;

/// The edition a route is inside, or `None` for the routes that name no platform.
///
/// The edition link in the bar is marked active for any route naming that edition, not just its
/// landing page, so a reader three levels deep can still see which platform they are in.
fn route_edition(route: &Route) -> Option<&PlatformEdition> {
    match route {
        Route::Landing { edition }
        | Route::Game { edition, .. }
        | Route::Leaderboard { edition, .. }
        | Route::Player { edition, .. } => Some(edition),
        Route::Home {} | Route::NotFound { .. } => None,
    }
}

/// The bar on every page: the mark, a link per edition, and the search box.
///
/// Mounted by [`AppShell`](crate::app::AppShell) rather than by each page, so it is built once
/// and a navigation repaints only the class that carries the theme.
#[component]
pub fn Header() -> Element {
    let route = use_route::<Route>();
    let theme_color = use_theme();
    let active_edition = route_edition(&route);

    rsx! {
        header {
            class: "{theme_color} sticky top-0 z-50 w-full border-b border-rule bg-ink-0/90 backdrop-blur-md",
            div { class: "container mx-auto px-6 h-14 flex items-center gap-8",

                // Brand
                Link { to: Route::Home {}, class: "flex items-baseline gap-2.5 group",
                    span { class: "serif text-xl text-paper-1", "MP Stats" }
                    span {
                        class: "font-mono text-[10px] tracking-[0.15em] uppercase text-theme-500 border border-theme-500/60 px-1.5 py-0.5 rounded-sm",
                        "Legacy"
                    }
                }

                // Edition links
                nav { class: "hidden md:flex items-center gap-6",
                    for edition in PlatformEdition::iter() {
                        Link {
                            key: "{edition}",
                            to: Route::Landing { edition: edition.clone() },
                            class: if active_edition == Some(edition) { "nav-link active" } else { "nav-link" },
                            {edition.display_name()}
                        }
                    }
                }

                // Search bar (right)
                div { class: "flex-1 max-w-sm ml-auto",
                    SearchBar {}
                }
            }
        }
    }
}
