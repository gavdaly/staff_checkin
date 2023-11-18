use leptos::*;

#[component]
pub fn Icon<'a>(name: &'a str) -> impl IntoView {
    view! {
        <svg class=format!("icon {name}")>
            <use_ href=format!("/icons.svg#{name}")></use_>
        </svg>
    }
}
