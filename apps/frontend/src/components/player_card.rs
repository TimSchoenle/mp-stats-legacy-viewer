use yew::prelude::*;

#[function_component(PlayerCard)]
pub fn player_card() -> Html {
    html! {
        <div class="bg-gray-800 p-6 rounded-lg shadow-lg">
            <h3 class="text-2xl font-bold mb-2">{ "Player Name" }</h3>
            <p class="text-gray-400 mb-4">{ "UUID: ..." }</p>
            // Stats grid
        </div>
    }
}
