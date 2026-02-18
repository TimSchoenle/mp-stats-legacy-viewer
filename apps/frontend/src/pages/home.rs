use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white">
            <h1 class="text-5xl font-bold mb-8">{ "Welcome to Stats Viewer" }</h1>
            <div class="flex space-x-8">
                <Link<Route> to={Route::JavaLanding} classes="p-10 bg-green-600 rounded-lg shadow-xl hover:bg-green-500 transition transform hover:scale-105">
                    <h2 class="text-3xl font-bold">{ "Java Edition" }</h2>
                </Link<Route>>
            </div>
        </div>
    }
}
