use chrono::{DateTime, Local};
use uuid::Uuid;
use leptos::*;

#[component]
pub fn CorrectionForm(uuid: Option<Uuid>) -> impl IntoView {
    view! {
        <form>
            {move || match uuid {
                Some(i) => {
                    view! { <input type="hidden" name="id" value=i.to_string()/> }.into_view()
                }
                None => view! {}.into_view(),
            }}
            <div>
                <label>"Start Time"</label>
                <input type="time" name="start_time"/>
            </div> <div>
                <label>"End Time"</label>
                <input type="time" name="end_tie"/>
            </div> <div>
                <label>"Reason"</label>
                <textarea name="reason"></textarea>
            </div> <button type="submit">"Ok"</button> <button type="cancel">"cancel"</button>
        </form>
    }
}

#[server]
pub async fn submit_correction_form(id: Option<Uuid>, start_date: DateTime<Local>, end_date: DateTime<Local>, reason: String) -> Result<(), ServerFnError> {
    use crate::models::sessions::add_correction;
    use crate::app::get_curent_user;

    let Ok(Some(user_id)) = get_curent_user().await else { return Err(ServerFnError::Args("Unathorized".into()))};
    let _ = add_correction(id, start_date, end_date, reason, user_id.id).await;
    Ok(())
}