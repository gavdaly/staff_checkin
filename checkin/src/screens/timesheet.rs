use leptos::*;
use leptos_router::*;
use crate::models::time_sheets::TimeSheet;
use crate::components::timesheet::TimeSheetDisplay as Tsd;

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
            <Suspense fallback=move || {
                view! { <div>"Loading..."</div> }
            }>
                {match timesheet() {
                    Some(Ok(timesheet)) => {
                        view! {
                            <div>
                                <Tsd timesheet/>
                            </div>
                        }
                    }
                    Some(Err(e)) => {
                        view! { <div>{format!("Error Getting Resource: {}", e)} ,</div> }
                    }
                    None => {
                        view! { <div>"Error Getting Resource"</div> }
                    }
                }}

            </Suspense>
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
    let Some(now) = NaiveDateTime::from_timestamp_opt(Local::now().timestamp(), 0) else {
        return Err(ServerFnError::ServerError("Error Converting Time".into()));
    };
    let three_weeks_before = now.clone().date().week(Weekday::Mon).first_day() - Duration::days(14);
    let end_of_week = now.date().week(Weekday::Mon).last_day() + Duration::days(7);

    match TimeSheet::generate_for(id, three_weeks_before, end_of_week).await {
        Ok(ts) => {
            leptos::tracing::info!("######| {:?}", ts);
            Ok(ts)},
        Err(_) => Err(ServerFnError::ServerError("Error Generating Time Sheet".into())),
    }
}