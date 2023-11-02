use leptos::*;
use leptos_router::*;
use crate::models::time_sheets::TimeSheet;
use crate::components::timesheet::TimeSheetDisplay as TSD;

/// Renders the home page of your application.
#[component]
pub fn TimeSheetIndex() -> impl IntoView {
    view! {
        <section class="stack">
            <Outlet/>
        </section>
    }
}

#[component]
pub fn TimeSheetDisplay() -> impl IntoView {
    let timesheet = create_resource(|| {},  |_| {get_active_user_timesheet()});
    {view! {
        <section class="stack">
            {match timesheet() {
                Some(Ok(timesheet)) => {
                    view! {
                        <div>
                            <TSD timesheet />
                        </div>
                    }
                }
                Some(Err(e)) => {
                    view! {
                        <div>{format!("Error Getting Resource: {}", e)} ,</div>
                    }
                }
                None => {
                    view! {
                        <div>"Error Getting Resource"</div>
                    }
                }
            }}

        </section>
    }}
}

#[component]
pub fn TimeSheetMissing() -> impl IntoView {
    view! { <h1>"Missing"</h1> }
}

#[server]
async fn get_active_user_timesheet() -> Result<TimeSheet, ServerFnError> {
    use axum_session::SessionPgSession;
    use uuid::Uuid;
    use chrono::{NaiveDateTime, Local, Duration, Weekday};

    let Some(session) = use_context::<SessionPgSession>() else {
        return Err(ServerFnError::ServerError("Session missing.".into()));
    };

    let Some(id) = session.get::<Uuid>("id") else {
        leptos_axum::redirect("/sign_in");
        return Err(ServerFnError::ServerError("Error getting Session!".into()));
    };

    let Some(now) = NaiveDateTime::from_timestamp_opt(  Local::now().timestamp(), 0) else {
        return Err(ServerFnError::ServerError("Error Converting Time".into()));
    };
    let three_weeks_before = now.clone().date().week(Weekday::Mon).first_day() - Duration::days(14);

    match TimeSheet::generate_for(id, three_weeks_before, now.date()).await {
        Ok(ts) => {
            leptos::tracing::error!("##| {}", id);
            Ok(ts)},
        Err(_) => Err(ServerFnError::ServerError("Error Generating Time Sheet".into())),
    }
}