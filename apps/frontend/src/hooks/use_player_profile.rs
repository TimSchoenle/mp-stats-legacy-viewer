//! The profile page's data, and the distinction between a player who is missing and a fetch that
//! failed.

use crate::Api;
use dioxus::prelude::*;
use mp_stats_core::models::{IdMap, PlatformEdition, PlayerProfile};

/// What a profile page has to render from.
#[derive(Clone, PartialEq, Debug)]
pub struct UsePlayerProfileResult {
    /// The player's stats, absent until the fetch lands and after it fails.
    pub profile: Option<PlayerProfile>,
    /// The name tables the profile's numeric ids resolve through.
    pub id_map: Option<IdMap>,
    /// A fetch is in flight.
    pub loading: bool,
    /// Why the profile could not be loaded. Not set for a player who has no profile.
    pub error: Option<String>,
    /// The player placed in nothing, or the UUID was not a UUID. Separated from `error` because
    /// it is the ordinary outcome of following a stale link and deserves a page rather than a
    /// failure banner.
    pub not_found: bool,
}

/// The two ways a profile fetch ends badly, which the page answers very differently.
#[derive(Clone, PartialEq, Debug)]
enum ProfileError {
    /// No such player in the archive, or a UUID that was never one. An ordinary answer.
    NotFound,
    /// The fetch itself failed.
    Failed(String),
}

/// Fetches one player's profile and then the name tables its ids resolve through.
///
/// The two are sequential rather than concurrent: a profile that is not there makes the second
/// fetch pointless.
#[must_use]
pub fn use_player_profile(edition: PlatformEdition, uuid: String) -> UsePlayerProfileResult {
    let api = use_context::<Api>();

    let resource = use_resource(use_reactive!(|edition, uuid| {
        let api = api.clone();

        async move {
            match api.fetch_player(&edition, &uuid).await {
                // The name tables are optional: a profile renders with numeric ids where names
                // would be rather than not rendering at all.
                Ok(profile) => Ok((profile, api.fetch_id_map(&edition).await.ok())),
                Err(error) => {
                    let message = error.to_string();

                    Err(
                        if message.contains("not found") || message.contains("Invalid UUID") {
                            ProfileError::NotFound
                        } else {
                            ProfileError::Failed(format!("Failed to load profile: {error}"))
                        },
                    )
                }
            }
        }
    }));

    let loading = matches!(resource.state().cloned(), UseResourceState::Pending);

    match resource.value().cloned() {
        None => UsePlayerProfileResult {
            profile: None,
            id_map: None,
            loading,
            error: None,
            not_found: false,
        },
        Some(Ok((profile, id_map))) => UsePlayerProfileResult {
            profile: Some(profile),
            id_map,
            loading,
            error: None,
            not_found: false,
        },
        // Both outcomes are withdrawn while a new fetch is in flight, so navigating from one
        // missing player to another does not flash the previous verdict over the spinner.
        Some(Err(ProfileError::NotFound)) => UsePlayerProfileResult {
            profile: None,
            id_map: None,
            loading,
            error: None,
            not_found: !loading,
        },
        Some(Err(ProfileError::Failed(error))) => UsePlayerProfileResult {
            profile: None,
            id_map: None,
            loading,
            error: (!loading).then_some(error),
            not_found: false,
        },
    }
}
