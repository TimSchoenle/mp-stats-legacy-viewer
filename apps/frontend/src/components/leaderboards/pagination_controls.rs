//! Moving between the pages of a board.

use crate::hooks::use_theme;
use dioxus::prelude::*;

/// The class every one of the five controls is drawn with.
const NAV_BUTTON: &str = "btn px-3 py-2 disabled:opacity-40 disabled:cursor-not-allowed";

/// First, previous, a page box, next and last.
///
/// The box is typed into freely and only reports on submit, and only for a number inside the
/// board, so a half-typed page never triggers a fetch.
///
/// `current_page` and `max_page` are both 1-based, and both ends disable their buttons on reaching
/// them. `on_change` is called with a 1-based page number already clamped to `1..=max_page`.
///
/// The theme comes from the route rather than from an `edition` prop the caller would have to keep
/// in step with the URL it already encodes.
#[component]
pub fn PaginationControls(
    current_page: u32,
    max_page: u32,
    on_change: EventHandler<u32>,
) -> Element {
    let theme_color = use_theme();

    let mut input_value = use_signal(|| current_page.to_string());

    // The box follows the page it is reporting on, so arriving at a page through any of the other
    // four controls - or through a link, or the back button - leaves the number agreeing with the
    // table beside it.
    use_effect(use_reactive!(
        |current_page| input_value.set(current_page.to_string())
    ));

    rsx! {
        div {
            class: "{theme_color} p-4 border-t border-rule flex flex-col md:flex-row justify-between items-center gap-3 bg-ink-1",

            // Left controls
            div { class: "flex items-center gap-2",
                button {
                    class: NAV_BUTTON,
                    disabled: current_page <= 1,
                    title: "First page",
                    onclick: move |_| on_change.call(1),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-4 w-4",
                        view_box: "0 0 20 20",
                        fill: "currentColor",
                        path {
                            fill_rule: "evenodd",
                            clip_rule: "evenodd",
                            d: "M15.707 15.707a1 1 0 01-1.414 0L8 9.414V13a1 1 0 11-2 0V7a1 1 0 011-1h6a1 1 0 110 2H9.414l6.293 6.293a1 1 0 010 1.414z",
                        }
                    }
                }
                button {
                    class: NAV_BUTTON,
                    disabled: current_page <= 1,
                    onclick: move |_| on_change.call(current_page.saturating_sub(1).max(1)),
                    "\u{2190} Prev"
                }
            }

            // Middle / page input
            form {
                class: "flex items-center gap-2",
                onsubmit: move |event| {
                    event.prevent_default();
                    if let Ok(page) = input_value().parse::<u32>()
                        && (1..=max_page).contains(&page)
                    {
                        on_change.call(page);
                    }
                },
                span { class: "eyebrow", "Page" }
                input {
                    r#type: "number",
                    class: "w-16 px-2 py-1 bg-ink-2 border border-rule rounded-md text-center font-mono text-sm text-paper-1 focus:border-theme-500/60 outline-none tnum",
                    value: "{input_value}",
                    oninput: move |event| input_value.set(event.value()),
                }
                span { class: "font-mono text-xs text-paper-3 tnum", "of {max_page}" }
            }

            // Right controls
            div { class: "flex items-center gap-2",
                button {
                    class: NAV_BUTTON,
                    disabled: current_page >= max_page,
                    onclick: move |_| on_change.call(current_page + 1),
                    "Next \u{2192}"
                }
                button {
                    class: NAV_BUTTON,
                    disabled: current_page >= max_page,
                    title: "Last page",
                    onclick: move |_| on_change.call(max_page),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-4 w-4",
                        view_box: "0 0 20 20",
                        fill: "currentColor",
                        path {
                            fill_rule: "evenodd",
                            clip_rule: "evenodd",
                            d: "M4.293 4.293a1 1 0 011.414 0L12 10.586V7a1 1 0 112 0v6a1 1 0 01-1 1H7a1 1 0 110-2h3.586L4.293 5.707a1 1 0 010-1.414z",
                        }
                    }
                }
            }
        }
    }
}
