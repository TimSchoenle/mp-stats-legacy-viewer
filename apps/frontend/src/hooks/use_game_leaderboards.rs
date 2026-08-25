//! The game page's data: one game's boards, and the names its ids resolve to.

use crate::Api;
use crate::models::{GameLeaderboardData, IdMap};
use dioxus::prelude::*;
use mp_stats_core::models::PlatformEdition;

/// What a game page has to render from.
#[derive(Clone, PartialEq)]
pub struct UseGameLeaderboardsResult {
    /// The game's categories and boards, absent until the fetch lands and after it fails.
    pub data: Option<GameLeaderboardData>,
    /// The name tables. Absent while loading, and also when only that half of the pair failed —
    /// the page still renders, with numeric ids where names would be.
    pub id_map: Option<IdMap>,
    /// A fetch is in flight.
    pub loading: bool,
    /// Why the game's metadata could not be loaded.
    pub error: Option<String>,
}

/// Fetches a game's metadata and the edition's name tables together, and reports the three states
/// a page can be in.
///
/// Refetches whenever `edition` or `game_id` changes. Both are named in the dependency list, so
/// there is no route that can change the edition without the fetch noticing - which naming only
/// the game would allow.
#[must_use]
pub fn use_game_leaderboards(
    edition: PlatformEdition,
    game_id: String,
) -> UseGameLeaderboardsResult {
    let api = use_context::<Api>();

    // One resource rather than two, because the page has no use for half of the pair: a game
    // whose metadata failed has nothing to put the names on. The two fetches inside it still
    // run concurrently.
    let resource = use_resource(use_reactive!(|edition, game_id| {
        let api = api.clone();

        async move {
            let (data, id_map) = futures::future::join(
                api.fetch_game_leaderboards(&edition, &game_id),
                api.fetch_id_map(&edition),
            )
            .await;

            match data {
                Ok(data) => Ok((data, id_map.ok())),
                Err(error) => Err(format!("Failed to load game data: {error}")),
            }
        }
    }));

    // `state()` rather than `pending()`: the latter peeks, so a component that read it would not
    // repaint when a navigation restarts the fetch and the spinner would never come back.
    let loading = matches!(resource.state().cloned(), UseResourceState::Pending);

    match resource.value().cloned() {
        // Nothing has landed yet, or the last run failed and this one is still in flight. Either
        // way the page has no data and no failure to report.
        None => UseGameLeaderboardsResult {
            data: None,
            id_map: None,
            loading,
            error: None,
        },
        Some(Ok((data, id_map))) => UseGameLeaderboardsResult {
            data: Some(data),
            id_map,
            loading,
            error: None,
        },
        // A failure is withdrawn the moment a new fetch starts, rather than sitting over the page
        // until that fetch decides whether it agrees.
        Some(Err(error)) => UseGameLeaderboardsResult {
            data: None,
            id_map: None,
            loading,
            error: (!loading).then_some(error),
        },
    }
}
