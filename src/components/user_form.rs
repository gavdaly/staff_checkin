use leptos::*;

pub struct UserF {}

#[component]
pub fn UserForm() -> impl IntoView {
    view! {
        <form>
            <input />
            <input />
        </form>
    }
}
