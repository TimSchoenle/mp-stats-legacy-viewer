use crate::hooks::use_theme;
use crate::{Api, Route};
use mp_stats_core::models::PlatformEdition;
use std::collections::BTreeMap;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct JavaLandingProps {
    pub edition: PlatformEdition,
}

#[function_component(JavaLanding)]
pub fn java_landing(props: &JavaLandingProps) -> Html {
    let games = use_state(Vec::new);
    let api_ctx = use_context::<Api>().expect("no api found found");

    {
        let games = games.clone();

        use_effect_with(
            (api_ctx, props.edition.clone()),
            move |(ctx, current_edition)| {
                let provider = ctx.clone();
                let edition_to_fetch = current_edition.clone();

                games.set(vec![]);

                spawn_local(async move {
                    if let Ok(meta) = provider.fetch_meta(&edition_to_fetch).await {
                        games.set(meta.games);
                    } else {
                        games.set(vec![]);
                    }
                });
                || ()
            },
        );
    }

    let sorted_games = {
        let mut games_vec = (*games).clone();
        games_vec.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        games_vec
    };

    // Group by first letter
    let mut by_letter: BTreeMap<char, Vec<_>> = BTreeMap::new();
    for game in &sorted_games {
        let ch = game
            .name
            .chars()
            .next()
            .map(|c| c.to_ascii_uppercase())
            .unwrap_or('#');
        let key = if ch.is_ascii_alphabetic() { ch } else { '#' };
        by_letter.entry(key).or_default().push(game.clone());
    }

    let theme_color = use_theme();
    let alphabet: Vec<char> = ('A'..='Z').collect();
    let has_non_alpha = by_letter.contains_key(&'#');

    html! {
        <div class={classes!(theme_color, "container", "mx-auto", "px-6", "py-8", "max-w-6xl")}>
            // Crumbs
            <div class="crumbs mb-5">
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                <span class="sep">{"/"}</span>
                <span class="here">{ props.edition.display_name() }</span>
            </div>

            // Hero
            <div class="flex flex-col md:flex-row justify-between items-start md:items-end gap-6 pb-7 border-b border-rule">
                <div>
                    <div class="eyebrow mb-3">{ format!("Edition · {}", props.edition.display_name()) }</div>
                    <h1 class="serif page-title text-5xl md:text-6xl text-paper-1">
                        <span class="text-theme-500">{ props.edition.display_name() }</span>
                        { " Edition" }
                    </h1>
                    <p class="mt-3 text-sm text-paper-3 max-w-xl leading-relaxed">
                        <span class="text-paper-1 font-medium">
                            { format!("{} archived games", games.len()) }
                        </span>
                        { ". Browse historical leaderboards from snapshots collected 2021–2023." }
                    </p>
                </div>
            </div>

            if games.is_empty() {
                // Loading skeleton
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 mt-8 animate-pulse">
                    { for (0..9).map(|_| html! {
                        <div class="h-14 bg-ink-2 rounded-lg border border-rule"></div>
                    }) }
                </div>
            } else {
                // Alphabet rail
                <div class="flex gap-1 mt-7 pb-3 border-b border-rule overflow-x-auto">
                    { for alphabet.iter().map(|l| {
                        let has = by_letter.contains_key(l);
                        let cls = if has {
                            "font-mono text-xs font-medium w-7 h-7 flex items-center justify-center rounded text-paper-2 hover:bg-ink-3 cursor-pointer"
                        } else {
                            "font-mono text-xs font-medium w-7 h-7 flex items-center justify-center rounded text-ink-4 cursor-default"
                        };
                        let href = format!("#letter-{l}");
                        html! { <a class={cls} href={href}>{ l.to_string() }</a> }
                    }) }
                    if has_non_alpha {
                        <a class="font-mono text-xs font-medium w-7 h-7 flex items-center justify-center rounded text-paper-2 hover:bg-ink-3 cursor-pointer" href="#letter-other">{"#"}</a>
                    }
                </div>

                // Grouped grid
                <div class="mt-2">
                    { for by_letter.iter().map(|(letter, games)| {
                        let anchor_id = if *letter == '#' { "letter-other".to_string() } else { format!("letter-{letter}") };
                        let label = if *letter == '#' { "#".to_string() } else { letter.to_string() };
                        let edition = props.edition.clone();

                        html! {
                            <div id={anchor_id} class="grid grid-cols-[60px_1fr] gap-6 py-6 border-b border-rule-soft scroll-mt-20">
                                <div class="serif text-5xl italic text-paper-4 leading-none">{ label }</div>
                                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-px bg-rule border border-rule rounded-lg overflow-hidden">
                                    { for games.iter().map(|game| {
                                        html! {
                                            <Link<Route>
                                                to={Route::Game { edition: edition.clone(), game: game.name.to_string() }}
                                                classes="bg-ink-2 hover:bg-ink-3 transition-colors px-4 py-3 flex justify-between items-center group"
                                            >
                                                <div class="text-sm font-medium text-paper-1 truncate pr-2">
                                                    { game.name.as_str() }
                                                </div>
                                                <span class="text-theme-500 text-xs opacity-50 group-hover:opacity-100 group-hover:translate-x-0.5 transition-all">
                                                    { "→" }
                                                </span>
                                            </Link<Route>>
                                        }
                                    }) }
                                </div>
                            </div>
                        }
                    }) }
                </div>
            }
        </div>
    }
}
