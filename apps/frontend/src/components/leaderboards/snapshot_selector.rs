use mp_stats_core::models::LeaderboardMeta;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SnapshotSelectorProps {
    pub current_snapshot: String,
    pub meta: Option<LeaderboardMeta>,
    pub on_change: Callback<String>,
}

#[function_component(SnapshotSelector)]
pub fn snapshot_selector(props: &SnapshotSelectorProps) -> Html {
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

    html! {
        <div class="flex flex-col gap-1">
            <label class="text-xs text-gray-400 font-medium">{"Snapshot:"}</label>
            <select
                value={props.current_snapshot.clone()}
                {onchange}
                class="px-3 py-2 bg-gray-800 border border-gray-700 hover:border-gray-600 rounded-lg text-sm text-white cursor-pointer focus:outline-none focus:border-emerald-500 transition-colors"
            >
                <option value="latest" selected={props.current_snapshot == "latest"}>
                    {"Latest"}
                </option>
                {for meta.snapshots.iter().map(|snap| {
                    html! {
                        <option
                            value={snap.snapshot_id.to_string()}
                            selected={props.current_snapshot == snap.snapshot_id.as_str()}
                        >
                            {snap.snapshot_id.to_string()}
                        </option>
                    }
                })}
            </select>
        </div>
    }
}
