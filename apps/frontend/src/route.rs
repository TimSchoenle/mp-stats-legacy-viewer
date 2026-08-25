//! The URL of every page, as the one enum the router and every link are built from.

use crate::app::AppShell;
use crate::pages::home::Home;
use crate::pages::java::{EditionLanding, GameView, LeaderboardView, PlayerView};
use crate::pages::not_found::NotFound;
use dioxus::prelude::*;
use mp_stats_core::models::PlatformEdition;
use std::fmt::Display;
use std::str::FromStr;

/// Every page of the site, and the URL it is reached at.
///
/// The segments are the same strings the converted tree uses for its directories, so a route is
/// most of the path to the files that answer it. Nothing validates them here: a game or a board
/// that does not exist routes fine and fails at the fetch.
///
/// Every variant sits inside [`AppShell`], so the header and footer are mounted once and survive
/// a navigation rather than being tacked onto each page. The component each variant renders is
/// named explicitly in its `#[route]` rather than being taken from the variant name, because the
/// names that read well as a URL are not the ones that read well as a component - and `Game`
/// would collide with [`mp_stats_core::models::Game`].
///
/// `NotFound` is a catch-all rather than a fixed path. It is reached both by a URL that matches
/// no shape at all and by one whose `:edition` is not an edition, because a dynamic segment that
/// fails to parse falls through to the next route rather than failing the match outright.
#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppShell)]
    /// The edition chooser, which is what a visitor lands on.
    #[route("/", Home)]
    Home {},
    /// One edition's game list.
    #[route("/:edition", EditionLanding)]
    Landing {
        /// Which platform's statistics to list.
        edition: PlatformEdition,
    },
    /// One game's categories, with each board's leader beside it.
    #[route("/:edition/game/:game", GameView)]
    Game {
        /// Which platform the game belongs to.
        edition: PlatformEdition,
        /// The game's directory in the tree.
        game: String,
    },
    /// One page of one board of one category, at one snapshot.
    #[route("/:edition/leaderboard/:game/:board/:stat/:page?:snapshot", LeaderboardView)]
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
        /// Which state of the board to read, in the query string rather than the path so that the
        /// board tabs and the page controls can rewrite their half of the URL without either
        /// having to know about the other.
        snapshot: Snapshot,
    },
    /// One player's profile.
    #[route("/:edition/player/:uuid", PlayerView)]
    Player {
        /// Which platform the profile belongs to.
        edition: PlatformEdition,
        /// The player's UUID, or on Bedrock the name standing in for one.
        uuid: String,
    },
    /// Anything else.
    #[route("/:..segments", NotFound)]
    NotFound {
        /// The path that matched nothing, which the page does not show but the router needs
        /// somewhere to put.
        segments: Vec<String>,
    },
}

/// Which state of a leaderboard is being read: the current standings, or one archived dump.
///
/// A type rather than a `String` compared against `"latest"` at each of the five places that ask.
/// The router needs [`FromStr`] and [`Display`] to move it in and out of `?snapshot=`, and the
/// [`Default`] is what an absent query means - which is what keeps a link written before this
/// query existed resolving to the same page it always did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Snapshot {
    /// The board as it stands in the most recent dump.
    #[default]
    Latest,
    /// One archived dump, by the id the tree files it under.
    Archived(String),
}

/// The spelling that goes in the URL, and that the tree's snapshot ids are compared against.
impl Snapshot {
    /// The snapshot id as the tree spells it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Latest => Self::LATEST,
            Self::Archived(id) => id,
        }
    }

    /// Whether this is the current board rather than an archived one, which decides which of the
    /// two fetch paths [`crate::api::Api`] offers is taken.
    #[must_use]
    pub fn is_latest(&self) -> bool {
        matches!(self, Self::Latest)
    }

    /// The id the tree itself uses for the current board, which is also what an absent query
    /// parses to.
    const LATEST: &'static str = "latest";
}

impl Display for Snapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Never fails. An empty query argument is what the router hands over for a URL that carries no
/// `?snapshot=` at all, and that means the same thing as asking for the latest.
impl FromStr for Snapshot {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "" | Self::LATEST => Self::Latest,
            id => Self::Archived(id.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parse a URL the way the router does when the browser hands it one.
    fn parse(url: &str) -> Route {
        url.parse::<Route>()
            .unwrap_or_else(|error| panic!("`{url}` should route: {error}"))
    }

    /// Every route, from both ends: the URL a link writes, and the value that URL parses back to.
    ///
    /// This is the site's whole URL grammar in one place, and the reason it is a test is that the
    /// `#[route]` strings are checked by the macro for shape but by nothing for meaning - a segment
    /// in the wrong order still compiles.
    #[test]
    fn every_route_survives_the_round_trip() {
        let cases = [
            (Route::Home {}, "/"),
            (
                Route::Landing {
                    edition: PlatformEdition::Java,
                },
                "/java",
            ),
            (
                Route::Game {
                    edition: PlatformEdition::Bedrock,
                    game: "Bedwars".to_string(),
                },
                "/bedrock/game/Bedwars",
            ),
            (
                Route::Player {
                    edition: PlatformEdition::Java,
                    uuid: "68b61e3c-4be0-4c0c-8897-6a8d3703fe9a".to_string(),
                },
                "/java/player/68b61e3c-4be0-4c0c-8897-6a8d3703fe9a",
            ),
            (
                Route::Leaderboard {
                    edition: PlatformEdition::Java,
                    game: "Bedwars".to_string(),
                    board: "All".to_string(),
                    stat: "wins".to_string(),
                    page: 3,
                    snapshot: Snapshot::Latest,
                },
                "/java/leaderboard/Bedwars/All/wins/3?snapshot=latest",
            ),
            (
                Route::Leaderboard {
                    edition: PlatformEdition::Bedrock,
                    game: "Bedwars".to_string(),
                    board: "monthly".to_string(),
                    stat: "wins".to_string(),
                    page: 1,
                    snapshot: Snapshot::Archived("2022-11-03".to_string()),
                },
                "/bedrock/leaderboard/Bedwars/monthly/wins/1?snapshot=2022-11-03",
            ),
        ];

        for (route, url) in cases {
            assert_eq!(route.to_string(), url, "{route:?} should render as {url}");
            assert_eq!(parse(url), route, "{url} should parse back");
        }
    }

    /// A category directory may contain spaces, so the segment has to be escaped on the way out
    /// and unescaped on the way back in. Written out rather than folded into the table above
    /// because it is the one case where the two spellings differ.
    #[test]
    fn a_category_with_a_space_is_escaped_in_the_path() {
        let route = Route::Leaderboard {
            edition: PlatformEdition::Java,
            game: "Global".to_string(),
            board: "All".to_string(),
            stat: "ingame time".to_string(),
            page: 1,
            snapshot: Snapshot::Latest,
        };

        assert_eq!(
            route.to_string(),
            "/java/leaderboard/Global/All/ingame%20time/1?snapshot=latest"
        );
        assert_eq!(parse(&route.to_string()), route);
    }

    /// An absent `?snapshot=` is the latest board, which is what every link written before that
    /// query existed looks like.
    #[test]
    fn a_leaderboard_url_without_a_query_reads_the_latest_board() {
        assert_eq!(
            parse("/java/leaderboard/Bedwars/All/wins/2"),
            Route::Leaderboard {
                edition: PlatformEdition::Java,
                game: "Bedwars".to_string(),
                board: "All".to_string(),
                stat: "wins".to_string(),
                page: 2,
                snapshot: Snapshot::Latest,
            }
        );
    }

    /// The two ways a URL reaches the 404 page: a shape no route has, and a shape some route has
    /// whose `:edition` is not an edition. The second is the one worth pinning - it works because
    /// a dynamic segment that fails to parse falls through to the next route rather than failing
    /// the match outright, which is a property of the router rather than of anything written here.
    #[test]
    fn anything_else_is_the_not_found_page() {
        for url in ["/404", "/not-an-edition", "/java/game", "/a/b/c/d/e/f/g"] {
            assert!(
                matches!(parse(url), Route::NotFound { .. }),
                "`{url}` should have routed to NotFound, got {:?}",
                parse(url)
            );
        }
    }

    #[test]
    fn an_absent_query_is_the_latest_snapshot() {
        assert_eq!("".parse::<Snapshot>().unwrap(), Snapshot::Latest);
        assert_eq!("latest".parse::<Snapshot>().unwrap(), Snapshot::Latest);
        assert!(Snapshot::default().is_latest());
    }

    /// The round trip the router relies on: what `Display` writes into the query has to parse
    /// back to the value it was written from.
    #[test]
    fn an_archived_id_survives_the_url() {
        let snapshot = Snapshot::Archived("2022-11-03".to_string());

        assert_eq!(snapshot.to_string(), "2022-11-03");
        assert_eq!(snapshot.to_string().parse::<Snapshot>().unwrap(), snapshot);
        assert!(!snapshot.is_latest());
    }

    /// `as_str` is what every comparison against a tree snapshot id goes through, including the
    /// one for the current board - the tree files that under `latest` too.
    #[test]
    fn the_latest_board_spells_itself_the_way_the_tree_does() {
        assert_eq!(Snapshot::Latest.as_str(), "latest");
        assert_eq!(Snapshot::Archived("abc".into()).as_str(), "abc");
    }
}
