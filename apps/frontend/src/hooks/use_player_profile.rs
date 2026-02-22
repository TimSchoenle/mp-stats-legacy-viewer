use crate::Api;
use mp_stats_core::models::{IdMap, JavaPlayerProfile, PlatformEdition};
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UsePlayerProfileResult {
    pub profile: Option<JavaPlayerProfile>,
    pub id_map: Option<IdMap>,
    pub loading: bool,
    pub error: Option<String>,
}

#[hook]
pub fn use_player_profile(edition: PlatformEdition, uuid: String) -> UsePlayerProfileResult {
    let profile = use_state(|| None::<JavaPlayerProfile>);
    let id_map = use_state(|| None::<IdMap>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    let context = use_context::<Api>().expect("no api context found");

    {
        let profile = profile.clone();
        let id_map = id_map.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((uuid, context), move |(id, ctx)| {
            error.set(None);
            loading.set(true);

            let id = id.clone();
            let provider = ctx.clone();

            spawn_local(async move {
                // Fetch profile first
                let p_res = provider.fetch_player(&edition, &id).await;
                match p_res {
                    Ok(p) => profile.set(Some(p)),
                    Err(e) => {
                        error.set(Some(format!("Failed to load profile: {}", e)));
                        loading.set(false);
                    }
                }

                // If no error with profile, fetch map
                if error.is_none() {
                    if let Ok(m) = provider.fetch_id_map(&edition).await {
                        id_map.set(Some(m));
                    }
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
    }
}
