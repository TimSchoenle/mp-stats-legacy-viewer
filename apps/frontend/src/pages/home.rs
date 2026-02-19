use crate::Route;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white">
            <h1 class="text-5xl font-bold mb-8">{ "MP Stats Legacy Viewer" }</h1>
            <p>{"This is an early prototype of the legacy data collected by the StatsBot. This page will crash and other stuff."}</p>
            <div class="flex space-x-8">
                <Link<Route> to={Route::Landing {edition: PlatformEdition::Java}} classes="p-10 bg-green-600 rounded-lg shadow-xl hover:bg-green-500 transition transform hover:scale-105">
                    <h2 class="text-3xl font-bold">{ "Java Edition" }</h2>
                </Link<Route>>
        
                <Link<Route> to={Route::Landing {edition: PlatformEdition::Bedrock}} classes="p-10 bg-green-600 rounded-lg shadow-xl hover:bg-green-500 transition transform hover:scale-105">
                    <h2 class="text-3xl font-bold">{ "Bedrock Edition" }</h2>
                </Link<Route>>
            </div>
        </div>
    }
}
