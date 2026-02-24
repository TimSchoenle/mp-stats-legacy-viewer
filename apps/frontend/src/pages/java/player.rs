use crate::Route;
use crate::components::error_message::ErrorMessage;
use crate::hooks::{use_player_profile, use_theme};
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

    let theme_color = use_theme();

    html! {
        <div class={classes!(theme_color, "container", "mx-auto", "px-4", "py-8", "text-gray-200", "relative")}>
            // Breadcrumb Navigation
            <div class="flex items-center text-sm text-gray-400 mb-4 space-x-2">
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
                <div class="glass-panel p-6 md:p-8 mb-8 flex flex-col md:flex-row items-center gap-6 md:gap-8">
                    // Avatar
                    <div class="flex-shrink-0">
                        <img
                            src={format!("https://mc-heads.net/avatar/{}/128", p.uuid)}
                            class="w-24 h-24 md:w-32 md:h-32 rounded-2xl shadow-xl bg-dark-950 border border-white/10"
                            alt={p.name.as_ref().map(|s| s.as_str()).unwrap_or("Player").to_string()}
                        />
                    </div>

                    // Info
                    <div class="text-center md:text-left flex-1">
                        <div class="flex flex-col md:flex-row md:items-center gap-4 mb-4">
                            <h1 class="text-4xl font-bold text-white tracking-tight">
                                { p.name.as_ref().map(|s| s.as_str()).unwrap_or("Unknown") }
                            </h1>
                            <div class="flex justify-center md:justify-start">
                                <span class={classes!("px-3", "py-1", "rounded-md", "bg-theme-500/20", "text-theme-400", "text-sm", "font-semibold", "border", "border-theme-500/30")}>
                                    {props.edition.display_name()}
                                </span>
                            </div>
                        </div>
                        <p class="font-mono text-gray-400 bg-dark-950 px-4 py-2 rounded-lg inline-block text-sm border border-white/10 select-all shadow-inner">
                            { p.uuid.as_str() }
                        </p>
                    </div>
                </div>

                // --- Stats Grid ---
                if let Some(map) = &profile_req.id_map {
                    <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
                        {
                            // Group stats by game
                            {
                                use std::collections::BTreeMap;
                                let mut games: BTreeMap<String, Vec<&crate::models::StatRaw>> = BTreeMap::new();

                                for stat in &p.stats {
                                    let game_name = map.games.get(&stat.game_id).map(|s| s.name.as_str()).unwrap_or("Unknown Game").to_string();
                                    games.entry(game_name).or_default().push(stat);
                                }

                                games.into_iter().map(|(game_name, stats)| {
                                    html! {
                                        <div class="card p-6 flex flex-col h-full bg-dark-850 hover:bg-dark-800 transition-colors">
                                            <Link<Route>
                                                to={Route::Game { edition: props.edition.clone(), game: game_name.clone() }}
                                                classes={classes!("text-xl", "font-bold", "mb-4", "pb-3", "border-b", "border-white/10", "text-theme-400", "hover:text-theme-300", "transition-colors", "flex", "items-center", "justify-between", "group/gamelink")}
                                            >
                                                <span>{ game_name }</span>
                                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 opacity-0 group-hover/gamelink:opacity-100 transition-opacity transform group-hover/gamelink:translate-x-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                                </svg>
                                            </Link<Route>>

                                            <div class="space-y-1 flex-1">
                                                { for stats.iter().map(|s| {
                                                    let board_name = map.boards.get(&s.board_id).map(|s| s.name.as_str()).unwrap_or("Board");
                                                    let stat_name = map.stats.get(&s.stat_id).map(|s| s.name.as_str()).unwrap_or("Stat");

                                                    // Clean up names (remove redundancy if needed)
                                                    let label = if board_name == "All" {
                                                        stat_name.to_string()
                                                    } else {
                                                        format!("{} ({})", stat_name, board_name)
                                                    };

                                                    html! {
                                                        <div class="stat-row py-3">
                                                            <Link<Route> to={Route::Leaderboard { edition: props.edition.clone(), game: s.game_id.to_string(), board: s.board_id.to_string(), stat: s.stat_id.to_string(), page: 1 }} classes="font-medium text-gray-300 group-hover:text-white transition-colors">{label}</Link<Route>>
                                                            <div class="flex items-center gap-3">
                                                                <div class="font-mono font-bold text-gray-200 bg-dark-900 px-2 py-0.5 rounded shadow-inner border border-white/5">{ s.score }</div>
                                                                if s.rank > 0 {
                                                                    <span class={classes!("rank-badge", "group-hover:border-theme-500/30", "transition-colors")}>{ format!("#{}", s.rank) }</span>
                                                                }
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
                    <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6 animate-pulse">
                        <div class="h-64 bg-dark-850 rounded-2xl border border-white/5"></div>
                        <div class="h-64 bg-dark-850 rounded-2xl border border-white/5"></div>
                        <div class="h-64 bg-dark-850 rounded-2xl border border-white/5"></div>
                    </div>
                }
            } else if profile_req.loading {
                 <div class="card p-16 flex justify-center items-center h-64 border border-white/5">
                    <div class={classes!("flex", "flex-col", "items-center", "gap-4", "text-theme-500")}>
                        <div class={classes!("animate-spin", "h-8", "w-8", "border-2", "border-theme-500", "border-t-theme-200", "rounded-full", "shadow-lg", "border-t-white/20")}></div>
                        <p class={classes!("font-medium", "text-theme-400")}>{ "Loading Profile..." }</p>
                    </div>
                </div>
            }
        </div>
    }
}
