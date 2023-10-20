use leptos::*;
use leptos_router::*;

/// Renders the home page of your application.
#[component]
pub fn TimeSheet() -> impl IntoView {
    view! {
        <section class="stack">
            <Outlet />
        </section>
    }
}

#[component]
pub fn TimeSheetDisplay() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <h1>"TimeSheet | To Do"</h1>
    }
}

#[component]
pub fn TimeSheetMissing() -> impl IntoView {
    view! {
        <h1>"Missing"</h1>
    }
}
