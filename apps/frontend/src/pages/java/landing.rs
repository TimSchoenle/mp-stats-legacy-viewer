use crate::{Api, Route};
use mp_stats_core::models::PlatformEdition;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct JavaLandingProps {
    pub edition: PlatformEdition,
}

#[function_component(JavaLanding)]
pub fn java_landing(props: &JavaLandingProps) -> Html {
    let games = use_state(|| vec![]);
    let api_ctx = use_context::<Api>().expect("no api found found");;

    {
        let games = games.clone();

        use_effect_with(
            (api_ctx, props.edition.clone()),
            move |(ctx, current_edition)| {
                let provider = ctx.clone();
                    let edition_to_fetch = current_edition.clone();

                    // Clear games
                    games.set(vec![]);

                    spawn_local(async move {
                        if let Ok(meta) = provider.fetch_meta(&edition_to_fetch).await {
                            games.set(meta.games);
                        } else {
                            games.set(vec![]);
                        }
                    });
                || ()
            },
        );
    }

    html! {
        <div class="container mx-auto px-4 py-8 text-white">
            // Hero
            <div class="text-center mb-12 py-8">
                <h1 class="text-4xl font-bold mb-3 tracking-tight text-white">
                    { format!("{} Edition Stats", props.edition.display_name()) }
                </h1>
                <p class="text-lg text-gray-400 max-w-2xl mx-auto">
                    { "Explore detailed statistics and leaderboards for " }
                    <span class="text-white font-bold">{ games.len() }</span>
                    { " games" }
                </p>
            </div>

            // Games Grid
            if games.is_empty() {
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 animate-pulse">
                    <div class="h-28 bg-gray-800 rounded-lg"></div>
                    <div class="h-28 bg-gray-800 rounded-lg"></div>
                    <div class="h-28 bg-gray-800 rounded-lg"></div>
                </div>
            } else {
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                    { for games.iter().map(|game| html! {
                        <Link<Route>
                            to={Route::Game { edition: props.edition.clone(), game: game.id.to_string() }}
                            classes="group card p-5 hover:border-emerald-600 transition-all flex flex-col"
                        >
                            <h2 class="text-lg font-bold mb-1 group-hover:text-emerald-400 transition-colors">{ &*game.name.as_str() }</h2>
                            <p class="text-sm text-gray-400 font-medium flex items-center gap-1 group-hover:text-gray-300">
                                { "View Stats" }
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 transform group-hover:translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                </svg>
                            </p>
                        </Link<Route>>
                    }) }
                </div>
            }
        </div>
    }
}
