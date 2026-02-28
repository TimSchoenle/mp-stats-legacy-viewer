use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::components::error_message::ErrorMessage;
use crate::components::leaderboards::board_type_selector::BoardTypeSelector;
use crate::components::leaderboards::header::LeaderboardHeader;
use crate::components::leaderboards::leaderboard_table::LeaderboardTable;
use crate::components::leaderboards::pagination_controls::PaginationControls;
use crate::components::leaderboards::snapshot_selector::SnapshotSelector;
use crate::hooks::{use_game_leaderboards, use_leaderboard_entries, use_theme};
use mp_stats_core::models::PlatformEdition;

#[derive(Properties, PartialEq, Clone)]
pub struct LeaderboardProps {
    pub edition: PlatformEdition,
    pub game: String,
    pub board: String,
    pub stat: String,
    pub page: u32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnapshotQuery {
    pub snapshot: String,
}

#[function_component(LeaderboardView)]
pub fn leaderboard_view(props: &LeaderboardProps) -> Html {
    let location = use_location().unwrap();
    let query: SnapshotQuery = location.query::<SnapshotQuery>().unwrap_or(SnapshotQuery {
        snapshot: "latest".to_string(), // Handle missing or malformed queries
    });

    let navigator = use_navigator().unwrap();

    // Fetch Game Metadata
    let game_req = use_game_leaderboards(props.edition.clone(), props.game.clone());

    // Compute active meta and snapshot based on game data
    let current_meta = use_memo(
        (
            game_req.data.clone(),
            props.board.clone(),
            props.stat.clone(),
        ),
        |(data, board, stat)| {
            data.as_ref()
                .and_then(|d| d.stats.get(stat.as_str()))
                .and_then(|stat_map| stat_map.get(board.as_str()))
                .cloned()
        },
    );

    let is_latest = query.snapshot == "latest";
    let current_snapshot_meta = use_memo(
        (current_meta.clone(), query.snapshot.clone()),
        |(meta, snapshot)| {
            let meta_ref = match meta.as_ref() {
                Some(m) => m,
                None => return None,
            };

            meta_ref
                .snapshots
                .iter()
                .find(|s| s.snapshot_id == *snapshot)
                .cloned()
        },
    );

    // Fetch Leaderboard Entries (Chunk)
    let entries_req = use_leaderboard_entries(
        props.edition.clone(),
        props.game.clone(),
        props.board.clone(),
        props.stat.clone(),
        props.page,
        (*current_snapshot_meta).clone(),
        is_latest,
    );

    let loading = game_req.loading || entries_req.loading;
    let error = game_req.error.clone().or(entries_req.error.clone());

    let total_pages = current_snapshot_meta
        .as_ref()
        .as_ref()
        .map(|meta| meta.total_pages)
        .unwrap_or(1);
    let max_page = if total_pages == 0 { 1 } else { total_pages };

    let change_page = {
        let navigator = navigator.clone();
        let props = props.clone();
        let query = query.clone();
        move |new_page: u32| {
            let route = Route::Leaderboard {
                edition: props.edition.clone(),
                game: props.game.clone(),
                board: props.board.clone(),
                stat: props.stat.clone(),
                page: new_page,
            };
            if query.snapshot == "latest" {
                navigator.push(&route);
            } else {
                navigator
                    .push_with_query(
                        &route,
                        &SnapshotQuery {
                            snapshot: query.snapshot.clone(),
                        },
                    )
                    .expect("Failed to navigate to new page");
            }
            if let Some(window) = web_sys::window() {
                window.scroll_to_with_x_and_y(0.0, 0.0);
            }
        }
    };

    let change_snapshot = {
        let navigator = navigator.clone();
        let props = props.clone();
        Callback::from(move |new_snapshot: String| {
            let route = Route::Leaderboard {
                edition: props.edition.clone(),
                game: props.game.clone(),
                board: props.board.clone(),
                stat: props.stat.clone(),
                page: 1,
            };
            if new_snapshot == "latest" {
                navigator.push(&route);
            } else {
                navigator
                    .push_with_query(
                        &route,
                        &SnapshotQuery {
                            snapshot: new_snapshot,
                        },
                    )
                    .expect("Failed to navigate");
            }
        })
    };

    let scroll_to_bottom = Callback::from(|_| {
        if let Some(window) = web_sys::window() {
            window.scroll_to_with_x_and_y(0.0, 100000.0);
        }
    });

    let boards = game_req
        .data
        .as_ref()
        .and_then(|d| d.stats.get(props.stat.as_str()))
        .map(|boards_map| boards_map.keys().map(|k| k.to_string()).collect::<Vec<_>>())
        .unwrap_or_default();

    let theme_color = use_theme();

    html! {
        <div class={classes!(theme_color, "container", "mx-auto", "px-4", "py-8", "text-white", "relative")}>
            <div class="mb-8">
                <LeaderboardHeader edition={props.edition.clone()} game={props.game.clone()} stat={props.stat.clone()} />
            </div>

            <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-6">
                <div>
                    if !boards.is_empty() {
                        <BoardTypeSelector
                            edition={props.edition.clone()}
                            game={props.game.clone()}
                            stat={props.stat.clone()}
                            current_board={props.board.clone()}
                            boards={boards}
                        />
                    }
                </div>

                <div class="flex items-center gap-3">
                    <SnapshotSelector
                        edition={props.edition.clone()}
                        current_snapshot={query.snapshot.clone()}
                        meta={(*current_meta).clone()}
                        on_change={change_snapshot}
                    />

                    <button
                        onclick={scroll_to_bottom}
                        class="px-4 py-2 bg-gray-800 hover:bg-gray-700 text-gray-300 hover:text-white rounded-lg transition-colors text-sm font-bold flex items-center gap-2"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
                        </svg>
                        {"Go to Bottom"}
                    </button>
                </div>
            </div>

            if let Some(err) = &error {
                <ErrorMessage title="Error Loading Data" message={err.clone()} />
            }

            if !loading && error.is_none() {
                <div class="card overflow-hidden">
                    <LeaderboardTable
                        game={props.game.clone()}
                        stat={props.stat.clone()}
                        entries={entries_req.entries.clone()}
                        edition={props.edition.clone()}
                    />
                    <PaginationControls
                        edition={props.edition.clone()}
                        current_page={props.page}
                        max_page={max_page}
                        on_change={Callback::from(change_page)}
                    />
                    if entries_req.entries.is_empty() {
                        <div class="p-12 text-center text-gray-500">
                             if game_req.data.is_some() {
                                 {"No entries found for this leaderboard."}
                             } else {
                                 {"Game data empty."}
                             }
                        </div>
                    }
                </div>
            }

            if loading {
                <div class="card p-12 text-center text-gray-500">
                    <div class={classes!("animate-spin", "h-6", "w-6", "border-2", "border-theme-500", "border-t-transparent", "rounded-full", "mx-auto", "mb-2")}></div>
                    {"Loading leaderboard..."}
                </div>
            }
        </div>
    }
}
