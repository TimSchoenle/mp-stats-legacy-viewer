use crate::Route;
use crate::components::error_message::ErrorMessage;
use crate::hooks::{use_game_leaderboards, use_theme};
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
        <div class={classes!(theme_color, "container", "mx-auto", "px-6", "py-8", "max-w-6xl")}>
            // Crumbs
            <div class="crumbs mb-5">
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                <span class="sep">{"/"}</span>
                <Link<Route> to={Route::Landing { edition: props.edition.clone() }}>{ props.edition.display_name() }</Link<Route>>
                <span class="sep">{"/"}</span>
                <span class="here">{ &props.game }</span>
            </div>

            // Header
            <div class="flex flex-col md:flex-row justify-between items-start md:items-end gap-6 pb-7 border-b border-rule">
                <div class="flex items-start gap-5 min-w-0">
                    if let Some(data) = &game_req.data
                        && let Some(icon_url) = &data.icon
                    {
                        <div class="w-16 h-16 bg-ink-2 rounded-lg flex items-center justify-center shrink-0 border border-rule overflow-hidden">
                            <img src={icon_url.to_string()} class="w-10 h-10 object-contain" alt={props.game.clone()} />
                        </div>
                    }
                    <div class="min-w-0">
                        <div class="eyebrow mb-2">
                            { format!("Game · {} categories", stats.len()) }
                        </div>
                        <h1 class="serif page-title text-5xl md:text-6xl text-paper-1 break-words">
                            { &props.game }
                        </h1>
                        if let Some(data) = &game_req.data
                            && let Some(desc) = &data.description
                        {
                            <p class="mt-3 text-sm text-paper-3 max-w-2xl leading-relaxed">
                                { desc.as_str() }
                            </p>
                        }
                        <p class="mt-2 text-sm text-paper-4">
                            { "Pick a category to view its leaderboard." }
                        </p>
                    </div>
                </div>
            </div>

            if let Some(err) = &game_req.error {
                <div class="mt-6">
                    <ErrorMessage title="Error loading game data" message={err.clone()} />
                </div>
            } else if game_req.loading {
                <div class="mt-7 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-px bg-rule border border-rule rounded-lg overflow-hidden animate-pulse">
                    { for (0..6).map(|_| html! {
                        <div class="h-16 bg-ink-2"></div>
                    }) }
                </div>
            } else if stats.is_empty() {
                <div class="mt-6 card p-12 text-center">
                    <p class="text-paper-3 text-sm">{ "No statistics found for this game." }</p>
                </div>
            } else {
                // Eyebrow row (table-style header)
                <div class="mt-7 grid grid-cols-[40px_1fr_80px] gap-4 px-4 pb-3">
                    <span class="eyebrow">{"#"}</span>
                    <span class="eyebrow">{"Category"}</span>
                    <span class="eyebrow text-right">{""}</span>
                </div>
                <div class="grid grid-cols-1 gap-px bg-rule border border-rule rounded-lg overflow-hidden">
                    { for stats.iter().enumerate().map(|(i, stat_name)| {
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
                                classes="bg-ink-2 hover:bg-ink-3 transition-colors px-4 py-3.5 grid grid-cols-[40px_1fr_80px] gap-4 items-center group"
                            >
                                <span class="font-mono text-[11px] text-paper-4">
                                    { format!("{:02}", i + 1) }
                                </span>
                                <div class="min-w-0">
                                    <div class="flex items-center gap-2.5">
                                        <span class="w-1.5 h-1.5 rounded-full bg-theme-500"></span>
                                        <span class="text-base font-medium text-paper-1 capitalize">
                                            { stat_name.replace("_", " ") }
                                        </span>
                                    </div>
                                    if let Some(desc) = stat_desc {
                                        <p class="text-xs text-paper-4 mt-0.5 pl-4 line-clamp-1">
                                            { desc.as_str() }
                                        </p>
                                    }
                                </div>
                                <span class="text-right text-xs font-mono text-paper-4 group-hover:text-theme-400 transition-colors">
                                    { "view →" }
                                </span>
                            </Link<Route>>
                        }
                    }) }
                </div>
            }
        </div>
    }
}
