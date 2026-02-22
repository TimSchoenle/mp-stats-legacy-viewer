use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ErrorMessageProps {
    pub title: String,
    pub message: String,
    #[prop_or(false)]
    pub is_banner: bool,
}

#[function_component(ErrorMessage)]
pub fn error_message(props: &ErrorMessageProps) -> Html {
    if props.is_banner {
        html! {
            <div class="bg-red-900/30 border border-red-700/50 text-red-200 p-4 rounded-lg mb-6 backdrop-blur-sm">
                <h3 class="font-bold flex items-center gap-2 text-red-400">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
                    </svg>
                    { &props.title }
                </h3>
                <p class="mt-1 opacity-90 text-sm">{ &props.message }</p>
            </div>
        }
    } else {
        html! {
            <div class="card p-8 text-center text-gray-500">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 mx-auto mb-3 text-red-500/70" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                </svg>
                <p class="font-medium text-gray-300">{ &props.title }</p>
                <p class="text-sm mt-1 text-gray-500">{ &props.message }</p>
            </div>
        }
    }
}
