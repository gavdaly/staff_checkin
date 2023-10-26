#[component]
pub fn Loading() -> impl IntoView {
    view! { <div data-state="loading">"Loading..."</div> }
}
