use crate::Route;
use crate::components::error_message::ErrorMessage;
use crate::hooks::{use_game_leaderboards, use_theme};
use crate::util::score_formatter::create_score_formatter;
use mp_stats_core::models::{GLOBAL_BOARD, PlatformEdition, TopEntry};
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

    // Resolve the `#1 holder` of a category strictly from the latest snapshot of
    // the global board. Returns `None` (rendered as "—") when the global board or
    // its top entry is missing, so missing data is handled gracefully.
    let top_holder = |stat: &str| -> Option<TopEntry> {
        game_req.data.as_ref().and_then(|data| {
            data.stats
                .get(stat)
                .and_then(|boards| boards.get(GLOBAL_BOARD))
                .and_then(|meta| meta.top.clone())
        })
    };

    let theme_color = use_theme();

    html! {
        <div class={classes!(theme_color, "container", "mx-auto", "px-6", "py-8", "max-w-6xl", "xl:max-w-7xl", "2xl:max-w-[1600px]")}>
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
                            {
                                match game_req.data.as_ref().map(|d| d.total_snapshots).unwrap_or(0) {
                                    0 => format!("Game · {} categories", stats.len()),
                                    snapshots => format!("Game · {} categories · {} snapshots", stats.len(), snapshots),
                                }
                            }
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
                            { "Pick a category to view its leaderboard. Each board defaults to the latest snapshot." }
                        </p>
                    </div>
                </div>
            </div>

            if let Some(err) = &game_req.error {
                <div class="mt-6">
                    <ErrorMessage title="Error loading game data" message={err.clone()} />
                </div>
            } else if game_req.loading {
                <div class="mt-7 grid grid-cols-1 gap-px bg-rule border border-rule rounded-lg overflow-hidden animate-pulse">
                    { for (0..6).map(|_| html! {
                        <div class="h-14 bg-ink-2"></div>
                    }) }
                </div>
            } else if stats.is_empty() {
                <div class="mt-6 card p-12 text-center">
                    <p class="text-paper-3 text-sm">{ "No statistics found for this game." }</p>
                </div>
            } else {
                // Table: header + rows share one grid so columns self-balance and stay aligned
                <div class="mt-7 grid grid-cols-[40px_1fr_80px] md:grid-cols-[40px_1fr_minmax(160px,1fr)_minmax(120px,auto)_80px] border border-rule rounded-lg overflow-hidden">
                    // Eyebrow row (table-style header)
                    <div class="col-span-full grid grid-cols-subgrid gap-4 px-4 py-3 bg-ink-1 border-b border-rule">
                        <span class="eyebrow">{"#"}</span>
                        <span class="eyebrow">{"Category"}</span>
                        <span class="eyebrow hidden md:block">{"#1 holder (latest)"}</span>
                        <span class="eyebrow hidden md:block text-right">{"Top score"}</span>
                        <span class="eyebrow text-right">{""}</span>
                    </div>
                    { for stats.iter().enumerate().map(|(i, stat_name)| {
                        let game = props.game.clone();
                        let stat = stat_name.clone();
                        let stat_desc = if let Some(map) = &game_req.id_map {
                            map.stats.values().find(|v| v.name == stat.as_str())
                                .and_then(|v| v.description.clone())
                        } else {
                            None
                        };

                        let top = top_holder(stat_name);
                        let formatter = create_score_formatter(&game, &stat);

                        html! {
                            <Link<Route>
                                to={Route::Leaderboard { edition: props.edition.clone(), game, board: "All".to_string(), stat: stat.clone(), page: 1 }}
                                classes="col-span-full grid grid-cols-subgrid gap-4 items-center bg-ink-2 hover:bg-ink-3 transition-colors px-4 py-3.5 border-b border-rule-soft last:border-0 group"
                            >
                                <span class="font-mono text-xs text-paper-3">
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
                                        <p class="text-xs text-paper-3 mt-0.5 pl-4 line-clamp-1">
                                            { desc.as_str() }
                                        </p>
                                    }
                                </div>
                                // #1 holder (latest)
                                <div class="hidden md:flex items-center gap-2 min-w-0">
                                    if let Some(top) = &top {
                                        <img
                                            src={format!("https://mc-heads.net/avatar/{}/32", top.uuid)}
                                            class="w-[18px] h-[18px] rounded bg-ink-3 border border-rule shrink-0"
                                            alt="Avatar"
                                            loading="lazy"
                                        />
                                        <span class="font-mono text-xs text-paper-2 truncate">
                                            { top.name.as_str() }
                                        </span>
                                    } else {
                                        <span class="font-mono text-xs text-paper-4">{ "—" }</span>
                                    }
                                </div>
                                // Top score
                                <span class="hidden md:block text-right font-mono text-sm text-paper-1 tnum whitespace-nowrap">
                                    {
                                        if let Some(top) = &top {
                                            formatter.format_score(top.score)
                                        } else {
                                            "—".to_string()
                                        }
                                    }
                                </span>
                                <span class="text-right text-xs font-mono text-paper-3 group-hover:text-theme-400 transition-colors">
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
