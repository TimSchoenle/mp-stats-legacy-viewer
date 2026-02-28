use crate::hooks::use_theme;
use mp_stats_core::models::{LeaderboardMeta, PlatformEdition};
use web_sys::js_sys::Date;
use web_sys::js_sys::Intl::DateTimeFormatOptions;
use web_sys::wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SnapshotSelectorProps {
    pub edition: PlatformEdition,
    pub current_snapshot: String,
    pub meta: Option<LeaderboardMeta>,
    pub on_change: Callback<String>,
}

#[function_component(SnapshotSelector)]
pub fn snapshot_selector(props: &SnapshotSelectorProps) -> Html {
    let theme_color = use_theme();

    let meta = match &props.meta {
        Some(m) => m,
        None => return html! {},
    };

    if meta.snapshots.is_empty() {
        return html! {};
    }

    let onchange = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: Event| {
            let target: web_sys::HtmlSelectElement = e.target_unchecked_into();
            on_change.emit(target.value());
        })
    };

    let mut sorted_snapshots = meta.snapshots.clone();
    sorted_snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    let locale = web_sys::window()
        .map(|w| w.navigator())
        .and_then(|n| n.language())
        .unwrap_or_else(|| "en-US".to_string());
    let date_formats = DateTimeFormatOptions::new();

    html! {
        <div class="flex items-center gap-3">
            <label class="text-xs text-gray-400 font-medium uppercase tracking-wider">{"Snapshot:"}</label>
            <div class="relative">
                <select
                    value={props.current_snapshot.clone()}
                    {onchange}
                    class={classes!(theme_color, "appearance-none", "px-4", "py-2", "pr-10", "bg-dark-900", "border", "border-white/10", "hover:border-white/20", "rounded-lg", "text-sm", "text-white", "cursor-pointer", "focus:outline-none", "focus:border-theme-500/50", "focus:ring-1", "focus:ring-theme-500/50", "transition-all", "shadow-sm")}
                >
                    {for sorted_snapshots.iter()
                        .map(|snap| {
                        let timestamp_ms = (snap.timestamp * 1000) as f64;
                                    let date = Date::new(&JsValue::from_f64(timestamp_ms));
                        let formated_date: String = date.to_locale_date_string(&locale, &date_formats).into();
                        html! {
                            <option
                                value={snap.snapshot_id.to_string()}
                                selected={props.current_snapshot == snap.snapshot_id.as_str()}
                            >
                                {
                                    if snap.snapshot_id == "latest" {
                                        format!("Latest ({})", &formated_date)
                                    } else {
                                        formated_date
                                    }
                                }
                            </option>
                        }
                    })}
                </select>
                <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3 text-gray-400">
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                </div>
            </div>
        </div>
    }
}
