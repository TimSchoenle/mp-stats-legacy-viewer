use crate::hooks::use_theme;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    pub edition: PlatformEdition,
    pub current_page: u32,
    pub max_page: u32,
    pub on_change: Callback<u32>,
}

#[function_component(PaginationControls)]
pub fn pagination_controls(props: &PaginationProps) -> Html {
    let theme_color = use_theme();

    let input_value = use_state(|| props.current_page.to_string());

    let current_page = props.current_page;
    let max_page = props.max_page;

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
            if let Ok(p) = input_value.parse::<u32>()
                && (p >= 1 && p <= max_page)
            {
                on_change.emit(p);
            }
        }
    };

    let emit_page = |page: u32| {
        let on_change = props.on_change.clone();
        Callback::from(move |_| on_change.emit(page))
    };

    let nav_btn = "btn px-3 py-2 disabled:opacity-40 disabled:cursor-not-allowed";

    html! {
        <div class={classes!(theme_color, "p-4", "border-t", "border-rule", "flex", "flex-col", "md:flex-row", "justify-between", "items-center", "gap-3", "bg-ink-1")}>
            // Left controls
            <div class="flex items-center gap-2">
                <button
                    onclick={ emit_page(1) }
                    disabled={current_page <= 1}
                    class={nav_btn}
                    title="First page"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M15.707 15.707a1 1 0 01-1.414 0L8 9.414V13a1 1 0 11-2 0V7a1 1 0 011-1h6a1 1 0 110 2H9.414l6.293 6.293a1 1 0 010 1.414z" clip-rule="evenodd" />
                    </svg>
                </button>
                <button
                    onclick={emit_page(current_page.saturating_sub(1).max(1))}
                    disabled={current_page <= 1}
                    class={nav_btn}
                >
                    { "← Prev" }
                </button>
            </div>

            // Middle / page input
            <form onsubmit={on_submit} class="flex items-center gap-2">
                <span class="eyebrow">{"Page"}</span>
                <input
                    type="number"
                    value={(*input_value).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let target: web_sys::HtmlInputElement = e.target_unchecked_into();
                        input_value.set(target.value());
                    })}
                    class="w-16 px-2 py-1 bg-ink-2 border border-rule rounded-md text-center font-mono text-sm text-paper-1 focus:border-theme-500/60 outline-none tnum"
                />
                <span class="font-mono text-xs text-paper-4 tnum">{ format!("of {}", props.max_page) }</span>
            </form>

            // Right controls
            <div class="flex items-center gap-2">
                <button
                    onclick={emit_page(current_page + 1)}
                    disabled={current_page >= max_page}
                    class={nav_btn}
                >
                    { "Next →" }
                </button>
                <button
                    onclick={emit_page(max_page)}
                    disabled={current_page >= max_page}
                    class={nav_btn}
                    title="Last page"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L12 10.586V7a1 1 0 112 0v6a1 1 0 01-1 1H7a1 1 0 110-2h3.586L4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                    </svg>
                </button>
            </div>
        </div>
    }
}
