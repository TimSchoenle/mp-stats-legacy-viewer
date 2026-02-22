use crate::Route;
use crate::components::error_message::ErrorMessage;
use crate::hooks::use_player_profile;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PlayerProps {
    pub edition: PlatformEdition,
    pub uuid: String,
}

#[function_component(PlayerView)]
pub fn player_view(props: &PlayerProps) -> Html {
    let profile_req = use_player_profile(props.edition.clone(), props.uuid.clone());

    html! {
        <div class="container mx-auto px-4 py-8 text-white">
            // Breadcrumb Navigation
            <div class="flex items-center text-sm text-gray-400 mb-2 space-x-2">
                <Link<Route> to={Route::Home} classes="hover:text-white transition">{"Home"}</Link<Route>>
                <span>{"/"}</span>
                <Link<Route> to={Route::Landing { edition: props.edition.clone() }} classes="hover:text-white transition">{props.edition.display_name()}</Link<Route>>
                <span>{"/"}</span>
                <span class="text-white">{"Player"}</span>
            </div>

            // Error Message
            if let Some(err) = &profile_req.error {
                <ErrorMessage title="Error Loading Profile" message={err.clone()} is_banner={true} />
            }

            if let Some(p) = &profile_req.profile {
                // --- Profile Header ---
                <div class="card p-6 mb-6 flex flex-col md:flex-row items-center gap-6">
                    // Avatar
                    <div class="flex-shrink-0">
                        <img
                            src={format!("https://mc-heads.net/avatar/{}/128", p.uuid)}
                            class="w-24 h-24 rounded-lg shadow-lg bg-gray-900"
                            alt={p.name.as_ref().map(|s| s.as_str()).unwrap_or("Player").to_string()}
                        />
                    </div>

                    // Info
                    <div class="text-center md:text-left flex-1">
                        <div class="flex flex-col md:flex-row md:items-baseline gap-2 mb-2">
                            <h1 class="text-3xl font-bold text-white tracking-tight">
                                { p.name.as_ref().map(|s| s.as_str()).unwrap_or("Unknown") }
                            </h1>
                            <span class="px-2 py-0.5 rounded-full bg-emerald-900 text-emerald-400 text-xs font-mono border border-emerald-700">
                                { "Java Edition" }
                            </span>
                        </div>
                        <p class="font-mono text-gray-400 bg-gray-900 px-3 py-1 rounded inline-block text-sm border border-gray-700 select-all">
                            { p.uuid.as_str() }
                        </p>
                    </div>
                </div>

                // --- Stats Grid ---
                if let Some(map) = &profile_req.id_map {
                    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
                        {
                            // Group stats by game
                            {
                                use std::collections::BTreeMap;
                                let mut games: BTreeMap<String, Vec<&crate::models::StatRaw>> = BTreeMap::new();

                                for stat in &p.stats {
                                    let game_name = map.games.get(&stat.game_id).map(|s| s.as_str()).unwrap_or("Unknown Game").to_string();
                                    games.entry(game_name).or_default().push(stat);
                                }

                                games.into_iter().map(|(game_name, stats)| {
                                    html! {
                                        <div class="card p-5 flex flex-col h-full">
                                            <h3 class="text-lg font-bold mb-3 pb-2 border-b border-gray-700 text-white">
                                                { game_name }
                                            </h3>

                                            <div class="space-y-2 flex-1">
                                                { for stats.iter().map(|s| {
                                                    let board_name = map.boards.get(&s.board_id).map(|s| s.as_str()).unwrap_or("Board");
                                                    let stat_name = map.stats.get(&s.stat_id).map(|s| s.as_str()).unwrap_or("Stat");

                                                    // Clean up names (remove redundancy if needed)
                                                    let label = if board_name == "All" {
                                                        stat_name.to_string()
                                                    } else {
                                                        format!("{} ({})", stat_name, board_name)
                                                    };

                                                    html! {
                                                        <div class="stat-row">
                                                            <span class="text-gray-400 text-sm">{ label }</span>
                                                            <div class="flex items-center gap-2">
                                                                <span class="font-bold font-mono text-white">{ s.score }</span>
                                                                <span class="rank-badge">{ format!("#{}", s.rank) }</span>
                                                            </div>
                                                        </div>
                                                    }
                                                })}
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </div>
                } else if profile_req.loading {
                    // Loading State
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 animate-pulse">
                        <div class="h-64 bg-gray-800 rounded-lg"></div>
                        <div class="h-64 bg-gray-800 rounded-lg"></div>
                        <div class="h-64 bg-gray-800 rounded-lg"></div>
                    </div>
                }
            } else if profile_req.loading {
                <div class="flex items-center justify-center p-20">
                    <div class="flex flex-col items-center gap-4 text-gray-500">
                        <div class="animate-spin h-8 w-8 border-2 border-emerald-500 border-t-transparent rounded-full"></div>
                        <p>{ "Loading Profile..." }</p>
                    </div>
                </div>
            }
        </div>
    }
}
