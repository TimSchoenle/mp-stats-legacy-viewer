//! The all-time and periodic board tabs.

use crate::Route;
use crate::hooks::use_theme;
use crate::pages::java::leaderboard::SnapshotQuery;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

/// Which boards exist for this category, and which one is open.
#[derive(Properties, PartialEq, Clone)]
pub struct BoardTypeSelectorProps {
    /// The edition every link stays inside.
    pub edition: PlatformEdition,
    /// The game every link stays inside.
    pub game: String,
    /// The category every link stays inside.
    pub stat: String,
    /// The board currently being viewed, matched against `boards` by exact string.
    pub current_board: String,
    /// The boards to offer, in any order.
    pub boards: Vec<String>,
}

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
#[function_component(BoardTypeSelector)]
pub fn board_type_selector(props: &BoardTypeSelectorProps) -> Html {
    let sorted_boards = sorted_board_types(props.boards.clone());
    let theme_color = use_theme();

    html! {
        <div class={classes!(theme_color, "inline-flex", "items-center", "gap-1", "p-1", "bg-ink-2", "border", "border-rule", "rounded-md")}>
            { for sorted_boards.iter().map(|board| {
                let is_active = *board == props.current_board;
                let classes = if is_active {
                    classes!(
                        "px-3", "py-1.5", "rounded", "text-xs", "font-medium",
                        "bg-ink-3", "text-theme-400", "border", "border-theme-500/40",
                        "font-mono", "tracking-wide"
                    )
                } else {
                    classes!(
                        "px-3", "py-1.5", "rounded", "text-xs", "font-medium",
                        "text-paper-3", "hover:text-paper-1", "hover:bg-ink-3",
                        "transition-colors", "cursor-pointer", "font-mono", "tracking-wide",
                        "border", "border-transparent"
                    )
                };

                let route = Route::Leaderboard {
                    edition: props.edition.clone(),
                    game: props.game.clone(),
                    board: board.to_string(),
                    stat: props.stat.clone(),
                    page: 1,
                };

                html! {
                    <Link<Route, SnapshotQuery>
                        to={route}
                        classes={classes}
                    >
                        { board.to_string() }
                    </Link<Route, SnapshotQuery>>
                }
            }) }
        </div>
    }
}
