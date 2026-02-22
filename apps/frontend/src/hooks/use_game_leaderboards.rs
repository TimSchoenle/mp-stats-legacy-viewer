use crate::Api;
use crate::models::GameLeaderboardData;
use mp_stats_core::models::PlatformEdition;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UseGameLeaderboardsResult {
    pub data: Option<GameLeaderboardData>,
    pub loading: bool,
    pub error: Option<String>,
}

#[hook]
pub fn use_game_leaderboards(
    edition: PlatformEdition,
    game_id: String,
) -> UseGameLeaderboardsResult {
    let data = use_state(|| None::<GameLeaderboardData>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    let api_ctx = use_context::<Api>().expect("no api context found");

    {
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let edition = edition.clone();

        use_effect_with((game_id.clone(), api_ctx.clone()), move |(game, ctx)| {
            error.set(None);

            let game = game.clone();
            let provider = ctx.clone();
            loading.set(true);

            spawn_local(async move {
                match provider.fetch_game_leaderboards(&edition, &game).await {
                    Ok(fetched_data) => {
                        data.set(Some(fetched_data));
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load game data: {}", e)));
                        loading.set(false);
                    }
                }
            });

            || ()
        });
    }

    UseGameLeaderboardsResult {
        data: (*data).clone(),
        loading: *loading,
        error: (*error).clone(),
    }
}
