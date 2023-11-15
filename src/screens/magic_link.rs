use leptos::*;
use leptos_router::*;

#[derive(Clone, Params, PartialEq)]
struct MagicLinkParams {
    link: String,
}

#[component]
pub fn MagicLink() -> impl IntoView {
    let params = use_params::<MagicLinkParams>();

    view! {
        <div>"Loading..."</div>
    }
}