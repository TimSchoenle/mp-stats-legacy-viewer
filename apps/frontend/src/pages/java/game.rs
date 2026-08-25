//! One game's categories.

use crate::Route;
use crate::components::error_message::ErrorMessage;
use crate::hooks::{use_game_leaderboards, use_theme};
use crate::route::Snapshot;
use crate::util::score_formatter::create_score_formatter;
use dioxus::prelude::*;
use mp_stats_core::models::{GLOBAL_BOARD, PlatformEdition, TopEntry};

/// One game's categories, each with the all-time leader beside it.
///
/// Every category links into the all-time board, since that is the only one whose leader is
/// recorded. The other boards are reached from the tabs on the leaderboard page.
///
/// `edition` is the platform the game belongs to and `game` is its directory in the tree.
#[component]
pub fn GameView(edition: PlatformEdition, game: String) -> Element {
    let game_req = use_game_leaderboards(edition.clone(), game.clone());
    let theme_color = use_theme();

    let mut stats = game_req.data.as_ref().map_or_else(Vec::new, |data| {
        data.stats.keys().map(ToString::to_string).collect()
    });
    stats.sort_by_key(|stat| stat.to_lowercase());

    // The `#1 holder` of a category, strictly from the latest snapshot of the global board.
    // `None` - rendered as an em dash - when the global board or its top entry is missing, so
    // missing data is handled gracefully.
    let top_holder = |stat: &str| -> Option<TopEntry> {
        game_req.data.as_ref().and_then(|data| {
            data.stats
                .get(stat)
                .and_then(|boards| boards.get(GLOBAL_BOARD))
                .and_then(|meta| meta.top.clone())
        })
    };

    let snapshot_count = game_req
        .data
        .as_ref()
        .map_or(0, |data| data.total_snapshots);
    let stat_count = stats.len();

    rsx! {
        div {
            class: "{theme_color} container mx-auto px-6 py-8 max-w-6xl xl:max-w-7xl 2xl:max-w-[1600px]",

            // Crumbs
            div { class: "crumbs mb-5",
                Link { to: Route::Home {}, "Home" }
                span { class: "sep", "/" }
                Link { to: Route::Landing { edition: edition.clone() }, {edition.display_name()} }
                span { class: "sep", "/" }
                span { class: "here", "{game}" }
            }

            // Header
            div {
                class: "flex flex-col md:flex-row justify-between items-start md:items-end gap-6 pb-7 border-b border-rule",
                div { class: "flex items-start gap-5 min-w-0",
                    if let Some(icon_url) = game_req.data.as_ref().and_then(|data| data.icon.clone()) {
                        div {
                            class: "w-16 h-16 bg-ink-2 rounded-lg flex items-center justify-center shrink-0 border border-rule overflow-hidden",
                            img {
                                src: "{icon_url}",
                                class: "w-10 h-10 object-contain",
                                alt: "{game}",
                            }
                        }
                    }
                    div { class: "min-w-0",
                        div { class: "eyebrow mb-2",
                            if snapshot_count == 0 {
                                "Game \u{b7} {stat_count} categories"
                            } else {
                                "Game \u{b7} {stat_count} categories \u{b7} {snapshot_count} snapshots"
                            }
                        }
                        h1 {
                            class: "serif page-title text-5xl md:text-6xl text-paper-1 break-words",
                            "{game}"
                        }
                        if let Some(description) = game_req.data.as_ref().and_then(|data| data.description.clone()) {
                            p { class: "mt-3 text-sm text-paper-3 max-w-2xl leading-relaxed",
                                "{description}"
                            }
                        }
                        p { class: "mt-2 text-sm text-paper-4",
                            "Pick a category to view its leaderboard. Each board defaults to the latest snapshot."
                        }
                    }
                }
            }

            if let Some(error) = game_req.error.clone() {
                div { class: "mt-6",
                    ErrorMessage { title: "Error loading game data", message: error }
                }
            } else if game_req.loading {
                div {
                    class: "mt-7 grid grid-cols-1 gap-px bg-rule border border-rule rounded-lg overflow-hidden animate-pulse",
                    for index in 0..6 {
                        div { key: "{index}", class: "h-14 bg-ink-2" }
                    }
                }
            } else if stats.is_empty() {
                div { class: "mt-6 card p-12 text-center",
                    p { class: "text-paper-3 text-sm", "No statistics found for this game." }
                }
            } else {
                // Table: header + rows share one grid so columns self-balance and stay aligned
                div {
                    class: "mt-7 grid grid-cols-[40px_1fr_80px] md:grid-cols-[40px_1fr_minmax(160px,1fr)_minmax(120px,auto)_80px] border border-rule rounded-lg overflow-hidden",

                    // Eyebrow row (table-style header)
                    div {
                        class: "col-span-full grid grid-cols-subgrid gap-4 px-4 py-3 bg-ink-1 border-b border-rule",
                        span { class: "eyebrow", "#" }
                        span { class: "eyebrow", "Category" }
                        span { class: "eyebrow hidden md:block", "#1 holder (latest)" }
                        span { class: "eyebrow hidden md:block text-right", "Top score" }
                        span { class: "eyebrow text-right" }
                    }

                    for (index , stat) in stats.iter().enumerate() {
                        {
                            let description = game_req.id_map.as_ref().and_then(|map| {
                                map.stats
                                    .values()
                                    .find(|entry| entry.name == stat.as_str())
                                    .and_then(|entry| entry.description.clone())
                            });
                            let top = top_holder(stat);
                            let formatter = create_score_formatter(&game, stat);
                            let position = index + 1;
                            let label = stat.replace('_', " ");

                            rsx! {
                                Link {
                                    key: "{stat}",
                                    to: Route::Leaderboard {
                                        edition: edition.clone(),
                                        game: game.clone(),
                                        board: "All".to_string(),
                                        stat: stat.clone(),
                                        page: 1,
                                        snapshot: Snapshot::Latest,
                                    },
                                    class: "col-span-full grid grid-cols-subgrid gap-4 items-center bg-ink-2 hover:bg-ink-3 transition-colors px-4 py-3.5 border-b border-rule-soft last:border-0 group",

                                    span { class: "font-mono text-xs text-paper-3", "{position:02}" }

                                    div { class: "min-w-0",
                                        div { class: "flex items-center gap-2.5",
                                            span { class: "w-1.5 h-1.5 rounded-full bg-theme-500" }
                                            span {
                                                class: "text-base font-medium text-paper-1 capitalize",
                                                "{label}"
                                            }
                                        }
                                        if let Some(description) = description {
                                            p { class: "text-xs text-paper-3 mt-0.5 pl-4 line-clamp-1",
                                                "{description}"
                                            }
                                        }
                                    }

                                    // #1 holder (latest)
                                    div { class: "hidden md:flex items-center gap-2 min-w-0",
                                        if let Some(top) = top.as_ref() {
                                            img {
                                                src: "https://mc-heads.net/avatar/{top.uuid}/32",
                                                class: "w-[18px] h-[18px] rounded bg-ink-3 border border-rule shrink-0",
                                                alt: "Avatar",
                                                loading: "lazy",
                                            }
                                            span { class: "font-mono text-xs text-paper-2 truncate",
                                                "{top.name}"
                                            }
                                        } else {
                                            span { class: "font-mono text-xs text-paper-4", "\u{2014}" }
                                        }
                                    }

                                    // Top score
                                    span {
                                        class: "hidden md:block text-right font-mono text-sm text-paper-1 tnum whitespace-nowrap",
                                        match top.as_ref() {
                                            Some(top) => formatter.format_score(top.score),
                                            None => "\u{2014}".to_string(),
                                        }
                                    }

                                    span {
                                        class: "text-right text-xs font-mono text-paper-3 group-hover:text-theme-400 transition-colors",
                                        "view \u{2192}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
