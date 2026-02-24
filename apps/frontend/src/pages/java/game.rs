use crate::components::error_message::ErrorMessage;
use crate::hooks::{use_game_leaderboards, use_theme};
use crate::Route;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct GameProps {
    pub edition: PlatformEdition,
    pub game: String,
}

#[function_component(GameView)]
pub fn game_view(props: &GameProps) -> Html {
    let game_req = use_game_leaderboards(props.edition.clone(), props.game.clone());

    let mut stats = if let Some(data) = &game_req.data {
        data.stats.keys().map(|k| k.to_string()).collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    stats.sort_by_key(|a| a.to_lowercase());

    let theme_color = use_theme();

    html! {
        <div class={classes!(theme_color, "container", "mx-auto", "px-4", "py-8", "text-white", "relative")}>
            // Breadcrumb
            <div class="flex items-center text-sm text-gray-400 mb-4 space-x-2">
                <Link<Route> to={Route::Home} classes="hover:text-white transition">{"Home"}</Link<Route>>
                <span>{"/"}</span>
                <Link<Route> to={Route::Landing { edition: props.edition.clone() }} classes="hover:text-white transition">{props.edition.display_name()}</Link<Route>>
                <span>{"/"}</span>
                <span class="text-white">{ &props.game }</span>
            </div>


            // Header Container
            <div class="mb-10 flex flex-col md:flex-row items-start md:items-center gap-6">
                if let Some(data) = &game_req.data {
                    if let Some(icon_url) = &data.icon {
                        <div class="w-16 h-16 bg-dark-950 rounded-xl shadow-inner flex items-center justify-center shrink-0 border border-white/10 relative overflow-hidden group">
                            <img src={icon_url.to_string()} class="w-10 h-10 object-contain group-hover:scale-110 transition-transform" alt={props.game.clone()} />
                            <div class={classes!("absolute", "inset-0", "bg-theme-500/10", "opacity-0", "group-hover:opacity-100", "transition-opacity")}></div>
                        </div>
                    }
                }
                <div class="flex-1">
                    <h1 class="text-2xl md:text-5xl font-bold tracking-tight mb-2 flex flex-wrap items-center gap-3">
                        <span class="text-white">{ &props.game }</span>
                        <span class="text-2xl text-gray-500 font-light">{"—"}</span>
                        <span class="text-2xl text-gray-400 font-medium">{"Statistics"}</span>
                    </h1>
                    
                    if let Some(data) = &game_req.data && let Some(desc) = &data.description {
                        <p class="text-gray-300 text-lg leading-relaxed max-w-4xl">
                            { desc.as_str() }
                        </p>
                    }

          <p class="text-gray-400">
                            { "Select a statistic to view the leaderboard. Defaults to All Time." }
                        </p>
                </div>
            </div>

            if let Some(err) = &game_req.error {
                <ErrorMessage title="Error loading game data" message={err.clone()} />
            } else if game_req.loading {
                 <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 animate-pulse">
                    { for (0..8).map(|_| html! {
                        <div class="h-32 bg-dark-850 rounded-2xl border border-white/5"></div>
                    }) }
                </div>
            } else if stats.is_empty() {
                 <div class="card p-12 text-center text-gray-500">
                    <p>{"No statistics found for this game."}</p>
                </div>
            } else {
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 gap-4">
                    { for stats.iter().map(|stat_name| {
                        let game = props.game.clone();
                        let stat = stat_name.clone();
                        let stat_desc = if let Some(map) = &game_req.id_map {
                            map.stats.values().find(|v| v.name == stat.as_str())
                                .and_then(|v| v.description.clone())
                        } else {
                            None
                        };

                        html! {
                            <Link<Route>
                                to={Route::Leaderboard { edition: props.edition.clone(), game, board: "All".to_string(), stat: stat.clone(), page: 1 }}
                                classes={classes!("group", "card", "p-5", "hover:border-theme-500/50", "transition-all", "flex", "flex-col", "h-full", "bg-dark-850", "hover:bg-dark-800")}
                            >
                                <h3 class={classes!("font-bold", "text-lg", "group-hover:text-theme-400", "transition-colors", "capitalize", "text-white")}>
                                    { stat_name.replace("_", " ") }
                                </h3>
                                if let Some(desc) = stat_desc {
                                    <p class="text-sm text-gray-400 mt-2 line-clamp-2">
                                        { desc.as_str() }
                                    </p>
                                }
                                <div class={classes!("mt-auto", "pt-4", "flex", "items-center", "text-xs", "text-gray-400", "font-semibold", "group-hover:text-theme-400", "transition-colors")}>
                                    {"View Leaderboard"}
                                    <svg class="w-4 h-4 ml-1 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path></svg>
                                </div>
                            </Link<Route>>
                        }
                    }) }
                </div>
            }
        </div>
    }
}
