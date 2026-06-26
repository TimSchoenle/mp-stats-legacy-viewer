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
            <div class="card mb-6 p-5" style="border-color: color-mix(in oklch, var(--color-brand-rose-500), transparent 60%);">
                <div class="eyebrow mb-2" style="color: var(--color-brand-rose-500);">{"⚠ "} { &props.title }</div>
                <p class="text-sm text-paper-2 leading-relaxed">{ &props.message }</p>
            </div>
        }
    } else {
        html! {
            <div class="card p-12 text-center">
                <div class="eyebrow mb-4" style="color: var(--color-brand-rose-500);">{"⚠ Error"}</div>
                <p class="serif text-2xl text-paper-1 mb-2">{ &props.title }</p>
                <p class="text-sm text-paper-3">{ &props.message }</p>
            </div>
        }
    }
}
