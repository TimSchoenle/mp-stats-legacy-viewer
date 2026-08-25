//! The title block of the leaderboard page.

use crate::Route;
use crate::hooks::use_theme;
use dioxus::prelude::*;
use mp_stats_core::models::PlatformEdition;

/// The breadcrumb trail and title above a leaderboard.
///
/// `edition` is both a crumb and the theme, `game` is shown as-is, and `stat` has its underscores
/// turned into spaces before it is shown.
#[component]
pub fn LeaderboardHeader(edition: PlatformEdition, game: String, stat: String) -> Element {
    let theme_color = use_theme();
    let stat_display = stat.replace('_', " ");

    rsx! {
        div { class: "{theme_color}",

            // Crumbs
            div { class: "crumbs mb-5",
                Link { to: Route::Home {}, "Home" }
                span { class: "sep", "/" }
                Link { to: Route::Landing { edition: edition.clone() }, {edition.display_name()} }
                span { class: "sep", "/" }
                Link {
                    to: Route::Game { edition: edition.clone(), game: game.clone() },
                    "{game}"
                }
                span { class: "sep", "/" }
                span { class: "here capitalize", "{stat_display}" }
            }

            // Title
            div { class: "flex flex-col md:flex-row md:items-end justify-between gap-3",
                div {
                    div { class: "eyebrow mb-2", "{game} \u{b7} Category" }
                    h1 { class: "serif page-title text-5xl md:text-6xl text-paper-1 capitalize",
                        span { class: "text-paper-3", "Top " }
                        "{stat_display}"
                    }
                }
            }
        }
    }
}
