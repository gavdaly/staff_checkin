use leptos::*;
use leptos_router::*;
use crate::components::loading_progress::Loading;
use uuid::Uuid;
use crate::models::sessions::Session;
use crate::models::time_sheets::TimeSheet;
use crate::components::timesheet::TimeSheetDisplay as Tsd;
use crate::components::session_form::CorrectionForm;

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
            <A href="/app/timesheet/missing">"Add missing time"</A>
            <Suspense fallback=move || {
                view! {
                    <div>
                        <Loading/>
                    </div>
                }
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
    view! { <CorrectionForm uuid=None date=|| None/> }
}

#[derive(Params, Clone, PartialEq)]
struct TimeSheetEditParams {
    uuid: Uuid
}

#[component]
pub fn TimeSheetEdit() -> impl IntoView {
    let params = use_params::<TimeSheetEditParams>();
    let session = create_server_action::<GetSession>();
    let value = session.value();
    let date = move || match value() {
        Some(Ok(Session { start_time, ..})) => Some(start_time.format("%y-%m-%d").to_string()),
        _ => None,
    };
    match params() {
        Ok(TimeSheetEditParams {
            uuid
        }) =>  {
            session.dispatch(GetSession { uuid });
            
            view! { <CorrectionForm uuid=Some(uuid) date/> }.into_view()
        },
        Err(e) => view! { <div data-state="error">"Error getting session: " {e.to_string()}</div> }.into_view()
    }
}

#[server]
async fn get_session(uuid: Uuid) -> Result<Session, ServerFnError> {
    crate::models::sessions::get_session(&uuid).await.map_err(|_| ServerFnError::Request("Error Getting Session".into()))
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
