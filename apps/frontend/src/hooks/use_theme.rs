//! Which of the three colour themes the page is drawn in.
//!
//! It follows the edition rather than a stored preference, so the two platforms stay
//! distinguishable at a glance and nothing has to be persisted.

use crate::Route;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

/// The theme class for the route being viewed, which is the edition's where a route names one.
#[hook]
pub fn use_theme() -> &'static str {
    let route_context = use_route::<Route>();

    let theme = use_memo(route_context, |route_ctx| {
        if let Some(
            Route::Landing { edition }
            | Route::Game { edition, .. }
            | Route::Leaderboard { edition, .. }
            | Route::Player { edition, .. },
        ) = route_ctx
        {
            get_theme_color(edition)
        } else {
            "theme-olive"
        }
    });

    *theme
}

/// The theme class an edition is drawn in.
#[must_use]
pub fn get_theme_color(edition: &PlatformEdition) -> &'static str {
    if *edition == PlatformEdition::Bedrock {
        "theme-bedrock"
    } else {
        "theme-java"
    }
}
