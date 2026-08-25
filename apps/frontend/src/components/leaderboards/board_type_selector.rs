//! The all-time and periodic board tabs.

use crate::Route;
use crate::hooks::use_theme;
use crate::route::Snapshot;
use dioxus::prelude::*;
use mp_stats_core::models::PlatformEdition;

/// Orders boards by window, longest first: all-time, yearly, monthly, weekly, daily, then anything
/// unrecognised alphabetically.
///
/// The tree stores them in a map, so without this the tabs would move between renders.
#[must_use]
pub fn sorted_board_types(mut boards: Vec<String>) -> Vec<String> {
    fn get_rank(board: &str) -> u8 {
        match board.to_lowercase().as_str() {
            "all" => 0,
            "yearly" => 1,
            "monthly" => 2,
            "weekly" => 3,
            "daily" => 4,
            _ => 5,
        }
    }

    boards.sort_by(|a, b| {
        let rank_a = get_rank(a);
        let rank_b = get_rank(b);

        rank_a.cmp(&rank_b).then_with(|| a.cmp(b))
    });

    boards
}

/// The row of board tabs. Each is a link, so a board is a URL and can be shared.
///
/// `edition`, `game` and `stat` are what every link stays inside, `current_board` is the one being
/// viewed and is matched against `boards` by exact string, and `boards` is what to offer, in any
/// order.
///
/// A tab drops the snapshot as well as the page: the boards of one category are archived
/// independently, so an id taken from the weekly board names nothing on the monthly one.
#[component]
pub fn BoardTypeSelector(
    edition: PlatformEdition,
    game: String,
    stat: String,
    current_board: String,
    boards: Vec<String>,
) -> Element {
    let theme_color = use_theme();

    rsx! {
        div {
            class: "{theme_color} inline-flex items-center gap-1 p-1 bg-ink-2 border border-rule rounded-md",
            for board in sorted_board_types(boards) {
                Link {
                    key: "{board}",
                    class: if board == current_board {
                        "px-3 py-1.5 rounded text-xs font-medium bg-ink-3 text-theme-400 border border-theme-500/40 font-mono tracking-wide"
                    } else {
                        "px-3 py-1.5 rounded text-xs font-medium text-paper-3 hover:text-paper-1 hover:bg-ink-3 transition-colors cursor-pointer font-mono tracking-wide border border-transparent"
                    },
                    to: Route::Leaderboard {
                        edition: edition.clone(),
                        game: game.clone(),
                        board: board.clone(),
                        stat: stat.clone(),
                        page: 1,
                        snapshot: Snapshot::Latest,
                    },
                    "{board}"
                }
            }
        }
    }
}
