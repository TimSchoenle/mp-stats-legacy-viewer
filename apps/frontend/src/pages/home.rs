use crate::components::search_bar::SearchBar;
use crate::hooks::use_theme::get_theme_color;
use crate::Route;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

fn leaderboard_button(edition: &PlatformEdition) -> Html {
    let theme_color = get_theme_color(edition);

    let box_styling = classes!(
        theme_color,
        "group", "flex-1", "p-8", "border", "rounded-2xl", "shadow-xl",
        "transition-all", "duration-300", "transform", "hover:-translate-y-1",
        "flex", "flex-col", "items-center", "justify-center",
        "bg-theme-900/40", "hover:bg-theme-800/60", "border-theme-500/30"
    );

    html! {
        <Link<Route> to={Route::Landing {edition: edition.clone()}} classes={box_styling} >
            <h2 class={classes!("text-3xl", "font-bold", "group-hover:text-white", "transition-colors", "text-theme-50")}>
                { format!("{} Edition", edition.display_name()) }
            </h2>
            <p class={classes!("mt-2", "text-sm", "transition-colors", "text-theme-200/70", "group-hover:text-theme-200")}>
                {format!("View {} Leaderboards", edition.display_name()) }
            </p>
        </Link<Route>>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen bg-dark-950 text-white p-6 relative overflow-hidden">
            <div class="relative z-10 w-full flex flex-col items-center">
                <h1 class="text-5xl md:text-6xl font-bold mb-4 text-center tracking-tight">
                    <span class="text-white">{"MP Stats Legacy Viewer"}</span>
                </h1>
                <p class="text-xl mb-12 text-gray-400 text-center max-w-2xl">
                    {"The complete historical stats archive created by the StatsBot Project."}
                </p>

                <div class="w-full max-w-5xl glass-panel p-8 md:p-10 mb-12">
                    <div class="flex flex-col md:flex-row gap-10">
                        <div class="flex-1 space-y-5">
                            <h2 class="text-2xl font-bold text-white flex items-center mb-6">
                                <svg class="w-6 h-6 mr-3 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                { "About the Archive" }
                            </h2>
                            <div class="text-gray-300 space-y-4 text-lg leading-relaxed">
                                <p>
                                    { "This platform serves as a permanent record of legacy player statistics, with data actively collected up until "}
                                    <strong class="text-white font-semibold">{"mid-January 2023"}</strong>{". " }
                                    { "The data presented is an authentic reflection of its source, provided without any edits or alterations." }
                                </p>
                            </div>
                        </div>

                        <div class="flex-1">
                            <div class="h-full bg-orange-950/30 border border-orange-500/20 rounded-xl p-6 relative overflow-hidden shadow-inner">
                                <div class="absolute top-0 left-0 w-1 h-full bg-orange-500"></div>
                                <h3 class="text-xl font-bold text-orange-400 mb-4 flex items-center">
                                    <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
                                    { "Data Limitations" }
                                </h3>
                                <ul class="list-disc list-outside ml-5 text-orange-200/80 space-y-3 text-base">
                                    <li><span class="font-semibold text-orange-300">{"Java Edition"}</span>{" stats are mostly limited to the top 1,000 ranking entries per category."}</li>
                                    <li><span class="font-semibold text-orange-300">{"Bedrock Edition"}</span>{" stats are mostly limited to the top 100 entries, and predominantly feature only "}<span class="italic">{"win"}</span>{" statistics."}</li>
                                    <li>{"Due to the raw archival nature of these dumps, you may encounter missing profiles, incomplete histories, or other historical inaccuracies."}</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="w-full max-w-3xl mb-12 flex justify-center relative z-20">
                    <SearchBar class={classes!("w-full", "max-w-none")} input_classes={classes!("py-5", "pl-6", "pr-14", "text-xl", "shadow-xl")} />
                </div>

                <div class="flex flex-col sm:flex-row gap-6 w-full max-w-3xl justify-center">
                    {for PlatformEdition::iter().map(leaderboard_button)}
                </div>
            </div>
        </div>
    }
}
