use crate::components::{footer::Footer, header::Header};
use crate::pages::{self, home::Home, not_found::NotFound};
use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <Header />
            <main class="flex-grow">
                <Switch<Route> render={switch} />
            </main>
            <Footer />
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Landing { edition } => html! { <pages::java::JavaLanding edition={edition} /> },
        Route::Game { edition, game } => html! {
            <pages::java::GameView edition={edition} game={game} />
        },
        Route::Leaderboard {
            edition,
            game,
            board,
            stat,
            page,
        } => html! {
            <pages::java::LeaderboardView edition={edition} game={game} board={board} stat={stat} page={page}  />
        },
        Route::Player { edition, uuid } => html! {
            <pages::java::PlayerView edition={edition} uuid={uuid} />
        },
        Route::NotFound => html! { <NotFound /> },
    }
}
