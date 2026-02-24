use crate::hooks::use_theme;
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
    let games = use_state(Vec::new);
    let api_ctx = use_context::<Api>().expect("no api found found");

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

    let sorted_games = {
        let mut games_vec = (*games).clone();
        games_vec.sort_by(|a, b| a.name.cmp(&b.name));
        games_vec
    };

    let theme_color = use_theme();

    html! {
        <div class={classes!(theme_color, "container", "mx-auto", "px-4", "py-8", "text-white", "relative")}>
            // Hero
            <div class="mb-12 mt-8 text-center">
                <h1 class="text-5xl font-bold mb-4 tracking-tight text-white drop-shadow-sm">
                    { format!("{} Edition", props.edition.display_name()) }
                </h1>
                <p class="text-xl text-gray-300 max-w-2xl mx-auto font-light">
                    { "Browse historical leaderboards and detailed statistics for " }
                    <span class="text-white font-bold">{ games.len() }</span>
                    { " archived games." }
                </p>
            </div>

            // Games Grid
            if games.is_empty() {
                <div class="grid grid-cols-1 md::grid-cols-2 lg:grid-cols-3 gap-6 animate-pulse">
                    <div class="h-32 bg-dark-850 rounded-2xl border border-white/5"></div>
                    <div class="h-32 bg-dark-850 rounded-2xl border border-white/5"></div>
                    <div class="h-32 bg-dark-850 rounded-2xl border border-white/5"></div>
                </div>
            } else {
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-5">
                    { for sorted_games.iter().map(|game| {
                        let edition = props.edition.clone();

                        html! {
                            <Link<Route>
                                to={Route::Game { edition, game: game.name.to_string() }}
                                classes={classes!("group", "card", "p-6", "hover:border-theme-500/50", "transition-all", "flex", "flex-col", "h-full", "bg-dark-850", "hover:bg-dark-800", "shadow-sm")}
                            >
                                <h2 class={classes!("text-xl", "font-bold", "mb-2", "group-hover:text-theme-400", "transition-colors", "text-white")}>{ game.name.as_str() }</h2>
                            <p class="text-sm text-gray-400 font-medium flex items-center gap-1 group-hover:text-gray-300 mt-auto pt-4">
                                { "View Statistics" }
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 transform group-hover:translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                </svg>
                            </p>
                        </Link<Route>>
                        }
                    }) }
                </div>
            }
        </div>
    }
}
