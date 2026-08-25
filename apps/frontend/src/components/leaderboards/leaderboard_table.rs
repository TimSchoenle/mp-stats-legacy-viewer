//! The leaderboard itself.

use crate::Route;
use crate::hooks::use_theme;
use crate::models::LeaderboardEntry;
use crate::util::percent::format_percent;
use crate::util::score_formatter::create_score_formatter;
use dioxus::prelude::*;
use mp_stats_core::models::PlatformEdition;

/// One page of a board as a table: rank, player, a bar against the leader, and the score.
///
/// `game` and `stat` together decide how a score is written. `entries` are the rows in rank
/// order, and the first one's score is what the comparison bars are measured against - so a page
/// other than the first compares against the best on that page. `edition` is what every player
/// link points into.
#[component]
pub fn LeaderboardTable(
    game: String,
    stat: String,
    entries: Vec<LeaderboardEntry>,
    edition: PlatformEdition,
) -> Element {
    let theme_color = use_theme();
    let score_formatter = create_score_formatter(&game, &stat);

    // Top score for the "vs. #1" bar.
    let top_score = entries.first().map_or(0, |entry| entry.score);

    rsx! {
        div { class: "{theme_color} overflow-x-auto",
            table { class: "w-full text-left border-collapse",
                thead {
                    tr {
                        th { class: "table-header w-20", "Rank" }
                        th { class: "table-header", "Player" }
                        th { class: "table-header w-72 hidden md:table-cell", "vs. #1" }
                        th { class: "table-header text-right", "Score" }
                    }
                }
                tbody {
                    for row in entries.iter() {
                        {
                            let is_top3 = row.rank <= 3;
                            let pct = if top_score > 0 {
                                #[allow(clippy::cast_precision_loss)]
                                let ratio = row.score as f64 / top_score as f64;
                                (ratio * 100.0).clamp(0.0, 100.0)
                            } else {
                                0.0
                            };
                            let bar_color = if is_top3 {
                                "var(--color-theme-500)"
                            } else {
                                "var(--color-paper-3)"
                            };

                            rsx! {
                                tr { key: "{row.uuid}-{row.rank}", class: "table-row group",
                                    td { class: "table-cell",
                                        span {
                                            class: if is_top3 {
                                                "font-mono text-sm font-semibold text-theme-500"
                                            } else {
                                                "font-mono text-sm text-paper-3"
                                            },
                                            "#{row.rank}"
                                        }
                                    }
                                    td { class: "table-cell",
                                        Link {
                                            to: Route::Player {
                                                edition: edition.clone(),
                                                uuid: row.uuid.to_string(),
                                            },
                                            class: "flex items-center gap-3 w-fit group/link",
                                            img {
                                                src: "https://mc-heads.net/avatar/{row.uuid}/32",
                                                class: "w-6 h-6 rounded bg-ink-3 border border-rule",
                                                alt: "Avatar",
                                                loading: "lazy",
                                            }
                                            span {
                                                class: "font-mono text-sm font-medium text-paper-1 group-hover/link:text-theme-400 transition-colors",
                                                "{row.name}"
                                            }
                                        }
                                    }
                                    td { class: "table-cell hidden md:table-cell",
                                        div { class: "flex items-center gap-3 max-w-[16rem]",
                                            span { class: "bar-track flex-1",
                                                span {
                                                    class: "bar-fill",
                                                    style: "width:{pct:.2}%; background:{bar_color};",
                                                }
                                            }
                                            span {
                                                class: "font-mono text-xs text-paper-3 tnum w-10 text-right",
                                                {format_percent(pct)}
                                            }
                                        }
                                    }
                                    td {
                                        class: "table-cell text-right font-mono font-medium text-paper-1 tnum",
                                        {score_formatter.format_score(row.score)}
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
