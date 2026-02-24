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

    html! {
        <div class={classes!(theme_color, "mb-6")}>
            <div class="flex items-center text-sm text-gray-400 mb-3 space-x-2">
                <Link<Route> to={Route::Home} classes="hover:text-white transition">{"Home"}</Link<Route>>
                <span>{"/"}</span>
                <Link<Route> to={Route::Landing { edition: props.edition.clone()}} classes="hover:text-white transition">{props.edition.display_name()}</Link<Route>>
                <span>{"/"}</span>
                <Link<Route> to={Route::Game {edition: props.edition.clone(), game: props.game.clone() }} classes="hover:text-white transition">{ &props.game }</Link<Route>>
                <span>{"/"}</span>
                <span class={classes!("text-theme-400", "font-medium", "capitalize")}>{ props.stat.replace("_", " ") }</span>
            </div>
            <h1 class="text-3xl font-bold flex items-center gap-3 tracking-tight">
                <span class="text-white">{ &props.game }</span>
                <span class="text-gray-600 font-light">{"/"}</span>
                <span class={classes!("text-theme-500", "capitalize")}>{ props.stat.replace("_", " ") }</span>

            </h1>
        </div>
    }
}
