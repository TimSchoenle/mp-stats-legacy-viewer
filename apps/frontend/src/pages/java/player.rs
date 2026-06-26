use crate::Route;
use crate::components::error_message::ErrorMessage;
use crate::hooks::{use_player_profile, use_theme};
use crate::util::score_formatter::create_score_formatter;
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
        <div class={classes!(theme_color, "container", "mx-auto", "px-6", "py-8", "max-w-6xl")}>
            // Crumbs
            <div class="crumbs mb-5">
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                <span class="sep">{"/"}</span>
                <Link<Route> to={Route::Landing { edition: props.edition.clone() }}>{ props.edition.display_name() }</Link<Route>>
                <span class="sep">{"/"}</span>
                <span class="here">{"Player"}</span>
            </div>

            if let Some(err) = &profile_req.error {
                <ErrorMessage title="Error loading profile" message={err.clone()} is_banner={true} />
            }

            if let Some(p) = &profile_req.profile {
                // ---- Header ----
                <div class="pb-7 border-b border-rule">
                    <div class="grid grid-cols-[80px_1fr] md:grid-cols-[120px_1fr] gap-6 items-center">
                        <img
                            src={format!("https://mc-heads.net/avatar/{}/240", p.uuid)}
                            class="w-20 h-20 md:w-[120px] md:h-[120px] rounded-lg bg-ink-2 border border-rule"
                            alt={p.name.as_ref().map(|s| s.as_str()).unwrap_or("Player").to_string()}
                        />
                        <div class="min-w-0">
                            <div class="eyebrow mb-2">
                                { format!("Player profile · {} edition", props.edition.display_name()) }
                            </div>
                            <h1 class="serif page-title text-5xl md:text-6xl text-paper-1 break-words">
                                { p.name.as_ref().map(|s| s.as_str()).unwrap_or("Unknown") }
                            </h1>
                            <div class="flex flex-wrap gap-2 mt-4">
                                <span class="chip select-all">{ p.uuid.as_str() }</span>
                                {
                                    {
                                        let summary = p.summary();
                                        html! {
                                            <>
                                                if summary.best_rank > 0 {
                                                    <span class="chip chip-mint">{ format!("Best #{}", summary.best_rank) }</span>
                                                }
                                                if summary.top_ten > 0 {
                                                    <span class="chip">{ format!("{} in top 10", summary.top_ten) }</span>
                                                }
                                                <span class="chip">{ format!("{} categories", summary.total_categories) }</span>
                                            </>
                                        }
                                    }
                                }
                                if let Some(map) = &profile_req.id_map {
                                    {
                                        {
                                            let game_count = {
                                                use std::collections::BTreeSet;
                                                let mut games = BTreeSet::new();
                                                for stat in &p.stats {
                                                    if let Some(g) = map.games.get(&stat.game_id) {
                                                        games.insert(g.name.clone());
                                                    }
                                                }
                                                games.len()
                                            };
                                            html! { <span class="chip chip-mint">{ format!("● {game_count} games") }</span> }
                                        }
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </div>

                // ---- Stats grid (per-game cards) ----
                if let Some(map) = &profile_req.id_map {
                    <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4 mt-7">
                        {
                            {
                                use std::collections::BTreeMap;
                                let mut games: BTreeMap<String, Vec<&crate::models::StatRaw>> = BTreeMap::new();

                                for stat in &p.stats {
                                    let game_name = map.games.get(&stat.game_id).map(|s| s.name.as_str()).unwrap_or("Unknown Game").to_string();
                                    games.entry(game_name).or_default().push(stat);
                                }

                                games.into_iter().map(|(game_name, mut stats)| {
                                    // Sort stats by rank (best first)
                                    stats.sort_by_key(|s| if s.rank > 0 { s.rank as i64 } else { i64::MAX });

                                    // Determine max rank in this game card for relative bar widths
                                    let max_rank = stats.iter()
                                        .map(|s| if s.rank > 0 { s.rank } else { 0 })
                                        .max()
                                        .unwrap_or(0)
                                        .max(100);

                                    html! {
                                        <div class="card p-5">
                                            <div class="flex items-baseline justify-between pb-3 mb-3 border-b border-rule">
                                                <Link<Route>
                                                    to={Route::Game { edition: props.edition.clone(), game: game_name.clone() }}
                                                    classes="serif text-xl text-theme-500 hover:underline truncate pr-2"
                                                >
                                                    { game_name.clone() }
                                                </Link<Route>>
                                                <span class="font-mono text-[10px] text-paper-4 shrink-0">
                                                    { format!("{} {}", stats.len(), if stats.len() == 1 { "category" } else { "cats" }) }
                                                </span>
                                            </div>

                                            <div class="flex flex-col gap-1.5">
                                                { for stats.iter().map(|s| {
                                                    let board_name = map.boards.get(&s.board_id).map(|s| s.name.as_str()).unwrap_or("Board");
                                                    let stat_name = map.stats.get(&s.stat_id).map(|s| s.name.to_string()).unwrap_or(String::from("Stat"));

                                                    let score_formatter = create_score_formatter(&game_name, &stat_name);
                                                    let formatted_score = score_formatter.format_score(s.score);

                                                    let label = if board_name == "All" {
                                                        stat_name.to_string()
                                                    } else {
                                                        format!("{stat_name} ({board_name})")
                                                    };

                                                    let rank = s.rank as u32;
                                                    let is_top10 = rank > 0 && rank <= 10;
                                                    let fill = if rank > 0 {
                                                        (1.0 - (rank as f64 / max_rank as f64)).max(0.05)
                                                    } else { 0.0 };
                                                    let bar_color = if is_top10 { "var(--color-theme-500)" } else { "var(--color-paper-3)" };
                                                    let bar_style = format!("width:{:.1}%; background:{};", fill * 100.0, bar_color);

                                                    let rank_class = if is_top10 {
                                                        "font-mono tnum text-[11px] font-semibold text-theme-500 text-right w-12"
                                                    } else if rank > 0 {
                                                        "font-mono tnum text-[11px] text-paper-4 text-right w-12"
                                                    } else {
                                                        "font-mono tnum text-[11px] text-paper-4 text-right w-12"
                                                    };

                                                    html! {
                                                        <Link<Route>
                                                            to={Route::Leaderboard { edition: props.edition.clone(), game: game_name.clone(), board: board_name.to_string(), stat: stat_name.to_string(), page: 1 }}
                                                            classes="grid grid-cols-[1fr_60px_60px_44px] gap-2.5 items-center py-1.5 rounded hover:bg-ink-3 -mx-1 px-1 transition-colors"
                                                        >
                                                            <span class="text-xs text-paper-2 truncate">{ label }</span>
                                                            <span class="bar-track">
                                                                <span class="bar-fill" style={bar_style}></span>
                                                            </span>
                                                            <span class="font-mono tnum text-[11px] text-paper-1 text-right truncate">
                                                                { formatted_score }
                                                            </span>
                                                            <span class={rank_class}>
                                                                { if rank > 0 { format!("#{rank}") } else { "—".to_string() } }
                                                            </span>
                                                        </Link<Route>>
                                                    }
                                                }) }
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </div>
                } else if profile_req.loading {
                    <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4 mt-7 animate-pulse">
                        { for (0..3).map(|_| html! {
                            <div class="h-56 card"></div>
                        }) }
                    </div>
                }
            } else if profile_req.loading {
                <div class="card p-16 flex flex-col items-center justify-center gap-3 mt-6">
                    <div class={classes!("animate-spin", "h-5", "w-5", "border-2", "border-theme-500", "border-t-transparent", "rounded-full")}></div>
                    <p class="text-sm text-paper-3">{ "Loading profile…" }</p>
                </div>
            }
        </div>
    }
}
