use crate::Route;
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
    html! {
        <div>
            <div class="flex items-center text-sm text-gray-400 mb-2 space-x-2">
                <Link<Route> to={Route::Home} classes="hover:text-white transition">{"Home"}</Link<Route>>
                <span>{"/"}</span>
                <Link<Route> to={Route::Landing { edition: props.edition.clone()}} classes="hover:text-white transition">{props.edition.display_name()}</Link<Route>>
                <span>{"/"}</span>
                <Link<Route> to={Route::Game {edition: props.edition.clone(), game: props.game.clone() }} classes="hover:text-white transition">{ &props.game }</Link<Route>>
                <span>{"/"}</span>
                <span class="text-white">{ &props.stat }</span>
            </div>
            <h1 class="text-3xl font-bold flex items-center gap-3">
                <span class="text-emerald-400">{ &props.game }</span>
                <span class="text-gray-600">{"/"}</span>
                <span class="text-blue-400 capitalize">{ props.stat.replace("_", " ") }</span>
            </h1>
        </div>
    }
}
