use leptos_router::ActionForm;
use uuid::Uuid;
use leptos::*;

#[cfg(feature = "ssr")]
use chrono::{Local, DateTime, TimeZone, NaiveDateTime};

/// Generates a form for submitting corrections.
///
/// # Arguments
///
/// * `uuid` - An optional UUID representing the correction form.
/// * `date` - A function that returns an optional string representing the date.s
///
/// # Flow
///
/// 1. Create a server action using `create_server_action` and assign it to the `action` variable.
/// 2. Get the value of the server action using `action.value()` and assign it to the `value` variable.
/// 3. Generate the form using the `ActionForm` component and set the `action` attribute to the server action.
/// 4. Render a hidden input field with the `id` value if `uuid` is `Some`, otherwise render an empty span.
/// 5. Render a hidden input field with the `date` value if `date()` returns `Some`, otherwise render a div with a label and an input field for selecting the date.
/// 6. Render input fields for the start time, end time, and reason.
/// 7. Render a submit button.
/// 8. Render a success message if the value of the server action is `Some(Ok(_))`.
/// 9. Render an error message with the error string if the value of the server action is `Some(Err(e))`.
/// 10. Render an empty span if the value of the server action is `None`.
///
/// # Returns
///
/// The generated form as an `IntoView`.
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

/// # submit_correction_form
///
/// This function handles the submission of a correction form.
///
/// ## Parameters
/// - `id` (optional): The UUID of the correction form.
/// - `start_time`: The start time of the correction.
/// - `end_time`: The end time of the correction.
/// - `reason`: The reason for the correction.
/// - `date`: The date of the correction.
///
/// ## Returns
/// - `Ok(())`: If the correction is added successfully.
/// - `Err(ServerFnError)`: If there is an error adding the correction.
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

/// Converts a date and time string to a `DateTime` object in the local timezone.
///
/// # Arguments
///
/// * `date` - A string representing the date in the format "dd-mm-yy".
/// * `time` - A string representing the time in the format "hh:mm".
///
/// # Returns
///
/// Returns a `Result` containing a `DateTime` object if the conversion is successful, or a `ServerFnError` if there is an error in the conversion process.
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