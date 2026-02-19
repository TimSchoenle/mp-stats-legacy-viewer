use mp_stats_core::models::PlatformEdition;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/:edition")]
    Landing {
        edition: PlatformEdition
    },
    #[at("/:edition/game/:game")]
    Game {
        edition: PlatformEdition,
        game: String,
    },
    #[at("/:edition/leaderboard/:game/:board/:stat/:page")]
    Leaderboard {
        edition: PlatformEdition,
        game: String,
        board: String,
        stat: String,
        page: u32,
    },
    #[at("/:edition/player/:uuid")]
    Player {
        edition: PlatformEdition,
        uuid: String,
    },
    #[not_found]
    #[at("/404")]
    NotFound,
}
