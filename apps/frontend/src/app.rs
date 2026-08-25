//! The frame every page renders inside.

use crate::Route;
use crate::components::{footer::Footer, header::Header};
use dioxus::prelude::*;

/// Header, routed body, footer.
///
/// This is the layout of the route table rather than a component any page mounts, which is what
/// makes the header and the footer outlive a navigation: the router swaps only what is inside
/// [`Outlet`], so the search box keeps its state and the sticky bar never repaints.
#[component]
pub fn AppShell() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Header {}
            main { class: "flex-grow",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}
