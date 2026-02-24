use yew::prelude::*;

#[function_component(PlayerCard)]
pub fn player_card() -> Html {
    html! {
        <div class="card p-6">
            <h3 class="text-2xl font-bold mb-2 text-white">{ "Player Name" }</h3>
            <p class="text-gray-400 mb-4 font-mono text-sm">{ "UUID: ..." }</p>
            // Stats grid
        </div>
    }
}
