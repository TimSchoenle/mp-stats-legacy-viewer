//! The page for a URL that matches no route.

use crate::Route;
use dioxus::prelude::*;

/// The 404 page, reached for any URL the router does not match.
///
/// `segments` is the path that matched nothing. The page does not show it - a reader who mistyped
/// a URL can already see it in the address bar - but the catch-all route has to put it somewhere.
#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-[calc(100vh-8rem)] p-6",
            div { class: "card p-12 text-center max-w-md w-full",
                div { class: "eyebrow mb-6", "Error \u{b7} page not found" }
                h1 { class: "serif page-title text-7xl mb-6 text-paper-1", "404" }
                p { class: "text-sm text-paper-3 mb-8 leading-relaxed",
                    "The page you're looking for isn't in the archive."
                }
                Link { to: Route::Home {}, class: "btn", "\u{2190} Return home" }
            }
        }
    }
}
