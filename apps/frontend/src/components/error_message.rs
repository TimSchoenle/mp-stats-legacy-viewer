//! The one way a page reports that something did not load.

use dioxus::prelude::*;

/// The site's one failure state, as a full-width card or as a banner over content that survived.
///
/// `title` should name what could not be loaded, `message` carries the detail, and `is_banner`
/// renders it as a strip above content that did load rather than in place of it.
#[component]
pub fn ErrorMessage(
    title: String,
    message: String,
    #[props(default = false)] is_banner: bool,
) -> Element {
    if is_banner {
        return rsx! {
            div {
                class: "card mb-6 p-5",
                style: "border-color: color-mix(in oklch, var(--color-brand-rose-500), transparent 60%);",
                div { class: "eyebrow mb-2", style: "color: var(--color-brand-rose-500);", "⚠ {title}" }
                p { class: "text-sm text-paper-2 leading-relaxed", "{message}" }
            }
        };
    }

    rsx! {
        div { class: "card p-12 text-center",
            div { class: "eyebrow mb-4", style: "color: var(--color-brand-rose-500);", "⚠ Error" }
            p { class: "serif text-2xl text-paper-1 mb-2", "{title}" }
            p { class: "text-sm text-paper-3", "{message}" }
        }
    }
}
