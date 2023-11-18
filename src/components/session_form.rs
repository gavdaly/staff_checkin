use chrono::format::{DelayedFormat, StrftimeItems};
use leptos_router::ActionForm;
use uuid::Uuid;
use leptos::*;

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
            Some(Ok(result)) => view! { <div>"success"</div> }.into_view(),
            Some(Err(e)) => {
                view! { <div data-state="error">"Error making correction: " {e.to_string()}</div> }
                    .into_view()
            }
            None => view! { <span>""</span> }.into_view(),
        }}

        {move || view! { <pre>{format!("{:?}", value())}</pre> }}
    }
}

#[server]
pub async fn submit_correction_form(id: Option<Uuid>, start_time: String, end_time: String, reason: String, date: String) -> Result<(), ServerFnError> {
    use crate::models::sessions::add_correction;
    use crate::app::get_curent_user;
    use chrono::Local;

    // TODO! Combine dates and submit correction
    let start_date = Local::now();
    let end_date = Local::now();
    let Ok(Some(user_id)) = get_curent_user().await else { return Err(ServerFnError::Args("Unathorized".into()))};
    // let _ = add_correction(id, start_date, end_date, reason, user_id.id).await;

    leptos_axum::redirect("/app/timesheet");
    Ok(())
}