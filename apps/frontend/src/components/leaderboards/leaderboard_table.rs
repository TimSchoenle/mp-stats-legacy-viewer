use crate::Route;
use crate::models::LeaderboardEntry;
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
    html! {
        <div class="overflow-x-auto">
            <table class="w-full text-left border-collapse">
                <thead>
                    <tr class="bg-gray-800 border-b border-gray-700 text-gray-400 text-sm uppercase tracking-wider">
                        <th class="p-4 font-medium w-16">{ "Rank" }</th>
                        <th class="p-4 font-medium">{ "Player" }</th>
                        <th class="p-4 font-medium text-right">{ "Score" }</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-gray-700">
                    { for props.entries.iter().map(|row| {
                        html! {
                        <tr class="hover:bg-gray-800 transition-colors group">
                            <td class="p-4 font-bold text-gray-500 group-hover:text-emerald-400 transition-colors">
                                { format!("#{}", row.rank) }
                            </td>
                            <td class="p-4">
                                    <Link<Route> to={Route::Player { edition: props.edition.clone(), uuid: row.uuid.to_string() }} classes="flex items-center gap-3 group/link">
                                    <img
                                        src={format!("https://mc-heads.net/avatar/{}/32", row.uuid)}
                                        class="w-8 h-8 rounded bg-gray-900 shadow-sm"
                                        alt="Avatar"
                                        loading="lazy"
                                    />
                                    <span class="font-bold text-gray-200 group-hover/link:text-emerald-400 transition-colors font-mono tracking-tight">
                                        { row.name.as_str() }
                                    </span>
                                </Link<Route>>
                            </td>
                            <td class="p-4 text-right font-mono font-bold text-lg text-white">
                                { row.score }
                            </td>
                        </tr>
                    }}) }
                </tbody>
            </table>
        </div>
    }
}
