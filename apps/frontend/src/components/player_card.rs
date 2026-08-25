//! An unused early sketch of the profile card.

use dioxus::prelude::*;

/// A profile card filled with placeholder text.
// Mounted by nothing. The profile page draws its own header.
#[component]
pub fn PlayerCard() -> Element {
    rsx! {
        div { class: "card p-6",
            h3 { class: "text-2xl font-bold mb-2 text-white", "Player Name" }
            p { class: "text-gray-400 mb-4 font-mono text-sm", "UUID: ..." }
            // Stats grid
        }
    }
}
