use crate::components::search_bar::SearchBar;
use crate::Route;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white p-6">
            <h1 class="text-5xl font-bold mb-4 text-center">{ "MP Stats Legacy Viewer" }</h1>
            <p class="text-xl mb-10 text-gray-400 text-center">{"MP Stats archive viewer based of the StatsBot data."}</p>

            <div class="w-full max-w-5xl bg-gray-800/80 rounded-2xl shadow-2xl p-8 md:p-10 mb-12 border border-gray-700/50 backdrop-blur-sm">
                <div class="flex flex-col md:flex-row gap-8">
                    <div class="flex-1 space-y-4">
                        <h2 class="text-3xl font-bold text-blue-400 flex items-center mb-6">
                            <svg class="w-8 h-8 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                            { "About the Data" }
                        </h2>
                        <div class="text-gray-300 space-y-4 text-lg leading-relaxed">
                            <p>
                                { "This is the complete historical stats record created by the StatsBot Project. " }
                            </p>
                            <p>
                                { "The project actively collected data up until " }<strong class="text-white font-semibold">{"mid-January 2023"}</strong>{", serving as a permanent record of legacy player statistics." }
                            </p>
                            <p class="text-gray-400 text-sm italic mt-4 pt-4 border-t border-gray-700/50">
                                { "Note: The data presented has not been edited, filtered, or altered in any way, authentically reflecting its original source." }
                            </p>
                        </div>
                    </div>

                    <div class="flex-1">
                        <div class="h-full bg-yellow-900/20 border border-yellow-700/30 rounded-xl p-6 relative overflow-hidden">
                            <div class="absolute top-0 left-0 w-1 h-full bg-yellow-500"></div>
                            <h3 class="text-xl font-bold text-yellow-500 mb-4 flex items-center">
                                <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
                                { "Data Limitations Warning" }
                            </h3>
                            <ul class="list-disc list-outside ml-5 text-yellow-100/80 space-y-3 text-base">
                                <li><span class="font-semibold text-yellow-300">{"Java Edition"}</span>{" stats are mostly limited to the top 1,000 ranking entries per category."}</li>
                                <li><span class="font-semibold text-yellow-300">{"Bedrock Edition"}</span>{" stats are mostly limited to the top 100 entries, and predominantly feature only "}<span class="italic">{"win"}</span>{" statistics."}</li>
                                <li>{"Due to the raw archival nature of these dumps, you may encounter missing profiles, incomplete histories, or other historical inaccuracies."}</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>

            <div class="w-full max-w-3xl mb-12 flex justify-center">
                <SearchBar class={classes!("w-full", "max-w-none")} input_classes={classes!("py-5", "pl-6", "pr-14", "text-xl")} />
            </div>

            <div class="flex flex-col sm:flex-row gap-6 w-full max-w-3xl justify-center">
                <Link<Route> to={Route::Landing {edition: PlatformEdition::Java}} classes="flex-1 p-10 bg-green-700/80 hover:bg-green-600 border border-green-500/30 rounded-2xl shadow-xl transition-all duration-300 transform hover:-translate-y-2 flex flex-col items-center justify-center group">
                    <h2 class="text-4xl font-bold text-green-50 group-hover:text-white transition-colors">{ "Java Edition" }</h2>
                    <p class="text-green-200/70 mt-3 text-base group-hover:text-green-200 transition-colors">{"View Java Leaderboards"}</p>
                </Link<Route>>

                <Link<Route> to={Route::Landing {edition: PlatformEdition::Bedrock}} classes="flex-1 p-10 bg-blue-700/80 hover:bg-blue-600 border border-blue-500/30 rounded-2xl shadow-xl transition-all duration-300 transform hover:-translate-y-2 flex flex-col items-center justify-center group">
                    <h2 class="text-4xl font-bold text-blue-50 group-hover:text-white transition-colors">{ "Bedrock Edition" }</h2>
                    <p class="text-blue-200/70 mt-3 text-base group-hover:text-blue-200 transition-colors">{"View Bedrock Leaderboards"}</p>
                </Link<Route>>
            </div>
        </div>
    }
}
