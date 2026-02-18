use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
}

#[function_component(Leaderboard)]
pub fn leaderboard(props: &Props) -> Html {
    html! {
        <div class="container mx-auto p-4">
            <h2 class="text-xl font-bold mb-4">{ &props.title }</h2>
            <div class="overflow-x-auto">
                <table class="min-w-full bg-gray-800 text-white rounded-lg overflow-hidden">
                    <thead class="bg-gray-700">
                        <tr>
                            <th class="p-3 text-left">{ "Rank" }</th>
                            <th class="p-3 text-left">{ "Name/UUID" }</th>
                            <th class="p-3 text-left">{ "Score" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        // Rows will be populated here
                        <tr>
                            <td class="p-3" colspan="3">{ "Loading..." }</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
