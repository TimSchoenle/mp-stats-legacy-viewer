use crate::Route;
use crate::hooks::use_theme;
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
    let theme_color = use_theme();

    html! {
        <div class="flex gap-1 glass-panel p-1 w-fit shadow-inner bg-dark-900 border border-white/5">
            { for sorted_boards.iter().map(|board| {
                let is_active = *board == props.current_board;
                let classes = if is_active {
                    classes!(theme_color, "px-4", "py-2", "rounded-lg", "text-sm", "font-bold", "bg-theme-600", "shadow-md", "text-white", "transition-all", "transform", "hover:scale-105")
                } else {
                    classes!("px-4", "py-2", "rounded-lg", "text-sm", "font-medium", "text-gray-400", "hover:text-white", "hover:bg-white/5", "transition-all", "cursor-pointer")
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
                        classes={classes!(classes)}
                    >
                        { board.to_string() }
                    </Link<Route, SnapshotQuery>>
                }
            }) }
        </div>
    }
}
