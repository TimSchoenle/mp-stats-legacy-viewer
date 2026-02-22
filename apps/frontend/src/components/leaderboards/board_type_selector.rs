use crate::Route;
use crate::pages::java::leaderboard::SnapshotQuery;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BoardTypeSelectorProps {
    pub edition: PlatformEdition,
    pub game: String,
    pub stat: String,
    pub current_board: String,
    pub boards: Vec<String>,
}

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

#[function_component(BoardTypeSelector)]
pub fn board_type_selector(props: &BoardTypeSelectorProps) -> Html {
    let sorted_boards = sorted_board_types(props.boards.clone());

    html! {
        <div class="flex gap-1 mb-6 bg-gray-800 rounded-lg p-1 w-fit">
            { for sorted_boards.iter().map(|board| {
                let is_active = *board == props.current_board;
                let classes = if is_active {
                    "px-4 py-2 rounded-md text-sm font-bold bg-emerald-600 text-white transition-all"
                } else {
                    "px-4 py-2 rounded-md text-sm font-medium text-gray-400 hover:text-white hover:bg-gray-700 transition-all cursor-pointer"
                };

                let route = Route::Leaderboard {
                    edition: props.edition.clone(),
                    game: props.game.clone(),
                    board: board.to_string(),
                    stat: props.stat.clone(),
                    page: 1, // Reset to page 1 on board switch
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
