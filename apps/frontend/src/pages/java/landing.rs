//! One edition's game list.

use crate::hooks::use_theme;
use crate::{Api, Route};
use dioxus::prelude::*;
use mp_stats_core::models::{Game, PlatformEdition};
use std::collections::BTreeMap;

/// The letter a game is filed under: its first, upper-cased, or `#` for anything that does not
/// start with a letter.
fn initial(game: &Game) -> char {
    match game.name.chars().next().map(|c| c.to_ascii_uppercase()) {
        Some(letter) if letter.is_ascii_alphabetic() => letter,
        _ => '#',
    }
}

/// One edition's games, grouped by first letter.
///
/// `edition` is the platform whose games are listed.
#[component]
pub fn EditionLanding(edition: PlatformEdition) -> Element {
    let api = use_context::<Api>();
    let theme_color = use_theme();

    // An edition with no game list and an edition whose game list failed to load look the same to
    // this page, and are rendered the same: the skeleton below. There is nothing a reader could do
    // with the difference, and the ID map this is built from is fetched again by every page they
    // could reach from here.
    let games_resource = use_resource(use_reactive!(|edition| {
        let api = api.clone();

        async move {
            api.fetch_meta(&edition)
                .await
                .map(|meta| meta.games)
                .unwrap_or_default()
        }
    }));

    let mut games = games_resource.value().cloned().unwrap_or_default();
    games.sort_by_key(|game| game.name.to_lowercase());

    let total_snapshots: u64 = games.iter().map(|game| game.total_snapshots).sum();
    let game_count = games.len();

    let mut by_letter: BTreeMap<char, Vec<Game>> = BTreeMap::new();
    for game in games {
        by_letter.entry(initial(&game)).or_default().push(game);
    }

    let has_non_alpha = by_letter.contains_key(&'#');

    rsx! {
        div {
            class: "{theme_color} container mx-auto px-6 py-8 max-w-6xl xl:max-w-7xl 2xl:max-w-[1600px]",

            // Crumbs
            div { class: "crumbs mb-5",
                Link { to: Route::Home {}, "Home" }
                span { class: "sep", "/" }
                span { class: "here", {edition.display_name()} }
            }

            // Hero
            div {
                class: "flex flex-col md:flex-row justify-between items-start md:items-end gap-6 pb-7 border-b border-rule",
                div {
                    div { class: "eyebrow mb-3", "Edition \u{b7} {edition.display_name()}" }
                    h1 { class: "serif page-title text-5xl md:text-6xl text-paper-1",
                        span { class: "text-theme-500", {edition.display_name()} }
                        " Edition"
                    }
                    p { class: "mt-3 text-sm text-paper-3 max-w-xl leading-relaxed",
                        span { class: "text-paper-1 font-medium", "{game_count} archived games" }
                        if total_snapshots > 0 {
                            " across "
                            span { class: "text-paper-1 font-medium", "{total_snapshots} snapshots" }
                        }
                        ". Browse historical leaderboards from snapshots collected 2021\u{2013}2023."
                    }
                }
            }

            if by_letter.is_empty() {
                // Loading skeleton
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 mt-8 animate-pulse",
                    for index in 0..9 {
                        div { key: "{index}", class: "h-14 bg-ink-2 rounded-lg border border-rule" }
                    }
                }
            } else {
                // Alphabet rail
                div { class: "flex gap-1 mt-7 pb-3 border-b border-rule overflow-x-auto",
                    for letter in 'A'..='Z' {
                        a {
                            key: "{letter}",
                            class: if by_letter.contains_key(&letter) {
                                "font-mono text-xs font-medium w-7 h-7 flex items-center justify-center rounded text-paper-2 hover:bg-ink-3 cursor-pointer"
                            } else {
                                "font-mono text-xs font-medium w-7 h-7 flex items-center justify-center rounded text-ink-4 cursor-default"
                            },
                            href: "#letter-{letter}",
                            "{letter}"
                        }
                    }
                    if has_non_alpha {
                        a {
                            class: "font-mono text-xs font-medium w-7 h-7 flex items-center justify-center rounded text-paper-2 hover:bg-ink-3 cursor-pointer",
                            href: "#letter-other",
                            "#"
                        }
                    }
                }

                // Grouped grid
                div { class: "mt-2",
                    for (letter , group) in by_letter {
                        div {
                            key: "{letter}",
                            id: if letter == '#' { "letter-other".to_string() } else { format!("letter-{letter}") },
                            class: "grid grid-cols-[60px_1fr] gap-6 py-6 border-b border-rule-soft scroll-mt-20",
                            div { class: "serif text-5xl italic text-paper-4 leading-none", "{letter}" }
                            div {
                                class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-px bg-rule border border-rule rounded-lg overflow-hidden",
                                for game in group {
                                    Link {
                                        key: "{game.name}",
                                        to: Route::Game {
                                            edition: edition.clone(),
                                            game: game.name.to_string(),
                                        },
                                        class: "bg-ink-2 hover:bg-ink-3 transition-colors px-4 py-3 flex justify-between items-center group",
                                        div { class: "text-sm font-medium text-paper-1 truncate pr-2",
                                            "{game.name}"
                                        }
                                        span {
                                            class: "text-theme-500 text-sm opacity-70 group-hover:opacity-100 group-hover:translate-x-0.5 transition-all",
                                            "\u{2192}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
