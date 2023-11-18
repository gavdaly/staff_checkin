use leptos::*;

#[component]
pub fn Icon<'a>(name: &'a str) -> impl IntoView {
    view!{
        <svg class="icon"><use_ href=format!("/icons.svg#{name}")/></svg>
    }
}