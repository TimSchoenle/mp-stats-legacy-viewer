//! One board of one category, at one page and one snapshot.

use crate::Route;
use crate::components::error_message::ErrorMessage;
use crate::components::leaderboards::board_type_selector::BoardTypeSelector;
use crate::components::leaderboards::header::LeaderboardHeader;
use crate::components::leaderboards::leaderboard_table::LeaderboardTable;
use crate::components::leaderboards::pagination_controls::PaginationControls;
use crate::components::leaderboards::snapshot_selector::SnapshotSelector;
use crate::hooks::{use_game_leaderboards, use_leaderboard_entries, use_theme};
use crate::route::Snapshot;
use dioxus::prelude::*;
use mp_stats_core::models::PlatformEdition;

/// Scroll the document to `y`, or do nothing where there is no window to scroll.
fn scroll_to(y: f64) {
    if let Some(window) = web_sys::window() {
        window.scroll_to_with_x_and_y(0.0, y);
    }
}

/// Far enough down that any page of any board ends above it.
const BOTTOM: f64 = 100_000.0;

/// One page of one board: the title, the board tabs, the snapshot timeline, the table and the
/// page controls.
///
/// Every argument comes straight off the route: `edition`, `game`, `board` and `stat` name the
/// file, `page` is 1-based, and `snapshot` says whether to read the current standings or an
/// archived dump.
#[component]
pub fn LeaderboardView(
    edition: PlatformEdition,
    game: String,
    board: String,
    stat: String,
    page: u32,
    snapshot: Snapshot,
) -> Element {
    let navigator = use_navigator();
    let theme_color = use_theme();

    let game_req = use_game_leaderboards(edition.clone(), game.clone());

    // The metadata for this one board of this one category, and within it the one snapshot being
    // viewed. The tree files the current standings under `latest` alongside the archived dumps, so
    // both cases are the same lookup.
    let board_meta = game_req.data.as_ref().and_then(|data| {
        data.stats
            .get(stat.as_str())
            .and_then(|boards| boards.get(board.as_str()))
            .cloned()
    });

    let snapshot_meta = board_meta.as_ref().and_then(|meta| {
        meta.snapshots
            .iter()
            .find(|candidate| candidate.snapshot_id == snapshot.as_str())
            .cloned()
    });

    let entries_req = use_leaderboard_entries(
        edition.clone(),
        game.clone(),
        board.clone(),
        stat.clone(),
        page,
        snapshot_meta.clone(),
        snapshot.is_latest(),
    );

    let loading = game_req.loading || entries_req.loading;
    let error = game_req.error.clone().or(entries_req.error.clone());

    let max_page = snapshot_meta
        .as_ref()
        .map_or(1, |meta| meta.total_pages)
        .max(1);

    let boards: Vec<String> = game_req
        .data
        .as_ref()
        .and_then(|data| data.stats.get(stat.as_str()))
        .map(|boards| boards.keys().map(ToString::to_string).collect())
        .unwrap_or_default();

    // Both controls rewrite the same route, and each leaves the other's half of it alone: a page
    // button keeps the snapshot, and the timeline returns to page 1 because a page number taken
    // from one dump means nothing in another.
    let to_page = {
        let (edition, game, board, stat, snapshot) = (
            edition.clone(),
            game.clone(),
            board.clone(),
            stat.clone(),
            snapshot.clone(),
        );

        move |new_page: u32| {
            navigator.push(Route::Leaderboard {
                edition: edition.clone(),
                game: game.clone(),
                board: board.clone(),
                stat: stat.clone(),
                page: new_page,
                snapshot: snapshot.clone(),
            });
            scroll_to(0.0);
        }
    };

    let to_snapshot = {
        let (edition, game, board, stat) =
            (edition.clone(), game.clone(), board.clone(), stat.clone());

        move |new_snapshot: Snapshot| {
            navigator.push(Route::Leaderboard {
                edition: edition.clone(),
                game: game.clone(),
                board: board.clone(),
                stat: stat.clone(),
                page: 1,
                snapshot: new_snapshot,
            });
        }
    };

    rsx! {
        div {
            class: "{theme_color} container mx-auto px-6 py-8 max-w-6xl xl:max-w-7xl 2xl:max-w-[1600px]",

            LeaderboardHeader {
                edition: edition.clone(),
                game: game.clone(),
                stat: stat.clone(),
            }

            // Snapshot timeline card
            div { class: "mt-6",
                SnapshotSelector {
                    current_snapshot: snapshot.clone(),
                    meta: board_meta,
                    on_change: to_snapshot,
                }
            }

            // Controls row: board selector + go-to-bottom
            div {
                class: "flex flex-col md:flex-row md:items-center justify-between gap-3 mt-6 mb-4",
                div {
                    if !boards.is_empty() {
                        BoardTypeSelector {
                            edition: edition.clone(),
                            game: game.clone(),
                            stat: stat.clone(),
                            current_board: board.clone(),
                            boards,
                        }
                    }
                }

                button {
                    class: "btn-ghost text-xs font-mono",
                    onclick: move |_| scroll_to(BOTTOM),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-3.5 w-3.5",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M19 14l-7 7m0 0l-7-7m7 7V3",
                        }
                    }
                    "Go to bottom"
                }
            }

            if let Some(error) = error {
                ErrorMessage { title: "Error loading data", message: error }
            } else if loading {
                div { class: "card p-12 text-center",
                    div {
                        class: "animate-spin h-5 w-5 border-2 border-theme-500 border-t-transparent rounded-full mx-auto mb-3",
                    }
                    p { class: "text-sm text-paper-3", "Loading leaderboard\u{2026}" }
                }
            } else {
                div { class: "card overflow-hidden",
                    LeaderboardTable {
                        game: game.clone(),
                        stat: stat.clone(),
                        entries: entries_req.entries.clone(),
                        edition: edition.clone(),
                    }
                    PaginationControls {
                        current_page: page,
                        max_page,
                        on_change: to_page,
                    }
                    if entries_req.entries.is_empty() {
                        div { class: "p-12 text-center",
                            p { class: "text-sm text-paper-3",
                                if game_req.data.is_some() {
                                    "No entries found for this leaderboard."
                                } else {
                                    "Game data empty."
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
