//! An unused early sketch of the leaderboard table.

use dioxus::prelude::*;

/// A leaderboard table that renders a permanent loading row, under the heading `title` says it is
/// a leaderboard of.
// A placeholder from before the page was built, mounted by nothing. The table that is really used
// is `components::leaderboards::leaderboard_table`.
#[component]
pub fn Leaderboard(title: String) -> Element {
    rsx! {
        div { class: "container mx-auto p-4",
            h2 { class: "text-xl font-bold mb-4", "{title}" }
            div { class: "overflow-x-auto",
                table { class: "min-w-full bg-gray-800 text-white rounded-lg overflow-hidden",
                    thead { class: "bg-gray-700",
                        tr {
                            th { class: "p-3 text-left", "Rank" }
                            th { class: "p-3 text-left", "Name/UUID" }
                            th { class: "p-3 text-left", "Score" }
                        }
                    }
                    tbody {
                        // Rows will be populated here
                        tr {
                            td { class: "p-3", colspan: "3", "Loading..." }
                        }
                    }
                }
            }
        }
    }
}
