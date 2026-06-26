use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="mt-auto border-t border-rule bg-ink-0">
            <div class="container mx-auto px-6 py-4 flex items-center justify-between">
                <span class="font-mono text-[11px] text-paper-4">{"archive · open source"}</span>
                <span class="font-mono text-[11px] text-paper-4">{"snapshots 2021 – Jan 2023"}</span>
            </div>
        </footer>
    }
}
