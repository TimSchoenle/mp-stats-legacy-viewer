use crate::hooks::use_theme;
use crate::{Api, Route};
use mp_stats_core::models::PlatformEdition;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{HtmlInputElement, KeyboardEvent, MouseEvent};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SearchBarProps {
    #[prop_or(Classes::from("max-w-md"))]
    pub class: Classes,
    #[prop_or(Classes::from("py-2 pl-10 pr-12 text-sm rounded-md"))]
    pub input_classes: Classes,
}

#[derive(Clone, PartialEq)]
enum Suggestion {
    Player(PlatformEdition, String, String),
    UuidAction(PlatformEdition, String),
}

struct SearchState {
    pub query: UseStateHandle<String>,
    pub latest_query: Rc<RefCell<String>>,
    pub suggestions: UseStateHandle<Vec<Suggestion>>,
    pub focused_index: UseStateHandle<Option<usize>>,
    pub show_dropdown: UseStateHandle<bool>,
}

#[hook]
fn use_player_search() -> SearchState {
    let query = use_state(String::new);
    let latest_query = use_mut_ref(String::new);
    let suggestions = use_state(Vec::<Suggestion>::new);
    let focused_index = use_state(|| Option::<usize>::None);
    let show_dropdown = use_state(|| false);

    SearchState {
        query,
        latest_query,
        suggestions,
        focused_index,
        show_dropdown,
    }
}

#[derive(Properties, PartialEq)]
struct DropdownProps {
    suggestions: Vec<Suggestion>,
    focused_index: Option<usize>,
    on_navigate: Callback<Suggestion>,
}

#[function_component(SearchDropdown)]
fn search_dropdown(props: &DropdownProps) -> Html {
    html! {
        <div class="absolute mt-2 w-full card overflow-hidden z-50 shadow-2xl">
            { for props.suggestions.iter().enumerate().map(|(index, suggestion)| {
                let is_focused = props.focused_index == Some(index);
                let bg_class = if is_focused { "bg-ink-3" } else { "hover:bg-ink-3/60" };

                let onmousedown = {
                    let on_navigate = props.on_navigate.clone();
                    let suggestion = suggestion.clone();
                    Callback::from(move |e: MouseEvent| {
                        e.prevent_default();
                        on_navigate.emit(suggestion.clone());
                    })
                };

                match suggestion {
                    Suggestion::Player(edition, name, uuid) => {
                        let badge_class = match edition {
                            PlatformEdition::Java => "chip chip-mint",
                            PlatformEdition::Bedrock => "chip chip-azure",
                        };
                        let short_uuid = if uuid.len() > 8 { &uuid[..8] } else { uuid.as_str() };
                        html! {
                            <div {onmousedown} class={classes!("px-4", "py-2.5", "cursor-pointer", "flex", "items-center", "justify-between", "gap-3", "transition-colors", bg_class)}>
                                <div class="flex items-center gap-3 min-w-0">
                                    <span class="text-paper-1 text-sm font-medium truncate">{name}</span>
                                    <span class="font-mono text-xs text-paper-3">{short_uuid}{"…"}</span>
                                </div>
                                <span class={badge_class}>{ edition.display_name() }</span>
                            </div>
                        }
                    }
                    Suggestion::UuidAction(edition, _uuid) => {
                        let badge_class = match edition {
                            PlatformEdition::Java => "chip chip-mint",
                            PlatformEdition::Bedrock => "chip chip-azure",
                        };
                        let text = match edition {
                            PlatformEdition::Java => "Look up UUID in Java",
                            PlatformEdition::Bedrock => "Look up UUID in Bedrock",
                        };
                        html! {
                            <div {onmousedown} class={classes!("px-4", "py-2.5", "cursor-pointer", "flex", "items-center", "justify-between", "gap-3", "transition-colors", bg_class)}>
                                <span class="text-sm font-medium text-paper-2">{text}</span>
                                <span class={badge_class}>{ edition.display_name() }</span>
                            </div>
                        }
                    }
                }
            }) }
        </div>
    }
}

#[function_component(SearchBar)]
pub fn search_bar(props: &SearchBarProps) -> Html {
    let navigator = use_navigator().unwrap();
    let api_ctx = use_context::<Api>().expect("no api found");
    let theme_color = use_theme();

    let state = use_player_search();

    let oninput = {
        let query = state.query.clone();
        let latest_query = state.latest_query.clone();
        let suggestions = state.suggestions.clone();
        let show_dropdown = state.show_dropdown.clone();
        let focused_index = state.focused_index.clone();
        let api_ctx = api_ctx.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let val = input.value();
            query.set(val.clone());
            *latest_query.borrow_mut() = val.clone();
            focused_index.set(None);

            if val.is_empty() {
                suggestions.set(Vec::new());
                show_dropdown.set(false);
                return;
            }

            let val_len = val.len();
            if val_len == 32 || val_len == 36 {
                suggestions.set(vec![
                    Suggestion::UuidAction(PlatformEdition::Java, val.clone()),
                    Suggestion::UuidAction(PlatformEdition::Bedrock, val.clone()),
                ]);
                show_dropdown.set(true);
                return;
            }

            if val_len >= 3 {
                let q = val.clone();
                let suggestions = suggestions.clone();
                let show_dropdown = show_dropdown.clone();
                let ctx = api_ctx.clone();
                let query_ref = latest_query.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(results) = ctx.search_players_by_name(&q).await
                        && *query_ref.borrow() == q
                    {
                        let mapped: Vec<Suggestion> = results
                            .into_iter()
                            .map(|(ed, name, uuid)| Suggestion::Player(ed, name, uuid))
                            .collect();
                        suggestions.set(mapped);
                        show_dropdown.set(true);
                    }
                });
            } else {
                suggestions.set(Vec::new());
                show_dropdown.set(false);
            }
        })
    };

    let navigate_to = {
        let navigator = navigator.clone();
        let show_dropdown = state.show_dropdown.clone();
        let query = state.query.clone();

        Callback::from(move |suggestion: Suggestion| {
            show_dropdown.set(false);
            match suggestion {
                Suggestion::Player(edition, name, uuid) => {
                    query.set(name);
                    navigator.push(&Route::Player { edition, uuid });
                }
                Suggestion::UuidAction(edition, uuid) => {
                    navigator.push(&Route::Player { edition, uuid });
                }
            }
        })
    };

    let onkeydown = {
        let suggestions = state.suggestions.clone();
        let focused_index = state.focused_index.clone();
        let show_dropdown = state.show_dropdown.clone();
        let navigate_to = navigate_to.clone();

        Callback::from(move |e: KeyboardEvent| {
            if !*show_dropdown || suggestions.is_empty() {
                return;
            }

            let len = suggestions.len();
            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    let next = match *focused_index {
                        Some(i) => {
                            if i + 1 < len {
                                i + 1
                            } else {
                                0
                            }
                        }
                        None => 0,
                    };
                    focused_index.set(Some(next));
                }
                "ArrowUp" => {
                    e.prevent_default();
                    let prev = match *focused_index {
                        Some(i) => {
                            if i > 0 {
                                i - 1
                            } else {
                                len - 1
                            }
                        }
                        None => len - 1,
                    };
                    focused_index.set(Some(prev));
                }
                "Enter" => {
                    e.prevent_default();
                    if let Some(i) = *focused_index {
                        if let Some(s) = suggestions.get(i) {
                            navigate_to.emit(s.clone());
                        }
                    } else if let Some(s) = suggestions.first() {
                        navigate_to.emit(s.clone());
                    }
                }
                "Escape" => {
                    show_dropdown.set(false);
                }
                _ => {}
            }
        })
    };

    let onsubmit = {
        let suggestions = state.suggestions.clone();
        let navigate_to = navigate_to.clone();
        let query = state.query.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if let Some(s) = suggestions.first() {
                navigate_to.emit(s.clone());
            } else {
                let val = query.trim().to_string();
                if val.is_empty() {
                    return;
                }
                // No matching suggestion: still route to the player page so the
                // visitor lands on the dedicated "no profile data" empty state
                // (which explains why a profile may be missing) instead of the
                // form silently doing nothing. Default to the Java edition.
                navigate_to.emit(Suggestion::UuidAction(PlatformEdition::Java, val));
            }
        })
    };

    let onblur = {
        let show_dropdown = state.show_dropdown.clone();
        Callback::from(move |_| {
            let show_dropdown = show_dropdown.clone();
            gloo_timers::callback::Timeout::new(200, move || {
                show_dropdown.set(false);
            })
            .forget();
        })
    };

    let input_ref = use_node_ref();

    let onfocus = {
        let show_dropdown = state.show_dropdown.clone();
        let query = state.query.clone();
        Callback::from(move |_| {
            if !query.is_empty() {
                show_dropdown.set(true);
            }
        })
    };

    html! {
        <div class={classes!(theme_color, "relative", "w-full", props.class.clone())}>
            <form {onsubmit} class="relative flex items-center">
                // Search icon (left)
                <span class="absolute left-4 top-1/2 -translate-y-1/2 text-paper-4 pointer-events-none">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <circle cx="11" cy="11" r="7"/>
                        <path stroke-linecap="round" d="m21 21-4.3-4.3"/>
                    </svg>
                </span>

                <input
                    ref={input_ref}
                    type="text"
                    placeholder="Find a player by name or UUID…"
                    class={classes!("input-text", "font-mono", props.input_classes.clone())}
                    value={(*state.query).clone()}
                    {oninput}
                    {onkeydown}
                    {onfocus}
                    {onblur}
                    autocomplete="off"
                />

                // ⌘K hint or submit button (right)
                <button
                    type="submit"
                    class="absolute right-2 top-1/2 -translate-y-1/2 px-3 py-1.5 rounded font-mono text-[11px] font-semibold uppercase tracking-[0.1em] bg-theme-500 text-ink-0 border border-theme-500 hover:bg-theme-400 transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
                    disabled={state.query.is_empty()}
                    title="Search"
                >
                    { "Search" }
                </button>
            </form>

            if *state.show_dropdown && !state.suggestions.is_empty() {
                <SearchDropdown
                    suggestions={(*state.suggestions).clone()}
                    focused_index={*state.focused_index}
                    on_navigate={navigate_to}
                />
            }
        </div>
    }
}
