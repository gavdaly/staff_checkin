use crate::models::adjustments::Adjustment;
use crate::models::assignation::Assignation;
use crate::models::corrections::Correction;
use crate::models::user::{Role, State as UserState, UserPublic};
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
    Ok(vec![])
}

#[server]
async fn load_hourly_users() -> Result<Vec<UserPublic>, ServerFnError> {
    match UserPublic::get_all_hourly().await {
        Ok(v) => Ok(v),
        Err(e) => Err(ServerFnError::ServerError("Server Error".to_string())),
    }
}

#[component]
pub fn TimeSheetsList() -> impl IntoView {
    let timesheets = create_resource(move || {}, move |_| load_hourly_users());
    // let user_view = |user| {
    //     view! {<div id={user.id.to_string()}>{user.last_name}, {user.first_name}</div>}
    // };

    view! {
        <Suspense  fallback=move || view! { <p>"Loading..."</p> }>
            {move || match timesheets.get() {
                Some(Ok(a)) => view!{
                    <div>
                    <label for="user_selected"></label>
                    <select name="user_selected" id="user_selected">
                        <option />
                        {a.iter().map(|user| { view! {
                        <option value={user.id.to_string()}>
                            {user.last_name.clone()}", "{user.first_name.clone()}
                        </option>} }).collect_view()
                    }</select>
                    <button type="submit">Switch User</button>
                    </div>},
                _ => view!{<div>"Server Error"</div>},
            }}
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
