use leptos::*;
use super::icon::Icon;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div data-state="loading">
            <Icon name="loading"/>
            <span>"Loading..."</span>
        </div>
    }
}
