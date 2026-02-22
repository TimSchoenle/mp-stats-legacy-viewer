use crate::components::error_message::ErrorMessage;
use crate::hooks::use_game_leaderboards;
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

    html! {
        <div class="container mx-auto px-4 py-8 text-white">
            // Breadcrumb
            <div class="flex items-center text-sm text-gray-400 mb-2 space-x-2">
                <Link<Route> to={Route::Home} classes="hover:text-white transition">{"Home"}</Link<Route>>
                <span>{"/"}</span>
                <Link<Route> to={Route::Landing { edition: props.edition.clone() }} classes="hover:text-white transition">{props.edition.display_name()}</Link<Route>>
                <span>{"/"}</span>
                <span class="text-white">{ &props.game }</span>
            </div>

            // Header
            <div class="mb-8">
                <h1 class="text-3xl font-bold">
                    <span class="text-emerald-400">{ &props.game }</span>
                    <span class="text-gray-500 text-lg ml-3">{ "— Statistics" }</span>
                </h1>
                <p class="text-gray-400 mt-2">
                    {"Select a statistic to view the leaderboard. Defaults to All Time."}
                </p>
            </div>

            if let Some(err) = &game_req.error {
                <ErrorMessage title="Error loading game data" message={err.clone()} />
            } else if game_req.loading {
                 <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 animate-pulse">
                    { for (0..8).map(|_| html! {
                        <div class="h-20 bg-gray-800 rounded-lg"></div>
                    }) }
                </div>
            } else if stats.is_empty() {
                 <div class="card p-12 text-center text-gray-500">
                    <p>{"No statistics found for this game."}</p>
                </div>
            } else {
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3">
                    { for stats.iter().map(|stat_name| {
                        let game = props.game.clone();
                        let stat = stat_name.clone();
                        html! {
                            <Link<Route>
                                to={Route::Leaderboard { edition: props.edition.clone(), game, board: "All".to_string(), stat: stat.clone(), page: 1 }}
                                classes="group card p-4 hover:border-emerald-600 transition-all"
                            >
                                <h3 class="font-bold text-sm group-hover:text-emerald-400 transition-colors capitalize">
                                    { stat_name.replace("_", " ") }
                                </h3>
                                <p class="text-xs text-gray-500 mt-1 group-hover:text-gray-400">
                                    {"View leaderboard →"}
                                </p>
                            </Link<Route>>
                        }
                    }) }
                </div>
            }
        </div>
    }
}
