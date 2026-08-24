//! The page footer.

use yew::prelude::*;

/// The rule at the bottom of every page. It carries no content yet.
#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="mt-auto border-t border-rule bg-ink-0">
            <div class="container mx-auto px-6 py-4 flex items-center justify-between">
            </div>
        </footer>
    }
}
