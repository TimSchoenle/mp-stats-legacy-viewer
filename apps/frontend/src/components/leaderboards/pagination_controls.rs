use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    pub current_page: u32,
    pub max_page: u32,
    pub on_change: Callback<u32>,
}

#[function_component(PaginationControls)]
pub fn pagination_controls(props: &PaginationProps) -> Html {
    let input_value = use_state(|| props.current_page.to_string());
    
    let current_page = props.current_page;
    let max_page = props.max_page;

    // Sync internal input when external page changes
    {
        let input_value = input_value.clone();
        let page = props.current_page;
        use_effect_with(page, move |p| input_value.set(p.to_string()));
    }

    let on_submit = {
        let on_change = props.on_change.clone();
        let input_value = input_value.clone();
        let max_page = props.max_page;
        move |e: SubmitEvent| {
            e.prevent_default();
            if let Ok(p) = input_value.parse::<u32>() {
                if p >= 1 && p <= max_page {
                    on_change.emit(p);
                }
            }
        }
    };

    let emit_page = |page: u32| {
        let on_change = props.on_change.clone();
        Callback::from(move |_| on_change.emit(page))
    };

    html! {
            <div class="p-4 border-t border-gray-700 flex flex-col md:flex-row justify-between items-center bg-gray-800 gap-4">
                 // Controls Left
                 <div class="flex items-center gap-2">
                     // First Page
                     <button
                        onclick={ emit_page(1) }
                        disabled={current_page <= 1}
                        class="p-2 bg-gray-700 hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed rounded text-white transition-colors"
                        title="First Page"
                     >
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z" clip-rule="evenodd" />
                        </svg>
                     </button>

                   // Previous
                    <button
                        onclick={emit_page(current_page - 1)}
                        disabled={current_page <= 1}
                        class="px-4 py-2 bg-gray-700 hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed rounded text-sm font-bold text-white transition-colors"
                    >
                        {"Previous"}
                    </button>
                </div>

                // Middle / Input
                <form onsubmit={on_submit} class="flex items-center gap-2">
                    <input
                        type="number"
                        value={(*input_value).clone()}
                        oninput={Callback::from(move |e: InputEvent| {
                            let target: web_sys::HtmlInputElement = e.target_unchecked_into();
                            input_value.set(target.value());
                        })}
                        class="w-16 px-2 py-1 bg-gray-900 border border-gray-600 rounded text-center text-white text-sm focus:border-emerald-500 outline-none"
                    />
                    <span class="text-gray-400 txt-sm">{ format!("of {}", props.max_page) }</span>
                </form>

                // Controls Right
                <div class="flex items-center gap-2">
                    // Next
                    <button
                        onclick={emit_page(current_page + 1)}
                        disabled={current_page >= max_page}
                        class="px-4 py-2 bg-gray-700 hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed rounded text-sm font-bold text-white transition-colors"
                    >
                        {"Next"}
                    </button>

                     // Last Page
                     <button
                        onclick={emit_page(max_page)}
                        disabled={current_page >= max_page}
                        class="p-2 bg-gray-700 hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed rounded text-white transition-colors"
                        title="Last Page"
                     >
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
                        </svg>
                     </button>
                 </div>
            </div>
        }
}
