use crate::Route;
use crate::hooks::use_theme;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub edition: PlatformEdition,
    pub game: String,
    pub stat: String,
}

#[function_component(LeaderboardHeader)]
pub fn leaderboard_header(props: &HeaderProps) -> Html {
    let theme_color = use_theme();
    let stat_display = props.stat.replace("_", " ");

    html! {
        <div class={classes!(theme_color)}>
            // Crumbs
            <div class="crumbs mb-5">
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                <span class="sep">{"/"}</span>
                <Link<Route> to={Route::Landing { edition: props.edition.clone() }}>{ props.edition.display_name() }</Link<Route>>
                <span class="sep">{"/"}</span>
                <Link<Route> to={Route::Game { edition: props.edition.clone(), game: props.game.clone() }}>{ &props.game }</Link<Route>>
                <span class="sep">{"/"}</span>
                <span class="here capitalize">{ stat_display.clone() }</span>
            </div>

            // Title
            <div class="flex flex-col md:flex-row md:items-end justify-between gap-3">
                <div>
                    <div class="eyebrow mb-2">{ format!("{} · Category", &props.game) }</div>
                    <h1 class="serif page-title text-5xl md:text-6xl text-paper-1 capitalize">
                        <span class="text-paper-3">{"Top "}</span>
                        { stat_display }
                    </h1>
                </div>
            </div>
        </div>
    }
}
