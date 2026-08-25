//! Which of the three colour themes the page is drawn in.
//!
//! It follows the edition rather than a stored preference, so the two platforms stay
//! distinguishable at a glance and nothing has to be persisted.

use crate::Route;
use dioxus::prelude::*;
use mp_stats_core::models::PlatformEdition;

/// The theme class for the route being viewed, which is the edition's where a route names one.
///
/// Reading the route here is what subscribes the calling component to it, so a component that
/// wears a theme class repaints on navigation without being handed the edition as a prop.
#[must_use]
pub fn use_theme() -> &'static str {
    match use_route::<Route>() {
        Route::Landing { edition }
        | Route::Game { edition, .. }
        | Route::Leaderboard { edition, .. }
        | Route::Player { edition, .. } => get_theme_color(&edition),
        Route::Home {} | Route::NotFound { .. } => "theme-olive",
    }
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
