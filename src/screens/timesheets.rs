use leptos::*;
use leptos_router::*;

/// Renders the home page of your application.
#[component]
pub fn TimeSheets() -> impl IntoView {
    view! {
        <nav class="subWrapper">
            <A href="" exact=true>"Time Sheets"</A>
            <A href="adjustment" exact=true>"Add Adjustment"</A>
            <A href="pending" exact=true>"Pending Corrections"</A>
        </nav>
        <section class="stack">
            <Outlet />
        </section>
    }
}

#[component]
pub fn TimeSheetsList() -> impl IntoView {
    view! {
        <h1>"TimeSheets | To Do"</h1>
    }
}

#[component]
pub fn TimeSheetsAdjustment() -> impl IntoView {
    view! {
        <h1>"Adjustment | To Do"</h1>
    }
}

#[component]
pub fn TimeSheetsPending() -> impl IntoView {
    view! {
        <h1>"TimeSheets | To Do"</h1>
    }
}
