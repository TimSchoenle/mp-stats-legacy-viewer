use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::leaderboards::header::LeaderboardHeader;
use crate::components::leaderboards::pagination_controls::PaginationControls;
use crate::models::{GameLeaderboardData, LeaderboardEntry};
use crate::{Api, Route};
use mp_stats_core::models::{LeaderboardMeta, PlatformEdition};
use mp_stats_core::{ENTRIES_PER_PAGE_F64, HistoricalSnapshot};
use yew::platform::spawn_local;
use yew::{
    Callback, Html, Properties, function_component, html, use_context, use_effect_with, use_state,
};

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

fn sorted_board_types(mut boards: Vec<String>) -> Vec<String> {
    fn get_rank(board: &str) -> u8 {
        match board.to_lowercase().as_str() {
            "all" => 0,
            "yearly" => 1,
            "monthly" => 2,
            "weekly" => 3,
            "daily" => 4,
            _ => 5,
        }
    }

    boards.sort_by(|a, b| {
        let rank_a = get_rank(a);
        let rank_b = get_rank(b);

        rank_a.cmp(&rank_b).then_with(|| a.cmp(b))
    });

    boards
}

#[function_component(LeaderboardView)]
pub fn leaderboard_view(props: &LeaderboardProps) -> Html {
    let location = use_location().unwrap();
    let query: SnapshotQuery = location.query::<SnapshotQuery>().unwrap_or(SnapshotQuery {
        snapshot: "latest".to_string(), // Handle missing or malformed queries
    });

    let game_data = use_state(|| None::<GameLeaderboardData>);
    let api_ctx = use_context::<Api>().expect("no api found found");
    let navigator = use_navigator().unwrap();

    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    let current_meta = use_state(|| None::<LeaderboardMeta>);
    let current_snapshot_meta = use_state(|| None::<HistoricalSnapshot>);
    let entries = use_state(|| Vec::<LeaderboardEntry>::new());
    let input_page = use_state(|| props.page.to_string());

    // Sync input value when props page changes
    {
        let input_page = input_page.clone();
        let page = props.page;
        use_effect_with(page, move |p| {
            input_page.set(p.to_string());
        });
    }

    // Fetch Game Data (Metadata)
    {
        let game_data = game_data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let context = api_ctx.clone();
        let game_id = props.game.clone();
        let edition = props.edition.clone();

        use_effect_with((game_id, context.clone()), move |(game, ctx)| {
            // Reset error
            error.set(None);

            let game = game.clone();
            let provider = ctx.clone();
            loading.set(true);
            spawn_local(async move {
                match provider.fetch_game_leaderboards(&edition, &game).await {
                    Ok(data) => {
                        game_data.set(Some(data));
                        // Loading is NOT set to false here, we wait for entries
                    }
                    Err(e) => {
                        loading.set(false);
                        error.set(Some(format!("Failed to load game data: {}", e)));
                    }
                }
            });

            || ()
        });
    }

    // Update current game data
    {
        let current_meta = current_meta.clone();
        let game_data = game_data.clone();
        let props = props.clone();

        use_effect_with(
            (game_data.clone(), props.board.clone(), props.stat.clone()),
            move |(game_data, board, stat)| {
                if let Some(data) = game_data.as_ref() {
                    if let Some(stat_map) = data.stats.get(stat.as_str()) {
                        if let Some(meta) = stat_map.get(board.as_str()) {
                            current_meta.set(Some(meta.clone()));
                            return;
                        }
                    }
                }

                current_meta.set(None);
            },
        )
    }

    // Update current snapshot meta
    {
        let current_snapshot_meta = current_snapshot_meta.clone();
        let current_meta = current_meta.clone();
        let props = props.clone();

        use_effect_with(
            (
                current_meta.clone(),
                props.game.clone(),
                props.board.clone(),
                props.stat.clone(),
                query.snapshot.clone(),
            ),
            move |(current_meta, _game, _board, stat, snapshot)| {
                // Update total entries from metadata or snapshot
                let is_latest = snapshot == "latest";

                if let Some(data) = current_meta.as_ref() {
                    if is_latest {
                        current_snapshot_meta.set(data.latest.clone());
                        return;
                    } else if let Some(snap) =
                        data.snapshots.iter().find(|s| s.snapshot_id == *snapshot)
                    {
                        current_snapshot_meta.set(Some(snap.clone()));
                        return;
                    }
                }

                current_snapshot_meta.set(None);
            },
        )
    }

    // Fetch Entries (Chunk)
    {
        let current_snapshot_meta = current_snapshot_meta.clone();
        let entries = entries.clone();
        let loading = loading.clone();
        let error = error.clone();
        let page = props.page;
        let props = props.clone();
        let context = api_ctx.clone();

        use_effect_with(
            (current_snapshot_meta.clone(), page.clone()),
            move |(snapshot, page_captured)| {
                // Reset error state
                error.set(None);

                if let Some(snapshot_data) = snapshot.as_ref() {
                    // Update total entries from metadata or snapshot

                    let page_idx = *page_captured - 1; // 0-based chunk
                    let provider = context.clone();
                    let snapshot_data = snapshot_data.clone();

                    loading.set(true);
                    spawn_local(async move {
                        let result = if snapshot_data.snapshot_id == "latest" {
                            provider
                                .fetch_leaderboard(
                                    &props.edition,
                                    &props.board,
                                    &props.game,
                                    &props.stat,
                                    page_idx,
                                )
                                .await
                        } else {
                            provider
                                .fetch_history_leaderboard(
                                    &props.edition,
                                    &props.board,
                                    &props.game,
                                    &props.stat,
                                    &snapshot_data.snapshot_id,
                                    page_idx,
                                )
                                .await
                        };

                        match result {
                            Ok(data) => {
                                entries.set(data);
                                loading.set(false);
                            }
                            Err(e) => {
                                loading.set(false);
                                // If it's a 404 on a chunk, passing empty list might be better than error if metadata says 0
                                if e.to_string().contains("404") {
                                    entries.set(vec![]);
                                } else {
                                    error.set(Some(format!("Failed to fetch chunk: {}", e)));
                                }
                            }
                        }
                    });
                }
                || ()
            },
        );
    }

    let current_entries = entries.clone();
    let max_page = current_snapshot_meta
        .as_ref()
        .map(|meta| meta.total_pages)
        .unwrap_or(1);
    let max_page = (max_page as f64 / ENTRIES_PER_PAGE_F64).ceil() as u32;
    let max_page = if max_page == 0 { 1 } else { max_page };

    let change_page = {
        let navigator = navigator.clone();
        let props = props.clone();
        let query = query.clone();
        move |new_page: u32| {
            if query.snapshot.clone() == "latest" {
                navigator.push(&Route::Leaderboard {
                    edition: props.edition.clone(),
                    game: props.game.clone(),
                    board: props.board.clone(),
                    stat: props.stat.clone(),
                    page: new_page,
                });
            } else {
                navigator
                    .push_with_query(
                        &Route::Leaderboard {
                            edition: props.edition.clone(),
                            game: props.game.clone(),
                            board: props.board.clone(),
                            stat: props.stat.clone(),
                            page: new_page,
                        },
                        &SnapshotQuery {
                            snapshot: query.snapshot.clone(),
                        },
                    )
                    .expect("Failed to navigate to new page");
            }
            // Scroll to top
            if let Some(window) = web_sys::window() {
                window.scroll_to_with_x_and_y(0.0, 0.0);
            }
        }
    };

    let change_snapshot = {
        let navigator = navigator.clone();
        let props = props.clone();
        move |new_snapshot: String| {
            if new_snapshot == "latest" {
                navigator.push(&Route::Leaderboard {
                    edition: props.edition.clone(),
                    game: props.game.clone(),
                    board: props.board.clone(),
                    stat: props.stat.clone(),
                    page: 1, // Reset to page 1 on snapshot change
                });
            } else {
                navigator
                    .push_with_query(
                        &Route::Leaderboard {
                            edition: props.edition.clone(),
                            game: props.game.clone(),
                            board: props.board.clone(),
                            stat: props.stat.clone(),
                            page: 1, // Reset to page 1 on snapshot change
                        },
                        &SnapshotQuery {
                            snapshot: new_snapshot,
                        },
                    )
                    .expect("Failed to navigate to new page");
            }
        }
    };

    let scroll_to_bottom = Callback::from(|_| {
        if let Some(window) = web_sys::window() {
            window.scroll_to_with_x_and_y(0.0, 100000.0); // Simple hack to scroll to bottom
        }
    });

    html! {
        <div class="container mx-auto px-4 py-8 text-white relative">
            <div class="mb-8 flex flex-col md:flex-row md:items-center justify-between gap-4">
                <LeaderboardHeader edition={props.edition.clone()} game={props.game.clone()} stat={props.stat.clone()} />

                // Snapshot Selector & Go to Bottom Button
                <div class="flex items-center gap-3">
                    // Time Snapshot Selector
                    if let Some(current_meta) = current_meta.as_ref() {
                        if !current_meta.snapshots.is_empty() {
                            <div class="flex flex-col gap-1">
                                <label class="text-xs text-gray-400 font-medium">{"Snapshot:"}</label>
                                <select
                                    value={query.snapshot.clone()}
                                    onchange={
                                        let change_snapshot = change_snapshot.clone();
                                        Callback::from(move |e: Event| {
                                            let target: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                            change_snapshot(target.value());
                                        })
                                    }
                                    class="px-3 py-2 bg-gray-800 border border-gray-700 hover:border-gray-600 rounded-lg text-sm text-white cursor-pointer focus:outline-none focus:border-emerald-500 transition-colors"
                                >
                                    <option value="latest" selected={query.snapshot == "latest"}>
                                        {"Latest"}
                                    </option>
                                    {for current_meta.snapshots.iter().map(|snap| {
                                        html! {
                                            <option
                                                value={snap.snapshot_id.to_string()}
                                                selected={query.snapshot == snap.snapshot_id.as_str()}
                                            >
                                                {snap.snapshot_id.to_string()}
                                            </option>
                                        }
                                    })}
                                </select>
                            </div>
                        }
                    }

                    // Go to Bottom Button
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

            // Board Type Selector Tabs
            <div class="flex gap-1 mb-6 bg-gray-800 rounded-lg p-1 w-fit">
                if let Some(game_data) = game_data.as_ref() && let Some(stat_data) = game_data.stats.get(props.stat.as_str()) {
                            { for sorted_board_types(stat_data.keys().map(|s| s.to_string()).collect()).iter().map(|board| {
                    let is_active = *board == props.board;
                    let classes = if is_active {
                        "px-4 py-2 rounded-md text-sm font-bold bg-emerald-600 text-white transition-all"
                    } else {
                        "px-4 py-2 rounded-md text-sm font-medium text-gray-400 hover:text-white hover:bg-gray-700 transition-all cursor-pointer"
                    };


                    let route = Route::Leaderboard {
                                            edition: props.edition.clone(),
                            game: props.game.clone(),
                            board: board.to_string(),
                            stat: props.stat.clone(),
                            page: 1, // Reset to page 1 on board switch
                        };

                    html! {
                        <Link<Route, SnapshotQuery>
                            to={route}
                            classes={classes}
                        >
                            { board.to_string() }
                        </Link<Route, SnapshotQuery>>
                    }
                }) }
                }

            </div>

            // Error message
            if let Some(err) = &*error {
                <div class="card p-8 text-center text-gray-500">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 mx-auto mb-3 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-2.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
                    </svg>
                    <p class="font-medium">{"No data found"}</p>
                    <p class="text-sm mt-1 text-gray-600">{ err }</p>
                </div>
            }

            // Table
            if !*loading && error.is_none() {
                <div class="card overflow-hidden">
                    <div class="overflow-x-auto">
                        <table class="w-full text-left border-collapse">
                            <thead>
                                <tr class="bg-gray-800 border-b border-gray-700 text-gray-400 text-sm uppercase tracking-wider">
                                    <th class="p-4 font-medium w-16">{ "Rank" }</th>
                                    <th class="p-4 font-medium">{ "Player" }</th>
                                    <th class="p-4 font-medium text-right">{ "Score" }</th>
                                </tr>
                            </thead>
                            <tbody class="divide-y divide-gray-700">
                                { for current_entries.iter().map(|row| {
                                    html! {
                                    <tr class="hover:bg-gray-800 transition-colors group">
                                        <td class="p-4 font-bold text-gray-500 group-hover:text-emerald-400 transition-colors">
                                            { format!("#{}", row.rank) }
                                        </td>
                                        <td class="p-4">
                                             <Link<Route> to={Route::Player { edition: props.edition.clone(), uuid: row.uuid.to_string() }} classes="flex items-center gap-3 group/link">
                                                <img
                                                    src={format!("https://mc-heads.net/avatar/{}/32", row.uuid)}
                                                    class="w-8 h-8 rounded bg-gray-900 shadow-sm"
                                                    alt="Avatar"
                                                    loading="lazy"
                                                />
                                                <span class="font-bold text-gray-200 group-hover/link:text-emerald-400 transition-colors font-mono tracking-tight">
                                                    { row.name.as_str() }
                                                </span>
                                            </Link<Route>>
                                        </td>
                                        <td class="p-4 text-right font-mono font-bold text-lg text-white">
                                            { row.score }
                                        </td>
                                    </tr>
                                }}) }
                            </tbody>
                        </table>
                    </div>

                    // Pagination
                    <PaginationControls
                        current_page={props.page}
                        max_page={max_page}
                        on_change={Callback::from(move |p| change_page(p))}
                    />

                    if current_entries.is_empty() && !*loading {
                        <div class="p-12 text-center text-gray-500">
                             // Check if we have data for the game at all?
                             if game_data.is_some() {
                                 {"No entries found for this leaderboard."}
                             } else {
                                 {"Game data empty."}
                             }
                        </div>
                    }
                </div>
            }

            // Loading state
            if *loading {
                <div class="card p-12 text-center text-gray-500">
                    <div class="animate-spin h-6 w-6 border-2 border-emerald-500 border-t-transparent rounded-full mx-auto mb-2"></div>
                    {"Loading leaderboard..."}
                </div>
            }
        </div>
    }
}
