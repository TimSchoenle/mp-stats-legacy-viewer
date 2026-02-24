use crate::Route;
use crate::models::LeaderboardEntry;
use crate::hooks::use_theme;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LeaderboardTableProps {
    pub entries: Vec<LeaderboardEntry>,
    pub edition: PlatformEdition,
}

#[function_component(LeaderboardTable)]
pub fn leaderboard_table(props: &LeaderboardTableProps) -> Html {
    let theme_color = use_theme();

    html! {
        <div class="overflow-x-auto">
            <table class="w-full text-left border-collapse">
                <thead>
                    <tr>
                        <th class="table-header w-20 text-center">{ "Rank" }</th>
                        <th class="table-header">{ "Player" }</th>
                        <th class="table-header text-right">{ "Score" }</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-white/5 bg-dark-900 border-b border-white/5">
                    { for props.entries.iter().map(|row| {
                        html! {
                        <tr class="table-row group">
                            <td class="table-cell text-center">
                                <span class={classes!(theme_color, "rank-badge", "inline-block", "font-bold", "text-gray-400", "group-hover:text-theme-400", "group-hover:border-theme-500/30", "transition-colors")}>
                                    { format!("#{}", row.rank) }
                                </span>
                            </td>
                            <td class="table-cell">
                                <Link<Route> to={Route::Player { edition: props.edition.clone(), uuid: row.uuid.to_string() }} classes="flex items-center gap-3 w-fit group/link">
                                    <img
                                        src={format!("https://mc-heads.net/avatar/{}/32", row.uuid)}
                                        class={classes!(theme_color, "w-8", "h-8", "rounded-md", "bg-dark-950", "shadow-sm", "border", "border-white/10", "group-hover/link:border-theme-500/50", "transition-colors")}
                                        alt="Avatar"
                                        loading="lazy"
                                    />
                                    <span class={classes!(theme_color, "font-bold", "text-gray-200", "group-hover/link:text-theme-400", "transition-colors", "font-mono", "tracking-tight")}>
                                        { row.name.as_str() }
                                    </span>
                                </Link<Route>>
                            </td>
                            <td class="table-cell text-right font-mono font-bold text-lg text-white">
                                { row.score }
                            </td>
                        </tr>
                    }}) }
                </tbody>
            </table>
        </div>
    }
}
