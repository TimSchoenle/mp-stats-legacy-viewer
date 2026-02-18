use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen text-white">
            <h1 class="text-6xl font-bold mb-4">{ "404" }</h1>
            <p class="text-xl">{ "Page not found" }</p>
        </div>
    }
}
