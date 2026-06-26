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
    let hovered = use_state(|| None::<usize>);

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

    // -------- Tickmark timeline --------
    // Sort ASC for timeline positions
    let mut chronological = meta.snapshots.clone();
    chronological.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    let total = chronological.len();
    let min_ts = chronological.first().map(|s| s.timestamp).unwrap_or(0);
    let max_ts = chronological.last().map(|s| s.timestamp).unwrap_or(1);
    let range = (max_ts - min_ts).max(1);

    let fmt_date = |ts: u64| -> String {
        let ms = (ts * 1000) as f64;
        let d = Date::new(&JsValue::from_f64(ms));
        d.to_locale_date_string(&locale, &date_formats).into()
    };

    let first_date = fmt_date(min_ts);
    let last_date = fmt_date(max_ts);

    let current_snapshot_str = props.current_snapshot.clone();

    let active_snap = if current_snapshot_str == "latest" {
        chronological.last().cloned()
    } else {
        chronological
            .iter()
            .find(|s| s.snapshot_id == current_snapshot_str)
            .cloned()
    };
    let active_label = active_snap.as_ref().map(|s| {
        if current_snapshot_str == "latest" {
            format!("Latest · {}", fmt_date(s.timestamp))
        } else {
            fmt_date(s.timestamp)
        }
    }).unwrap_or_else(|| "—".to_string());

    html! {
        <div class={classes!(theme_color, "card", "p-4")}>
            <div class="flex items-baseline justify-between mb-3 gap-3 flex-wrap">
                <div class="eyebrow">
                    { format!("Snapshot · {} archived", total) }
                </div>
                <div class="font-mono text-[11px] text-paper-3">
                    { "Viewing " }
                    <span class="text-theme-500">{ active_label }</span>
                </div>
            </div>

            // Tickmark SVG (interactive)
            <div class="relative w-full">
                <svg viewBox="0 0 1180 36" preserveAspectRatio="none" class="block w-full h-9">
                    <line x1="0" y1="22" x2="1180" y2="22" stroke="var(--color-rule)" stroke-width="1" />
                    { for chronological.iter().enumerate().map(|(i, s)| {
                        let t = if range > 0 {
                            (s.timestamp - min_ts) as f64 / range as f64
                        } else { 0.0 };
                        let x = 8.0 + t * 1164.0;
                        let is_active = if current_snapshot_str == "latest" {
                            s.timestamp == max_ts
                        } else {
                            s.snapshot_id == current_snapshot_str
                        };
                        let is_hovered = *hovered == Some(i);
                        let stroke = if is_active {
                            "var(--color-theme-500)"
                        } else if is_hovered {
                            "var(--color-paper-1)"
                        } else {
                            "var(--color-paper-4)"
                        };
                        let stroke_w = if is_active || is_hovered { 2 } else { 1 };
                        html! {
                            <>
                                <line
                                    x1={format!("{x}")} y1="14"
                                    x2={format!("{x}")} y2="30"
                                    stroke={stroke}
                                    stroke-width={stroke_w.to_string()}
                                />
                                if is_active {
                                    <circle cx={format!("{x}")} cy="22" r="4" fill="var(--color-theme-500)"/>
                                }
                            </>
                        }
                    }) }
                </svg>

                // Invisible hit targets for hover/click on each snapshot
                <div class="absolute inset-0">
                    { for chronological.iter().enumerate().map(|(i, s)| {
                        let t = if range > 0 {
                            (s.timestamp - min_ts) as f64 / range as f64
                        } else { 0.0 };
                        let left_pct = (8.0 + t * 1164.0) / 1180.0 * 100.0;
                        let snap_id = s.snapshot_id.to_string();
                        let on_click = {
                            let on_change = props.on_change.clone();
                            let id = snap_id.clone();
                            Callback::from(move |_: MouseEvent| on_change.emit(id.clone()))
                        };
                        let on_enter = {
                            let hovered = hovered.clone();
                            Callback::from(move |_: MouseEvent| hovered.set(Some(i)))
                        };
                        let on_leave = {
                            let hovered = hovered.clone();
                            Callback::from(move |_: MouseEvent| hovered.set(None))
                        };
                        html! {
                            <button
                                type="button"
                                aria-label={fmt_date(s.timestamp)}
                                onclick={on_click}
                                onmouseenter={on_enter}
                                onmouseleave={on_leave}
                                class="absolute top-0 h-full w-3 -translate-x-1/2 cursor-pointer border-0 bg-transparent p-0 focus:outline-none"
                                style={format!("left:{left_pct}%")}
                            />
                        }
                    }) }
                </div>

                // Hover tooltip
                {
                    if let Some(idx) = *hovered
                        && let Some(s) = chronological.get(idx)
                    {
                        let t = if range > 0 {
                            (s.timestamp - min_ts) as f64 / range as f64
                        } else { 0.0 };
                        let left_pct = (8.0 + t * 1164.0) / 1180.0 * 100.0;
                        let is_active_tip = if current_snapshot_str == "latest" {
                            s.timestamp == max_ts
                        } else {
                            s.snapshot_id == current_snapshot_str
                        };
                        html! {
                            <div
                                class="pointer-events-none absolute bottom-full z-10 mb-2 -translate-x-1/2 whitespace-nowrap rounded-md border border-rule bg-ink-3 px-3 py-2 shadow-lg"
                                style={format!("left:{left_pct}%")}
                            >
                                <div class="font-mono text-[11px] text-paper-1">
                                    { fmt_date(s.timestamp) }
                                    if is_active_tip {
                                        <span class="text-theme-500">{ " · viewing" }</span>
                                    }
                                </div>
                                <div class="mt-0.5 font-mono text-[11px] text-paper-3">
                                    { format!("{} entries · {} pages", s.total_entries, s.total_pages) }
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>

            <div class="font-mono text-[11px] text-paper-3 flex justify-between mt-1 tracking-[0.06em] uppercase">
                <span>{ first_date }</span>
                <span class="text-theme-500">{ last_date }</span>
            </div>

            // Dropdown selector
            <div class="flex items-center gap-3 mt-4 pt-4 border-t border-rule-soft">
                <label class="eyebrow shrink-0">{"Jump to:"}</label>
                <div class="relative flex-1 max-w-xs">
                    <select
                        value={props.current_snapshot.clone()}
                        {onchange}
                        class={classes!(theme_color, "appearance-none", "w-full", "px-3", "py-2", "pr-9", "bg-ink-2", "border", "border-rule", "rounded-md", "text-sm", "font-mono", "text-paper-1", "cursor-pointer", "focus:outline-none", "focus:border-theme-500/60", "transition-colors")}
                    >
                        {for sorted_snapshots.iter().map(|snap| {
                            let ms = (snap.timestamp * 1000) as f64;
                            let d = Date::new(&JsValue::from_f64(ms));
                            let formatted: String = d.to_locale_date_string(&locale, &date_formats).into();
                            html! {
                                <option
                                    value={snap.snapshot_id.to_string()}
                                    selected={props.current_snapshot == snap.snapshot_id.as_str()}
                                >
                                    {
                                        if snap.snapshot_id == "latest" {
                                            format!("Latest ({formatted})")
                                        } else {
                                            formatted
                                        }
                                    }
                                </option>
                            }
                        })}
                    </select>
                    <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3 text-paper-4">
                        <svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
                        </svg>
                    </div>
                </div>
            </div>
        </div>
    }
}
