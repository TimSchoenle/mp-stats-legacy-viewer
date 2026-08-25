//! Finding a player by name or by UUID, from the bar in the header.

use crate::hooks::use_theme;
use crate::{Api, Route};
use dioxus::prelude::*;
use mp_stats_core::models::PlatformEdition;

/// One row of the dropdown.
#[derive(Clone, Debug, PartialEq)]
enum Suggestion {
    /// A player the names index matched, by edition, name and UUID.
    Player(PlatformEdition, String, String),
    /// When the typed text looks like a UUID and neither index was consulted: an offer to open
    /// that profile in one edition or the other.
    UuidAction(PlatformEdition, String),
}

impl Suggestion {
    /// The edition this row would open a profile in, which is also the chip it is labelled with.
    fn edition(&self) -> &PlatformEdition {
        match self {
            Self::Player(edition, ..) | Self::UuidAction(edition, ..) => edition,
        }
    }

    /// The chip class for [`Self::edition`].
    fn badge_class(&self) -> &'static str {
        match self.edition() {
            PlatformEdition::Java => "chip chip-mint",
            PlatformEdition::Bedrock => "chip chip-azure",
        }
    }
}

/// The length of a names index shard, and so the shortest prefix there is a file to look in for.
const MIN_QUERY_LEN: usize = 3;

/// The two lengths a UUID is written at: bare hex, and hex with dashes.
const UUID_LENGTHS: [usize; 2] = [32, 36];

/// The suggestion list.
///
/// Rows commit on mouse-down rather than click, because the box loses focus first and a click
/// would arrive after the list had closed.
#[component]
fn SearchDropdown(
    suggestions: Vec<Suggestion>,
    focused_index: Option<usize>,
    on_navigate: EventHandler<Suggestion>,
) -> Element {
    rsx! {
        div { class: "absolute mt-2 w-full card overflow-hidden z-50 shadow-2xl",
            for (index , suggestion) in suggestions.into_iter().enumerate() {
                div {
                    key: "{index}",
                    class: if focused_index == Some(index) {
                        "px-4 py-2.5 cursor-pointer flex items-center justify-between gap-3 transition-colors bg-ink-3"
                    } else {
                        "px-4 py-2.5 cursor-pointer flex items-center justify-between gap-3 transition-colors hover:bg-ink-3/60"
                    },
                    onmousedown: {
                        let suggestion = suggestion.clone();
                        move |event: Event<MouseData>| {
                            event.prevent_default();
                            on_navigate.call(suggestion.clone());
                        }
                    },

                    match &suggestion {
                        Suggestion::Player(_, name, uuid) => rsx! {
                            div { class: "flex items-center gap-3 min-w-0",
                                span { class: "text-paper-1 text-sm font-medium truncate", "{name}" }
                                span { class: "font-mono text-xs text-paper-3",
                                    "{uuid.get(..8).unwrap_or(uuid)}\u{2026}"
                                }
                            }
                        },
                        Suggestion::UuidAction(edition, _) => rsx! {
                            span { class: "text-sm font-medium text-paper-2",
                                "Look up UUID in {edition.display_name()}"
                            }
                        },
                    }

                    span { class: suggestion.badge_class(), {suggestion.edition().display_name()} }
                }
            }
        }
    }
}

/// The search box and its dropdown.
///
/// Text of 32 or 36 characters is taken for a UUID and offered as a direct profile link in either
/// edition, since a UUID names a shard and needs no index. Anything else searches from three
/// characters up, which is the shortest prefix a names index shard exists for. A response is
/// dropped unless the box still holds the query it was made for, so a slow fetch cannot overwrite
/// the suggestions for what is now typed.
///
/// `class` sizes the wrapper and `input_classes` the input, because the header and the home page
/// want different sizes out of the same component.
#[component]
pub fn SearchBar(
    #[props(default = "max-w-md".to_string())] class: String,
    #[props(default = "py-2 pl-10 pr-12 text-sm rounded-md".to_string())] input_classes: String,
) -> Element {
    let navigator = use_navigator();
    let api = use_context::<Api>();
    let theme_color = use_theme();

    let mut query = use_signal(String::new);
    let mut suggestions = use_signal(Vec::<Suggestion>::new);
    let mut focused_index = use_signal(|| Option::<usize>::None);
    let mut show_dropdown = use_signal(|| false);

    // Committing a row: navigate, and close the list behind us. Held in a callback rather than
    // written out three times, because the box, the keyboard and the dropdown all commit rows.
    let navigate_to = use_callback(move |suggestion: Suggestion| {
        show_dropdown.set(false);

        match suggestion {
            Suggestion::Player(edition, name, uuid) => {
                query.set(name);
                navigator.push(Route::Player { edition, uuid });
            }
            Suggestion::UuidAction(edition, uuid) => {
                navigator.push(Route::Player { edition, uuid });
            }
        };
    });

    let on_input = move |event: Event<FormData>| {
        let value = event.value();
        query.set(value.clone());
        focused_index.set(None);

        if value.is_empty() {
            suggestions.set(Vec::new());
            show_dropdown.set(false);
            return;
        }

        // A UUID names the shard it lives in, so there is nothing to look up: offer it in both
        // editions and let the profile page report which one has it.
        if UUID_LENGTHS.contains(&value.len()) {
            suggestions.set(vec![
                Suggestion::UuidAction(PlatformEdition::Java, value.clone()),
                Suggestion::UuidAction(PlatformEdition::Bedrock, value),
            ]);
            show_dropdown.set(true);
            return;
        }

        if value.len() < MIN_QUERY_LEN {
            suggestions.set(Vec::new());
            show_dropdown.set(false);
            return;
        }

        let api = api.clone();
        spawn(async move {
            let Ok(results) = api.search_players_by_name(&value).await else {
                return;
            };

            // The box may have moved on while this was in flight. `peek` rather than a read,
            // because subscribing a spawned task to the query it is checking would restart it.
            if *query.peek() != value {
                return;
            }

            suggestions.set(
                results
                    .into_iter()
                    .map(|(edition, name, uuid)| Suggestion::Player(edition, name, uuid))
                    .collect(),
            );
            show_dropdown.set(true);
        });
    };

    let on_key_down = move |event: Event<KeyboardData>| {
        let length = suggestions.read().len();
        if !show_dropdown() || length == 0 {
            return;
        }

        match event.key() {
            Key::ArrowDown => {
                event.prevent_default();
                focused_index.set(Some(match focused_index() {
                    Some(index) if index + 1 < length => index + 1,
                    Some(_) | None => 0,
                }));
            }
            Key::ArrowUp => {
                event.prevent_default();
                focused_index.set(Some(match focused_index() {
                    Some(index) if index > 0 => index - 1,
                    Some(_) | None => length - 1,
                }));
            }
            Key::Enter => {
                event.prevent_default();

                // With nothing focused, Enter takes the best match, which is the first row.
                let index = focused_index().unwrap_or(0);
                let suggestion = suggestions.read().get(index).cloned();
                if let Some(suggestion) = suggestion {
                    navigate_to.call(suggestion);
                }
            }
            Key::Escape => show_dropdown.set(false),
            _ => {}
        }
    };

    let on_submit = move |event: Event<FormData>| {
        event.prevent_default();

        let first = suggestions.read().first().cloned();
        if let Some(suggestion) = first {
            navigate_to.call(suggestion);
            return;
        }

        let typed = query().trim().to_string();
        if typed.is_empty() {
            return;
        }

        // No matching suggestion: still route to the player page so the visitor lands on the
        // dedicated "no profile data" empty state (which explains why a profile may be missing)
        // instead of the form silently doing nothing. Default to the Java edition.
        navigate_to.call(Suggestion::UuidAction(PlatformEdition::Java, typed));
    };

    let on_blur = move |_| {
        // The list has to outlive the blur, because a row commits on mouse-down and the box loses
        // focus before that lands.
        gloo_timers::callback::Timeout::new(200, move || show_dropdown.set(false)).forget();
    };

    rsx! {
        div { class: "{theme_color} relative w-full {class}",
            form { class: "relative flex items-center", onsubmit: on_submit,

                // Search icon (left)
                span { class: "absolute left-4 top-1/2 -translate-y-1/2 text-paper-4 pointer-events-none",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-4 w-4",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "2",
                        circle { cx: "11", cy: "11", r: "7" }
                        path { stroke_linecap: "round", d: "m21 21-4.3-4.3" }
                    }
                }

                input {
                    r#type: "text",
                    placeholder: "Find a player by name or UUID\u{2026}",
                    class: "input-text font-mono {input_classes}",
                    value: "{query}",
                    autocomplete: "off",
                    oninput: on_input,
                    onkeydown: on_key_down,
                    onfocus: move |_| {
                        if !query().is_empty() {
                            show_dropdown.set(true);
                        }
                    },
                    onblur: on_blur,
                }

                button {
                    r#type: "submit",
                    class: "absolute right-2 top-1/2 -translate-y-1/2 px-3 py-1.5 rounded font-mono text-[11px] font-semibold uppercase tracking-[0.1em] bg-theme-500 text-ink-0 border border-theme-500 hover:bg-theme-400 transition-colors disabled:opacity-40 disabled:cursor-not-allowed",
                    disabled: query().is_empty(),
                    title: "Search",
                    "Search"
                }
            }

            if show_dropdown() && !suggestions.read().is_empty() {
                SearchDropdown {
                    suggestions: suggestions(),
                    focused_index: focused_index(),
                    on_navigate: move |suggestion| navigate_to.call(suggestion),
                }
            }
        }
    }
}
