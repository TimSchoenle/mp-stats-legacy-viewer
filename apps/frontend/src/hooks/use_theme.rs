use crate::Route;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

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

pub fn get_theme_color(edition: &PlatformEdition) -> &'static str {
    if *edition == PlatformEdition::Bedrock {
        "theme-bedrock"
    } else {
        "theme-java"
    }
}
