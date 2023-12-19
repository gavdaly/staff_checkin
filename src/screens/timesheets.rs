use crate::components::loading_progress::Loading;
use crate::models::user::UserPublic;
use chrono::NaiveDate;
use leptos::*;
use leptos_router::*;
use uuid::Uuid;
use crate::components::timesheet::TimeSheetDisplay;
use crate::models::time_sheets::TimeSheet;

/// Renders the home page of your application.
#[component]
pub fn TimeSheets() -> impl IntoView {
    view! {
        <nav class="subWrapper">
            <A href="" exact=true>
                "Time Sheets"
            </A>
            <A href="adjustment" exact=true>
                "Add Adjustment"
            </A>
            <A href="pending" exact=true>
                "Pending Corrections"
            </A>
        </nav>
        <section class="stack">
            <Outlet/>
        </section>
    }
}

#[server]
async fn load_timesheet_for<'a>(user_id: String) -> Result<TimeSheet, ServerFnError> {
    use uuid::Uuid;
    use chrono::{NaiveDateTime, Local, Duration, Weekday};

    let Ok(id) = Uuid::parse_str(&user_id) else {
        return Err(ServerFnError::Deserialization("Error parsing ID".into()));
    };
    
    let Some(now) = NaiveDateTime::from_timestamp_opt(Local::now().timestamp(), 0) else {
        return Err(ServerFnError::ServerError("Error Converting Time".into()));
    };
    let three_weeks_before = now.clone().date().week(Weekday::Mon).first_day() - Duration::days(14);
    let end_of_week = now.date().week(Weekday::Mon).last_day() + Duration::days(7);

    match TimeSheet::generate_for(id, three_weeks_before, end_of_week).await {
        Ok(ts) => Ok(ts),
        Err(_) => Err(ServerFnError::ServerError("Error Generating Time Sheet".into())),
    }
}

#[server]
pub async fn load_hourly_users() -> Result<Vec<UserPublic>, ServerFnError> {
    match UserPublic::get_all_hourly().await {
        Ok(v) => Ok(v),
        Err(_) => Err(ServerFnError::ServerError("Server Error".to_string())),
    }
}

#[component]
pub fn TimeSheetsList() -> impl IntoView {
    let (current_user, set_current_user) = create_signal(String::new());
    let users = create_resource(move || {}, move |_| load_hourly_users());
    let timesheet = create_resource(current_user, load_timesheet_for);

    create_effect( { move |_|
        leptos::logging::log!("{:?}", current_user())
    });

    view! {
        <Suspense fallback=move || {
            view! {
                <p>
                    <Loading/>
                </p>
            }
        }>
            {move || match users.get() {
                Some(Ok(a)) => {
                    view! {
                        <div>
                            <label for="user_selected"></label>
                            <select
                                name="user_selected"
                                id="user_selected"
                                on:change=move |e| set_current_user(event_target_value(&e))
                            >
                                <Show when=move || current_user().is_empty()>
                                    <option value="">"-- Select User --"</option>
                                </Show>
                                {a
                                    .iter()
                                    .map(|user| {
                                        view! {
                                            <option value=user
                                                .id
                                                .to_string()>
                                                {user.last_name.clone()} ", " {user.first_name.clone()}
                                            </option>
                                        }
                                    })
                                    .collect_view()}
                            </select>
                        </div>
                    }
                }
                _ => view! { <div>"Server Error"</div> },
            }}
            <Show when=move || {
                !current_user().is_empty()
            }>
                {move || match timesheet() {
                    Some(Ok(timesheet)) => {
                        view! {
                            <div>
                                <TimeSheetDisplay timesheet/>
                            </div>
                        }
                    }
                    Some(Err(e)) => view! { <div>"Error: " {e.to_string()}</div> },
                    None => view! { <div>"Error loading timesheet"</div> },
                }}

            </Show>

        </Suspense>
    }
}

#[component]
pub fn TimeSheetsAdjustment() -> impl IntoView {
    let users = create_resource(move || {}, move |_| load_hourly_users());
    let create_adjustment = create_server_action::<CreateAdjustment>();
    view! {
        <ActionForm action=create_adjustment>
            <Suspense fallback=move || {
                view! {
                    <p>
                        <Loading/>
                    </p>
                }
            }>
                {move || match users.get() {
                    Some(Ok(a)) => {
                        view! {
                            <div>
                                <label for="user_id">"User"</label>
                                <select name="user_id" id="user_id">
                                    {a
                                        .iter()
                                        .map(|user| {
                                            view! {
                                                <option value=user
                                                    .id
                                                    .to_string()>
                                                    {user.last_name.clone()} ", " {user.first_name.clone()}
                                                </option>
                                            }
                                        })
                                        .collect_view()}
                                </select>
                            </div>
                        }
                    }
                    _ => view! { <div>"Server Error"</div> },
                }}

            </Suspense>
            <div>
                <label for="date">"Date"</label>
                <input type="date" name="date" id="date"/>
            </div>
            <div>
                <label for="hours">"Hours"</label>
                <input type="number" name="hours" id="hours"/>
            </div>
            <button type="submit">"Submit"</button>
        </ActionForm>
    }
}

#[server]
pub async fn create_adjustment(user_id: Uuid, date: NaiveDate, hours: i32, response: String) -> Result<(), ServerFnError> {
    use crate::models::adjustments::create_adjustment as create_adjustment_fn;

    match create_adjustment_fn(&user_id, date, hours, &response).await {
        Ok(_) => Ok(()),
        Err(_) => Err(ServerFnError::ServerError("Server Error".to_string())),
    }
}

#[component]
pub fn TimeSheetsPending() -> impl IntoView {
    view! { <h1>"TimeSheets | To Do"</h1> }
}
