use crate::{Api, Route};
use mp_stats_core::models::PlatformEdition;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PlayerProps {
    pub edition: PlatformEdition,
    pub uuid: String,
}

#[function_component(PlayerView)]
pub fn player_view(props: &PlayerProps) -> Html {
    let profile = use_state(|| None);
    let id_map = use_state(|| None);
    let error = use_state(|| None::<String>);
    let uuid = props.uuid.clone();
    let api_ctx = use_context::<Api>().expect("no api found found");

    {
        let profile = profile.clone();
        let id_map = id_map.clone();
        let error = error.clone();
        let edition = props.edition.clone();

        use_effect_with((uuid, api_ctx), move |(id, ctx)| {
            let id = id.clone();
            let provider = ctx.clone();
            spawn_local(async move {
                // Fetch profile first
                let p_res = provider.fetch_player(&edition, &id).await;
                match p_res {
                    Ok(p) => profile.set(Some(p)),
                    Err(e) => error.set(Some(format!("Failed to load profile: {}", e))),
                }

                // Then fetch map
                if let Ok(m) = provider.fetch_id_map(&edition).await {
                    id_map.set(Some(m));
                }
            });
            || ()
        });
    }

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
            if let Some(err) = &*error {
                <div class="bg-red-900 border border-red-700 text-red-200 p-4 rounded-lg mb-6">
                    <h3 class="font-bold flex items-center gap-2">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" /></svg>
                        { "Error Loading Profile" }
                    </h3>
                    <p class="mt-1 opacity-90">{ err }</p>
                </div>
            }

            if let Some(p) = &*profile {
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
                            { &*p.uuid.as_str() }
                        </p>
                    </div>
                </div>

                // --- Stats Grid ---
                if let Some(map) = &*id_map {
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
                } else {
                    // Loading State
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 animate-pulse">
                        <div class="h-64 bg-gray-800 rounded-lg"></div>
                        <div class="h-64 bg-gray-800 rounded-lg"></div>
                        <div class="h-64 bg-gray-800 rounded-lg"></div>
                    </div>
                }
            } else {
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
