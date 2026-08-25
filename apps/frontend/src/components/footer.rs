//! The page footer.

use dioxus::prelude::*;

/// The rule at the bottom of every page. It carries no content yet.
#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "mt-auto border-t border-rule bg-ink-0",
            div { class: "container mx-auto px-6 py-4 flex items-center justify-between" }
        }
    }
}
