//! Choosing which archived state of a board to look at.

use crate::hooks::use_theme;
use crate::route::Snapshot;
use dioxus::prelude::*;
use mp_stats_core::models::{HistoricalSnapshot, LeaderboardMeta};
use std::cmp::Reverse;
use web_sys::js_sys::Date;
use web_sys::js_sys::Intl::DateTimeFormatOptions;
use web_sys::wasm_bindgen::JsValue;

/// The width the timeline is drawn in, and the inset that keeps the first and last tick from
/// being clipped by the edge of it. Both are `viewBox` units rather than pixels: the SVG is
/// stretched to whatever width the card has.
const TIMELINE_WIDTH: f64 = 1180.0;
/// The horizontal padding inside [`TIMELINE_WIDTH`] that ticks are laid out within.
const TIMELINE_INSET: f64 = 8.0;
/// The span the ticks are spread across, which is the width less the inset at both ends.
const TIMELINE_SPAN: f64 = TIMELINE_WIDTH - 2.0 * TIMELINE_INSET;

/// The reader's locale, or `en-US` where the browser will not say.
fn browser_locale() -> String {
    web_sys::window()
        .map(|window| window.navigator())
        .and_then(|navigator| navigator.language())
        .unwrap_or_else(|| "en-US".to_string())
}

/// One snapshot's timestamp as a date in `locale`.
fn format_date(timestamp: u64, locale: &str) -> String {
    #[allow(clippy::cast_precision_loss)]
    let milliseconds = (timestamp * 1000) as f64;

    Date::new(&JsValue::from_f64(milliseconds))
        .to_locale_date_string(locale, &DateTimeFormatOptions::new())
        .into()
}

/// Where along the timeline a snapshot taken at `timestamp` sits, as a `viewBox` x coordinate.
///
/// Placed by timestamp rather than evenly, so a decade of dumps taken at irregular intervals reads
/// as the gaps it really has.
#[allow(clippy::cast_precision_loss)]
fn tick_x(timestamp: u64, min_ts: u64, range: u64) -> f64 {
    let fraction = (timestamp - min_ts) as f64 / range as f64;

    TIMELINE_INSET + fraction * TIMELINE_SPAN
}

/// Whether `snapshot` is the one being viewed, given that "latest" names whichever is newest
/// rather than an id of its own.
fn is_viewing(snapshot: &HistoricalSnapshot, current: &Snapshot, max_ts: u64) -> bool {
    if current.is_latest() {
        snapshot.timestamp == max_ts
    } else {
        snapshot.snapshot_id == current.as_str()
    }
}

/// A timeline of every snapshot of this board, with a dropdown under it.
///
/// Dates are formatted in the browser's locale. `current_snapshot` is the one being viewed, and
/// `meta` is the board's metadata - absent or empty renders nothing at all, which is how a board
/// with no history hides the control. `on_change` is called with the snapshot chosen.
///
/// The theme comes from the route, so there is no `edition` prop to keep in step with it.
#[component]
pub fn SnapshotSelector(
    current_snapshot: Snapshot,
    meta: Option<LeaderboardMeta>,
    on_change: EventHandler<Snapshot>,
) -> Element {
    let theme_color = use_theme();
    let mut hovered = use_signal(|| Option::<usize>::None);

    // After the hooks, never before: a board with no history still has to run them, or the next
    // board the reader opens reads this component's hook state at the wrong offsets.
    let Some(meta) = meta.filter(|meta| !meta.snapshots.is_empty()) else {
        return rsx! {};
    };

    let locale = browser_locale();

    // Ascending for the timeline, descending for the dropdown: the timeline reads left to right
    // through history, and the dropdown offers the most recent dump first.
    let mut chronological = meta.snapshots.clone();
    chronological.sort_by_key(|snapshot| snapshot.timestamp);

    let mut most_recent_first = meta.snapshots.clone();
    most_recent_first.sort_by_key(|snapshot| Reverse(snapshot.timestamp));

    let total = chronological.len();
    let min_ts = chronological
        .first()
        .map_or(0, |snapshot| snapshot.timestamp);
    let max_ts = chronological
        .last()
        .map_or(1, |snapshot| snapshot.timestamp);
    let range = (max_ts - min_ts).max(1);

    let first_date = format_date(min_ts, &locale);
    let last_date = format_date(max_ts, &locale);

    let active_label = chronological
        .iter()
        .find(|snapshot| is_viewing(snapshot, &current_snapshot, max_ts))
        .map_or_else(
            || "\u{2014}".to_string(),
            |snapshot| {
                let date = format_date(snapshot.timestamp, &locale);

                if current_snapshot.is_latest() {
                    format!("Latest \u{b7} {date}")
                } else {
                    date
                }
            },
        );

    rsx! {
        div { class: "{theme_color} card p-4",
            div { class: "flex items-baseline justify-between mb-3 gap-3 flex-wrap",
                div { class: "eyebrow", "Snapshot \u{b7} {total} archived" }
                div { class: "font-mono text-[11px] text-paper-3",
                    "Viewing "
                    span { class: "text-theme-500", "{active_label}" }
                }
            }

            // Tickmark timeline
            div { class: "relative w-full",
                svg {
                    view_box: "0 0 1180 36",
                    preserve_aspect_ratio: "none",
                    class: "block w-full h-9",
                    line {
                        x1: "0",
                        y1: "22",
                        x2: "1180",
                        y2: "22",
                        stroke: "var(--color-rule)",
                        stroke_width: "1",
                    }
                    for (index , snapshot) in chronological.iter().enumerate() {
                        {
                            let x = tick_x(snapshot.timestamp, min_ts, range);
                            let is_active = is_viewing(snapshot, &current_snapshot, max_ts);
                            let is_hovered = hovered() == Some(index);

                            rsx! {
                                line {
                                    key: "{snapshot.snapshot_id}",
                                    x1: "{x}",
                                    y1: "14",
                                    x2: "{x}",
                                    y2: "30",
                                    stroke: if is_active {
                                        "var(--color-theme-500)"
                                    } else if is_hovered {
                                        "var(--color-paper-1)"
                                    } else {
                                        "var(--color-paper-4)"
                                    },
                                    stroke_width: if is_active || is_hovered { "2" } else { "1" },
                                }
                                if is_active {
                                    circle {
                                        cx: "{x}",
                                        cy: "22",
                                        r: "4",
                                        fill: "var(--color-theme-500)",
                                    }
                                }
                            }
                        }
                    }
                }

                // Invisible hit targets for hover/click on each snapshot
                div { class: "absolute inset-0",
                    for (index , snapshot) in chronological.iter().enumerate() {
                        {
                            let left_pct = tick_x(snapshot.timestamp, min_ts, range)
                                / TIMELINE_WIDTH * 100.0;
                            let snapshot_id = snapshot.snapshot_id.to_string();

                            rsx! {
                                button {
                                    key: "{snapshot.snapshot_id}",
                                    r#type: "button",
                                    aria_label: {format_date(snapshot.timestamp, &locale)},
                                    class: "absolute top-0 h-full w-3 -translate-x-1/2 cursor-pointer border-0 bg-transparent p-0 focus:outline-none",
                                    style: "left:{left_pct}%",
                                    onclick: move |_| {
                                        on_change.call(snapshot_id.parse().unwrap_or_default());
                                    },
                                    onmouseenter: move |_| hovered.set(Some(index)),
                                    onmouseleave: move |_| hovered.set(None),
                                }
                            }
                        }
                    }
                }

                // Hover tooltip
                if let Some(snapshot) = hovered().and_then(|index| chronological.get(index)) {
                    {
                        let left_pct = tick_x(snapshot.timestamp, min_ts, range) / TIMELINE_WIDTH
                            * 100.0;

                        rsx! {
                            div {
                                class: "pointer-events-none absolute bottom-full z-10 mb-2 -translate-x-1/2 whitespace-nowrap rounded-md border border-rule bg-ink-3 px-3 py-2 shadow-lg",
                                style: "left:{left_pct}%",
                                div { class: "font-mono text-[11px] text-paper-1",
                                    {format_date(snapshot.timestamp, &locale)}
                                    if is_viewing(snapshot, &current_snapshot, max_ts) {
                                        span { class: "text-theme-500", " \u{b7} viewing" }
                                    }
                                }
                                div { class: "mt-0.5 font-mono text-[11px] text-paper-3",
                                    "{snapshot.total_entries} entries \u{b7} {snapshot.total_pages} pages"
                                }
                            }
                        }
                    }
                }
            }

            div {
                class: "font-mono text-[11px] text-paper-3 flex justify-between mt-1 tracking-[0.06em] uppercase",
                span { "{first_date}" }
                span { class: "text-theme-500", "{last_date}" }
            }

            // Dropdown selector
            div { class: "flex items-center gap-3 mt-4 pt-4 border-t border-rule-soft",
                label { class: "eyebrow shrink-0", "Jump to:" }
                div { class: "relative flex-1 max-w-xs",
                    select {
                        class: "{theme_color} appearance-none w-full px-3 py-2 pr-9 bg-ink-2 border border-rule rounded-md text-sm font-mono text-paper-1 cursor-pointer focus:outline-none focus:border-theme-500/60 transition-colors",
                        value: "{current_snapshot}",
                        onchange: move |event| {
                            on_change.call(event.value().parse().unwrap_or_default());
                        },
                        for snapshot in most_recent_first.iter() {
                            {
                                let formatted = format_date(snapshot.timestamp, &locale);
                                let id = snapshot.snapshot_id.as_str();

                                rsx! {
                                    option {
                                        key: "{id}",
                                        value: "{id}",
                                        selected: current_snapshot.as_str() == id,
                                        if id == "latest" {
                                            "Latest ({formatted})"
                                        } else {
                                            "{formatted}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "pointer-events-none absolute inset-y-0 right-0 flex items-center px-3 text-paper-4",
                        svg {
                            class: "h-3.5 w-3.5",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            stroke_width: "2",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M19 9l-7 7-7-7",
                            }
                        }
                    }
                }
            }
        }
    }
}
