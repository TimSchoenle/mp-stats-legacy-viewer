//! The game page's data: one game's boards, and the names its ids resolve to.

use crate::Api;
use crate::models::{GameLeaderboardData, IdMap};
use mp_stats_core::models::PlatformEdition;
use yew::platform::spawn_local;
use yew::prelude::*;

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
/// Refetches whenever `game_id` changes; `edition` is captured once, which is correct only because
/// no route changes the edition without also changing the game.
#[hook]
pub fn use_game_leaderboards(
    edition: PlatformEdition,
    game_id: String,
) -> UseGameLeaderboardsResult {
    let data = use_state(|| None::<GameLeaderboardData>);
    let id_map = use_state(|| None::<IdMap>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    let api_ctx = use_context::<Api>().expect("no api context found");

    {
        let data = data.clone();
        let id_map = id_map.clone();
        let loading = loading.clone();
        let error = error.clone();
        let edition = edition.clone();

        use_effect_with((game_id.clone(), api_ctx.clone()), move |(game, ctx)| {
            error.set(None);

            let game = game.clone();
            let provider = ctx.clone();
            loading.set(true);

            spawn_local(async move {
                let data_fetch = provider.fetch_game_leaderboards(&edition, &game);
                let map_fetch = provider.fetch_id_map(&edition);

                let (data_res, map_res) = futures::future::join(data_fetch, map_fetch).await;

                match data_res {
                    Ok(fetched_data) => {
                        data.set(Some(fetched_data));
                        if let Ok(m) = map_res {
                            id_map.set(Some(m));
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load game data: {}", e)));
                    }
                }
                loading.set(false);
            });

            || ()
        });
    }

    UseGameLeaderboardsResult {
        data: (*data).clone(),
        id_map: (*id_map).clone(),
        loading: *loading,
        error: (*error).clone(),
    }
}
