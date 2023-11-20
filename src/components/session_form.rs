use leptos_router::ActionForm;
use uuid::Uuid;
use leptos::*;

#[cfg(feature = "ssr")]
use chrono::{Local, DateTime, TimeZone, NaiveDateTime};

#[component]
pub fn CorrectionForm<F>(uuid: Option<Uuid>, date: F) -> impl IntoView where
    F: Fn() -> Option<String> + 'static
{
    let action = create_server_action::<SubmitCorrectionForm>();
    let value = action.value();
    view! {
        <ActionForm action class="stack">
            {move || match uuid {
                Some(i) => {
                    view! { <input type="hidden" name="id" value=i.to_string()/> }.into_view()
                }
                None => view! { <span></span> }.into_view(),
            }}

            {move || match date() {
                Some(date) => view! { <input type="hidden" name="date" value=date/> }.into_view(),
                None => {
                    view! {
                        <div class="input">
                            <label>"Date"</label>
                            <input type="date" name="date"/>
                        </div>
                    }
                        .into_view()
                }
            }}

            <div class="input">
                <label>"Start Time"</label>
                <input type="time" name="start_time"/>
            </div>
            <div class="input">
                <label>"End Time"</label>
                <input type="time" name="end_time"/>
            </div>
            <div class="input">
                <label>"Reason"</label>
                <textarea name="reason"></textarea>
            </div>
            <button type="submit">"Ok"</button>
        </ActionForm>
        {move || match value() {
            Some(Ok(_)) => view! { <div>"success"</div> }.into_view(),
            Some(Err(e)) => {
                view! { <div data-state="error">"Error making correction: " {e.to_string()}</div> }
                    .into_view()
            }
            None => view! { <span>""</span> }.into_view(),
        }}
    }
}

#[server]
pub async fn submit_correction_form(id: Option<Uuid>, start_time: String, end_time: String, reason: String, date: String) -> Result<(), ServerFnError> {
    use crate::models::sessions::add_correction;
    use crate::app::get_curent_user;

    let start_date = convert_string_to_local_datetime(&date, &start_time)?;
    let end_date = convert_string_to_local_datetime(&date, &end_time)?;
    let Ok(Some(user_id)) = get_curent_user().await else { return Err(ServerFnError::Args("Unathorized".into()))};
    match add_correction(id, start_date, end_date, reason, user_id.id).await {
        Ok(_) => {
            leptos_axum::redirect("/app/timesheet");
            Ok(())
        },
        Err(e) => {
            leptos::logging::error!("Error adding correction: {}", e.to_string());
            Err(ServerFnError::MissingArg(format!("Error adding correction: {}", e.to_string())))
        }
    }
}

///"23-11-16" "13:01"
#[cfg(feature="ssr")]
fn convert_string_to_local_datetime(date: &str, time: &str) -> Result<DateTime<Local>, ServerFnError> {
    let date_time_string = date.to_owned() + " " + time;
    let parse_date_short = NaiveDateTime::parse_from_str(&date_time_string, "%y-%m-%d %R");
    let parse_date_long = NaiveDateTime::parse_from_str(&date_time_string, "%Y-%m-%d %R");
    let naive = match (parse_date_short, parse_date_long) {
        (Ok(d), _) => d,
        (_, Ok(d)) => d,
        (_, _) => return Err(ServerFnError::Deserialization(format!("Date in incorrect format: `{date_time_string}` is invalid")))
    };
    match Local.from_local_datetime(&naive) {
        chrono::LocalResult::Single(dt) => Ok(dt),
        chrono::LocalResult::Ambiguous(dt, _) => Ok(dt),
        chrono::LocalResult::None => Err(ServerFnError::Deserialization("Error Deserializaing Local".into())),
    }
}