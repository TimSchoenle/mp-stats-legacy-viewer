use crate::Api;
use mp_stats_core::models::{IdMap, PlatformEdition, PlayerProfile};
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct UsePlayerProfileResult {
    pub profile: Option<PlayerProfile>,
    pub id_map: Option<IdMap>,
    pub loading: bool,
    pub error: Option<String>,
    /// True when the player simply has no profile data (i.e. they were not
    /// present in the latest page of any game), as opposed to a genuine
    /// fetch/network error. This drives a dedicated empty state in the UI.
    pub not_found: bool,
}

#[hook]
pub fn use_player_profile(edition: PlatformEdition, uuid: String) -> UsePlayerProfileResult {
    let profile = use_state(|| None::<PlayerProfile>);
    let id_map = use_state(|| None::<IdMap>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let not_found = use_state(|| false);

    let context = use_context::<Api>().expect("no api context found");

    {
        let edition = edition.clone();
        let profile = profile.clone();
        let id_map = id_map.clone();
        let loading = loading.clone();
        let error = error.clone();
        let not_found = not_found.clone();

        use_effect_with((edition, uuid, context), move |(edition, id, ctx)| {
            profile.set(None);
            id_map.set(None);
            error.set(None);
            not_found.set(false);
            loading.set(true);

            let edition = edition.clone();
            let id = id.clone();
            let provider = ctx.clone();

            spawn_local(async move {
                // Fetch profile first
                let p_res = provider.fetch_player(&edition, &id).await;
                match p_res {
                    Ok(p) => profile.set(Some(p)),
                    Err(e) => {
                        // A missing player (not present in any leaderboard shard) is an
                        // expected, benign outcome rather than a real error: surface it
                        // through `not_found` so the UI can explain it gracefully.
                        let msg = e.to_string();
                        if msg.contains("not found") || msg.contains("Invalid UUID") {
                            not_found.set(true);
                        } else {
                            error.set(Some(format!("Failed to load profile: {}", e)));
                        }
                        loading.set(false);
                    }
                }

                // If no error with profile, fetch map
                if error.is_none()
                    && let Ok(m) = provider.fetch_id_map(&edition).await
                {
                    id_map.set(Some(m));
                }

                loading.set(false);
            });
            || ()
        });
    }

    UsePlayerProfileResult {
        profile: (*profile).clone(),
        id_map: (*id_map).clone(),
        loading: *loading,
        error: (*error).clone(),
        not_found: *not_found,
    }
}
