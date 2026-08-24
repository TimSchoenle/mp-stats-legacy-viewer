//! The URL of every page, as the one enum the router and every link are built from.

use mp_stats_core::models::PlatformEdition;
use yew_router::prelude::*;

/// Every page of the site, and the URL it is reached at.
///
/// The segments are the same strings the converted tree uses for its directories, so a route is
/// most of the path to the files that answer it. Nothing validates them here: a game or a board
/// that does not exist routes fine and fails at the fetch.
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    /// The edition chooser, which is what a visitor lands on.
    #[at("/")]
    Home,
    /// One edition's game list.
    #[at("/:edition")]
    Landing {
        /// Which platform's statistics to list.
        edition: PlatformEdition,
    },
    /// One game's categories, with each board's leader beside it.
    #[at("/:edition/game/:game")]
    Game {
        /// Which platform the game belongs to.
        edition: PlatformEdition,
        /// The game's directory in the tree.
        game: String,
    },
    /// One page of one board of one category.
    #[at("/:edition/leaderboard/:game/:board/:stat/:page")]
    Leaderboard {
        /// Which platform the board belongs to.
        edition: PlatformEdition,
        /// The game's directory in the tree.
        game: String,
        /// The board's directory, such as the all-time board or one of the periodic ones.
        board: String,
        /// The category's directory, which may contain spaces.
        stat: String,
        /// 1-based page number, which the client turns into the zero-based chunk it fetches.
        page: u32,
    },
    /// One player's profile. The snapshot being viewed rides in the query string rather than here.
    #[at("/:edition/player/:uuid")]
    Player {
        /// Which platform the profile belongs to.
        edition: PlatformEdition,
        /// The player's UUID, or on Bedrock the name standing in for one.
        uuid: String,
    },
    /// Anything else.
    #[not_found]
    #[at("/404")]
    NotFound,
}
