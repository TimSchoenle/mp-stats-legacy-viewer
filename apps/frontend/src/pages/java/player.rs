//! One player's profile.

use crate::Route;
use crate::components::error_message::ErrorMessage;
use crate::hooks::{use_player_profile, use_theme};
use crate::models::{IdMap, PlayerProfile, StatRaw};
use crate::route::Snapshot;
use crate::util::score_formatter::create_score_formatter;
use dioxus::prelude::*;
use mp_stats_core::models::PlatformEdition;
use std::collections::BTreeMap;

/// The rank a card's bars are measured against, so that a game where nobody placed better than
/// #400 still draws a readable spread rather than six full bars.
const MIN_BAR_SCALE: u32 = 100;

/// A player's standings grouped by the game they were set in, each group in rank order.
fn by_game(profile: &PlayerProfile, map: &IdMap) -> BTreeMap<String, Vec<StatRaw>> {
    let mut games: BTreeMap<String, Vec<StatRaw>> = BTreeMap::new();

    for stat in &profile.stats {
        let game_name = map
            .games
            .get(&stat.game_id)
            .map_or("Unknown Game", |game| game.name.as_str())
            .to_string();

        games.entry(game_name).or_default().push(stat.clone());
    }

    for stats in games.values_mut() {
        // Unranked entries sort last rather than first, which is where a rank of 0 would put them.
        stats.sort_by_key(|stat| {
            if stat.rank > 0 {
                i64::from(stat.rank)
            } else {
                i64::MAX
            }
        });
    }

    games
}

/// One player's standings, with the summary counted over them.
///
/// `edition` is the platform the profile belongs to, and `uuid` is the player's UUID - or on
/// Bedrock the name standing in for one.
#[component]
pub fn PlayerView(edition: PlatformEdition, uuid: String) -> Element {
    let profile_req = use_player_profile(edition.clone(), uuid.clone());
    let theme_color = use_theme();

    rsx! {
        div {
            class: "{theme_color} container mx-auto px-6 py-8 max-w-6xl xl:max-w-7xl 2xl:max-w-[1600px]",

            // Crumbs
            div { class: "crumbs mb-5",
                Link { to: Route::Home {}, "Home" }
                span { class: "sep", "/" }
                Link { to: Route::Landing { edition: edition.clone() }, {edition.display_name()} }
                span { class: "sep", "/" }
                span { class: "here", "Player" }
            }

            if let Some(error) = profile_req.error.clone() {
                ErrorMessage {
                    title: "Error loading profile",
                    message: error,
                    is_banner: true,
                }
            }

            if let Some(profile) = profile_req.profile.as_ref() {
                // Header
                div { class: "pb-7 border-b border-rule",
                    div {
                        class: "grid grid-cols-[80px_1fr] md:grid-cols-[120px_1fr] gap-6 items-center",
                        img {
                            src: "https://mc-heads.net/avatar/{profile.uuid}/240",
                            class: "w-20 h-20 md:w-[120px] md:h-[120px] rounded-lg bg-ink-2 border border-rule",
                            alt: profile.name.as_deref().unwrap_or("Player"),
                        }
                        div { class: "min-w-0",
                            div { class: "eyebrow mb-2",
                                "Player profile \u{b7} {edition.display_name()} edition"
                            }
                            h1 {
                                class: "serif page-title text-5xl md:text-6xl text-paper-1 break-words",
                                {profile.name.as_deref().unwrap_or("Unknown")}
                            }
                            div { class: "flex flex-wrap gap-2 mt-4",
                                span { class: "chip select-all", "{profile.uuid}" }
                            }
                        }
                    }
                }

                // Stats grid (per-game cards)
                if let Some(map) = profile_req.id_map.as_ref() {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4 mt-7",
                        for (game_name , stats) in by_game(profile, map) {
                            {
                                let category_count = stats.len();
                                let max_rank = stats
                                    .iter()
                                    .map(|stat| stat.rank)
                                    .max()
                                    .unwrap_or(0)
                                    .max(MIN_BAR_SCALE);

                                rsx! {
                                    div { key: "{game_name}", class: "card p-5",
                                        div {
                                            class: "flex items-baseline justify-between pb-3 mb-3 border-b border-rule",
                                            Link {
                                                to: Route::Game {
                                                    edition: edition.clone(),
                                                    game: game_name.clone(),
                                                },
                                                class: "serif text-xl text-theme-500 hover:underline truncate pr-2",
                                                "{game_name}"
                                            }
                                            span { class: "font-mono text-xs text-paper-3 shrink-0",
                                                if category_count == 1 {
                                                    "1 category"
                                                } else {
                                                    "{category_count} categories"
                                                }
                                            }
                                        }

                                        div {
                                            class: "grid grid-cols-[1fr_56px_minmax(56px,auto)_minmax(40px,auto)] gap-x-2.5 gap-y-0.5",
                                            for stat in stats {
                                                {
                                                    let board_name = map
                                                        .boards
                                                        .get(&stat.board_id)
                                                        .map_or("Board", |board| board.name.as_str())
                                                        .to_string();
                                                    let stat_name = map
                                                        .stats
                                                        .get(&stat.stat_id)
                                                        .map_or_else(|| "Stat".to_string(), |entry| entry.name.to_string());

                                                    let score = create_score_formatter(&game_name, &stat_name)
                                                        .format_score(stat.score);

                                                    let label = if board_name == "All" {
                                                        stat_name.clone()
                                                    } else {
                                                        format!("{stat_name} ({board_name})")
                                                    };

                                                    let rank = stat.rank;
                                                    let is_top10 = (1..=10).contains(&rank);

                                                    #[allow(clippy::cast_precision_loss)]
                                                    let fill = if rank > 0 {
                                                        (1.0 - f64::from(rank) / f64::from(max_rank)).max(0.05) * 100.0
                                                    } else {
                                                        0.0
                                                    };

                                                    let bar_color = if is_top10 {
                                                        "var(--color-theme-500)"
                                                    } else {
                                                        "var(--color-paper-3)"
                                                    };

                                                    rsx! {
                                                        Link {
                                                            key: "{stat.board_id}-{stat.stat_id}",
                                                            to: Route::Leaderboard {
                                                                edition: edition.clone(),
                                                                game: game_name.clone(),
                                                                board: board_name.clone(),
                                                                stat: stat_name.clone(),
                                                                page: 1,
                                                                snapshot: Snapshot::Latest,
                                                            },
                                                            class: "col-span-full grid grid-cols-subgrid gap-x-2.5 items-center py-1.5 rounded hover:bg-ink-3 -mx-1 px-1 transition-colors",
                                                            span { class: "text-xs text-paper-2 truncate", "{label}" }
                                                            span { class: "bar-track",
                                                                span {
                                                                    class: "bar-fill",
                                                                    style: "width:{fill:.1}%; background:{bar_color};",
                                                                }
                                                            }
                                                            span {
                                                                class: "font-mono tnum text-xs text-paper-1 text-right whitespace-nowrap",
                                                                "{score}"
                                                            }
                                                            span {
                                                                class: if is_top10 {
                                                                    "font-mono tnum text-xs font-semibold text-theme-500 text-right whitespace-nowrap"
                                                                } else if rank > 0 {
                                                                    "font-mono tnum text-xs text-paper-3 text-right whitespace-nowrap"
                                                                } else {
                                                                    "font-mono tnum text-xs text-paper-4 text-right whitespace-nowrap"
                                                                },
                                                                if rank > 0 {
                                                                    "#{rank}"
                                                                } else {
                                                                    "\u{2014}"
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
                    }
                } else if profile_req.loading {
                    div {
                        class: "grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4 mt-7 animate-pulse",
                        for index in 0..3 {
                            div { key: "{index}", class: "h-56 card" }
                        }
                    }
                }
            } else if profile_req.loading {
                div { class: "card p-16 flex flex-col items-center justify-center gap-3 mt-6",
                    div {
                        class: "animate-spin h-5 w-5 border-2 border-theme-500 border-t-transparent rounded-full",
                    }
                    p { class: "text-sm text-paper-3", "Loading profile\u{2026}" }
                }
            } else if profile_req.not_found {
                NoProfileData { edition: edition.clone(), uuid: uuid.clone() }
            }
        }
    }
}

/// What a profile page shows for a player the archive has nothing on.
///
/// A profile exists only for players the dumps caught on the all-time board, so a UUID that is
/// perfectly valid and belongs to a real player can still have no page. Saying so is the whole
/// point of this component: the alternative reads as a fault, and readers report it as one.
#[component]
fn NoProfileData(edition: PlatformEdition, uuid: String) -> Element {
    rsx! {
        div { class: "mt-6 flex flex-col items-center",
            div { class: "card w-full max-w-2xl p-8 md:p-12 text-center",

                // Ghost avatar
                div { class: "flex justify-center mb-6",
                    div { class: "relative",
                        div {
                            class: "w-24 h-24 md:w-28 md:h-28 rounded-lg bg-ink-2 border border-rule flex items-center justify-center text-5xl text-paper-4 opacity-70 select-none",
                            "?"
                        }
                        span { class: "absolute -bottom-2 -right-2 chip chip-rose text-[10px]",
                            "no data"
                        }
                    }
                }

                div { class: "eyebrow mb-3",
                    "Player profile \u{b7} {edition.display_name()} edition"
                }
                h1 {
                    class: "serif page-title text-4xl md:text-5xl text-paper-1 mb-4 break-words",
                    "No profile data found"
                }

                p { class: "text-sm text-paper-3 leading-relaxed max-w-lg mx-auto mb-6",
                    "We couldn't find any archived statistics for this player. That usually doesn't mean anything is broken \u{2014} it simply means this player was never captured by the archive."
                }

                // Explanation box: why profiles can be missing
                div { class: "rounded-lg border border-rule bg-ink-2 p-5 text-left mb-7",
                    div { class: "eyebrow mb-2", style: "color: var(--color-theme-500);",
                        "Why is this empty?"
                    }
                    p { class: "text-sm text-paper-2 leading-relaxed",
                        "A profile only exists if the player appeared inside the "
                        span { class: "text-paper-1 font-semibold", "latest page" }
                        " of a game's leaderboard when each snapshot was taken. Players ranked beyond that final page \u{2014} or who never placed on any board \u{2014} leave no record behind, so there is nothing to show here."
                    }
                }

                // The looked-up id, for reference
                div { class: "flex flex-wrap gap-2 justify-center mb-7",
                    span { class: "chip select-all", "{uuid}" }
                }

                // Actions
                div { class: "flex flex-wrap gap-3 justify-center",
                    Link {
                        to: Route::Landing { edition: edition.clone() },
                        class: "btn",
                        "\u{2190} Browse {edition.display_name()} games"
                    }
                    Link { to: Route::Home {}, class: "btn btn-ghost", "Return home" }
                }

                p { class: "text-xs text-paper-4 mt-6 leading-relaxed",
                    "Tip: double-check the spelling of the name or UUID \u{2014} even a small difference points to a different player."
                }
            }
        }
    }
}
