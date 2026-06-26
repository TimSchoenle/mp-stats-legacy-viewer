use crate::Route;
use crate::components::search_bar::SearchBar;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

fn edition_card(edition: &PlatformEdition) -> Html {
    let desc = match edition {
        PlatformEdition::Java => "Top 1,000 entries per category.",
        PlatformEdition::Bedrock => "Top 100 entries, mostly win statistics.",
    };

    let theme_class = match edition {
        PlatformEdition::Java => "theme-java",
        PlatformEdition::Bedrock => "theme-bedrock",
    };

    html! {
        <Link<Route>
            to={Route::Landing { edition: edition.clone() }}
            classes={classes!(theme_class, "card", "p-7", "relative", "overflow-hidden", "group", "block")}
        >
            <div
                class="absolute top-0 left-0 right-0 h-[3px] bg-theme-500"
            ></div>
            <div class="flex items-start justify-between">
                <div class="serif text-4xl text-paper-1 tracking-tight">
                    { format!("{} Edition", edition.display_name()) }
                </div>
                <div class="font-mono text-[11px] text-paper-4 group-hover:text-theme-400 transition-colors">
                    { "→" }
                </div>
            </div>
            <div class="mt-4 text-sm text-paper-3">
                { desc }
            </div>
        </Link<Route>>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="theme-amber relative">
            // Hero
            <div class="container mx-auto px-6 pt-16 pb-12 max-w-6xl">
                <div class="eyebrow mb-5">{"Archive · snapshots 2021 → Jan 2023"}</div>
                <h1 class="serif page-title text-6xl md:text-7xl lg:text-8xl text-paper-1 max-w-4xl">
                    { "The legacy stats" }
                    <br/>
                    <span class="italic" style="color: var(--color-brand-amber-500);">{ "archive" }</span>
                    { ", kept whole." }
                </h1>
                <p class="text-base text-paper-3 max-w-xl mt-6 leading-relaxed">
                    { "A permanent record of player rankings collected by the StatsBot Project until mid-January 2023. Browse leaderboards by game, step through snapshots, and look up any player who appeared in one." }
                </p>
            </div>

            // Search bar
            <div class="container mx-auto px-6 max-w-6xl mb-10">
                <SearchBar
                    class={classes!("w-full", "max-w-none")}
                    input_classes={classes!("py-5", "pl-14", "pr-16", "text-base", "rounded-xl")}
                />
            </div>

            // Edition cards
            <div class="container mx-auto px-6 max-w-6xl mb-10">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    { for PlatformEdition::iter().map(edition_card) }
                </div>
            </div>

            // Archive notice
            <div class="container mx-auto px-6 max-w-6xl pb-20">
                <div
                    class="card p-6 max-w-2xl"
                    style="border-color: color-mix(in oklch, var(--color-brand-amber-500), transparent 70%);"
                >
                    <div class="eyebrow mb-3" style="color: var(--color-brand-amber-500);">
                        {"⚠ Archive notice"}
                    </div>
                    <p class="text-sm text-paper-2 leading-relaxed">
                        { "This is an archival mirror. Some profiles will appear incomplete or contain historical inaccuracies. Data is preserved exactly as collected — without edits, corrections, or backfills." }
                    </p>
                    <ul class="mt-4 text-sm text-paper-3 space-y-1.5 list-disc list-outside pl-5 marker:text-paper-4">
                        <li>
                            <span class="text-paper-1 font-medium">{"Java Edition"}</span>
                            { " stats are mostly limited to the top 1,000 ranking entries per category." }
                        </li>
                        <li>
                            <span class="text-paper-1 font-medium">{"Bedrock Edition"}</span>
                            { " stats are mostly limited to the top 100 entries, and predominantly feature only " }
                            <span class="italic">{"win"}</span>
                            { " statistics." }
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
