use crate::Route;
use crate::hooks::use_theme;
use crate::models::LeaderboardEntry;
use crate::util::percent::format_percent;
use crate::util::score_formatter::create_score_formatter;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LeaderboardTableProps {
    pub game: String,
    pub stat: String,
    pub entries: Vec<LeaderboardEntry>,
    pub edition: PlatformEdition,
}

#[function_component(LeaderboardTable)]
pub fn leaderboard_table(props: &LeaderboardTableProps) -> Html {
    let theme_color = use_theme();
    let score_formatter = create_score_formatter(&props.game, &props.stat);

    // Top score for "vs. #1" bar
    let top_score = props.entries.first().map(|e| e.score).unwrap_or(0);

    html! {
        <div class={classes!(theme_color, "overflow-x-auto")}>
            <table class="w-full text-left border-collapse">
                <thead>
                    <tr>
                        <th class="table-header w-20">{ "Rank" }</th>
                        <th class="table-header">{ "Player" }</th>
                        <th class="table-header w-72 hidden md:table-cell">{ "vs. #1" }</th>
                        <th class="table-header text-right">{ "Score" }</th>
                    </tr>
                </thead>
                <tbody>
                    { for props.entries.iter().map(|row| {
                        let is_top3 = row.rank <= 3;
                        let pct = if top_score > 0 {
                            ((row.score as f64 / top_score as f64) * 100.0).min(100.0).max(0.0)
                        } else {
                            0.0
                        };
                        let bar_color = if is_top3 { "var(--color-theme-500)" } else { "var(--color-paper-3)" };
                        let bar_style = format!("width:{:.2}%; background:{};", pct, bar_color);

                        let rank_class = if is_top3 {
                            "font-mono text-sm font-semibold text-theme-500"
                        } else {
                            "font-mono text-sm text-paper-3"
                        };

                        html! {
                        <tr class="table-row group">
                            <td class="table-cell">
                                <span class={rank_class}>
                                    { format!("#{}", row.rank) }
                                </span>
                            </td>
                            <td class="table-cell">
                                <Link<Route> to={Route::Player { edition: props.edition.clone(), uuid: row.uuid.to_string() }} classes="flex items-center gap-3 w-fit group/link">
                                    <img
                                        src={format!("https://mc-heads.net/avatar/{}/32", row.uuid)}
                                        class="w-6 h-6 rounded bg-ink-3 border border-rule"
                                        alt="Avatar"
                                        loading="lazy"
                                    />
                                    <span class="font-mono text-sm font-medium text-paper-1 group-hover/link:text-theme-400 transition-colors">
                                        { row.name.as_str() }
                                    </span>
                                </Link<Route>>
                            </td>
                            <td class="table-cell hidden md:table-cell">
                                <div class="flex items-center gap-3 max-w-[16rem]">
                                    <span class="bar-track flex-1">
                                        <span class="bar-fill" style={bar_style}></span>
                                    </span>
                                    <span class="font-mono text-xs text-paper-3 tnum w-10 text-right">
                                        { format_percent(pct) }
                                    </span>
                                </div>
                            </td>
                            <td class="table-cell text-right font-mono font-medium text-paper-1 tnum">
                                { score_formatter.format_score(row.score) }
                            </td>
                        </tr>
                    }}) }
                </tbody>
            </table>
        </div>
    }
}
