//! Mounts the client. Everything it mounts is [`mp_stats_frontend`].

use dioxus::prelude::*;
use mp_stats_frontend::{Api, Route};

/// The one place the [`Api`] context and the router are created, so every component below shares
/// a single fetch cache for the lifetime of the tab.
///
/// The context is provided above [`Router`] rather than inside the route table, because a value
/// provided by a layout is dropped and rebuilt when the reader navigates out of that layout - and
/// a cache that empties on navigation is not a cache.
#[component]
fn Root() -> Element {
    use_context_provider(Api::default);

    rsx! {
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(Root);
}
