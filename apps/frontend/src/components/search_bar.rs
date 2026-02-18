use crate::Route;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    let navigator = use_navigator().unwrap();
    let query = use_state(|| String::new());
    let loading = use_state(|| false);

    let oninput = {
        let query = query.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            query.set(input.value());
        })
    };

    let onsubmit = {
        let query = query.clone();
        let navigator = navigator.clone();
        let loading = loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let q = query.to_string();
            let navigator = navigator.clone();
            let loading = loading.clone();

            if q.is_empty() {
                return;
            }

            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                // Heuristic: if length is 32 or 36, assume UUID
                if q.len() == 32 || q.len() == 36 {
                    navigator.push(&Route::JavaPlayer { uuid: q });
                } else {
                    // Try to resolve name to UUID (Java) - only available in WASM
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Ok(Some(lookup)) =
                            mp_stats_data_client::api::find_player_uuid(&q).await
                        {
                            navigator.push(&Route::JavaPlayer {
                                uuid: lookup.uuid.to_string(),
                            });
                        }
                    }
                }
                loading.set(false);
            });
        })
    };

    html! {
        <form onsubmit={onsubmit} class="relative w-full max-w-md">
            <div class="relative flex items-center">
                <input
                    type="text"
                    placeholder="Search player (Name/UUID)..."
                    class="w-full bg-white/5 border border-white/10 text-white placeholder-gray-400 rounded-full py-2 pl-4 pr-12 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:bg-white/10 transition-all backdrop-blur-sm"
                    value={(*query).clone()}
                    {oninput}
                    disabled={*loading}
                />
                <button
                    type="submit"
                    class="absolute right-1 top-1 bottom-1 bg-emerald-600 hover:bg-emerald-500 text-white rounded-full px-4 text-sm font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                    disabled={*loading || query.is_empty()}
                >
                    if *loading {
                        <span class="animate-pulse">{ "..." }</span>
                    } else {
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                        </svg>
                    }
                </button>
            </div>
        </form>
    }
}
