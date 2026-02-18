use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/java")]
    JavaLanding,
    #[at("/java/game/:game")]
    JavaGame { game: String },
    #[at("/java/leaderboard/:game/:board/:stat/:page")]
    JavaLeaderboard {
        game: String,
        board: String,
        stat: String,
        page: u32,
    },
    #[at("/java/player/:uuid")]
    JavaPlayer { uuid: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
