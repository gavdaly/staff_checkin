use crate::models::adjustments::Adjustment;
use crate::models::assignation::Assignation;
use crate::models::corrections::Correction;
use crate::models::user::{Role, State as UserState};
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct TimeSheet {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub display_name: String,
    pub state: UserState,
    pub role: Role,
    pub assignations: Vec<Assignation>,
    pub corrections: Vec<Correction>,
    pub adjustments: Vec<Adjustment>,
}

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

#[server]
async fn load_timesheets_data() -> Result<Vec<TimeSheet>, ServerFnError> {
    use crate::utils::db;

    let db = db().await?;

    Ok(vec![])
}

#[component]
pub fn TimeSheetsList() -> impl IntoView {
    let timesheets = create_resource(move || {}, move |_| load_timesheets_data());
    view! {
        <Suspense  fallback=move || view! { <p>"Loading..."</p> }>
            {move || timesheets.get().map(|ts| {

            })}
        </Suspense>
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
