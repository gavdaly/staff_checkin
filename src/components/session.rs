use chrono::Local;
use leptos::*;
use leptos_router::{A, ActionForm};
use uuid::Uuid;
use crate::models::sessions::SessionAndCorrection;
use crate::models::corrections::Correction;
use crate::utils::miliseconds_to_string;

/// Renders a session component that displays session information.
/// 
/// # Arguments
/// 
/// * `session` - A reference to a `SessionAndCorrection` struct that contains session data.
#[component]
pub fn Session<'a>(session: &'a SessionAndCorrection) -> impl IntoView {
    let id = session.id.to_string();
    let start_time = session.start_time.with_timezone(&Local);
    let start_string = start_time.format("%I:%M %P").to_string();
    let start_dt = start_time.to_string();
    let end_time = match session.end_time {
        Some(t) => Some(t.with_timezone(&Local)),
        None => None,
    };
    view! {
        <span class="start_time">
            <time datetime=start_dt>{start_string}</time>
        </span>
        {match end_time {
            Some(t) => {
                view! {
                    <span class="end_time">
                        <time datetime=t.to_string()>{t.format("%I:%M %P").to_string()}</time>
                    </span>

                    <span class="duration">
                        {miliseconds_to_string(&(t - start_time).num_milliseconds()).to_string()}
                    </span>

                    {match session.state {
                        0 => view! { <span class="state">"open"</span> }.into_view(),
                        1 => {
                            view! {
                                <A class="state" href=format!("/app/timesheet/edit/{}", id)>
                                    edit
                                </A>
                            }
                                .into_view()
                        }
                        2 => {
                            view! {
                                <A class="state" href=format!("/app/timesheet/edit/{}", id)>
                                    error
                                </A>
                            }
                                .into_view()
                        }
                        3 => view! { <span class="state">"pending"</span> }.into_view(),
                        4 => view! { <span class="state">"accepted"</span> }.into_view(),
                        5 => view! { <span class="state">"rejected"</span> }.into_view(),
                        6 => view! { <span class="state">"done"</span> }.into_view(),
                        _ => {
                            view! {
                                <span class="state" data-state="error">
                                    "ERROR"
                                </span>
                            }
                                .into_view()
                        }
                    }}
                }
                    .into_view()
            }
            None => view! { <span class="open">"Session not closed yet!"</span> }.into_view(),
        }}

        {match session.correction.clone() {
            Some(correction) => {
                view! { <Correction correction session_state=session.state/> }
            }
            None => view! {}.into_view(),
        }}
    }
}

#[component]
fn Correction(correction: Correction, session_state: i32) -> impl IntoView {
    let start = correction.new_start_time.with_timezone(&Local).format("%I:%M %P").to_string();
    let end = correction.new_end_time.with_timezone(&Local).format("%I:%M %P").to_string();
    let handle_correction_response = create_server_action::<HandleCorrectionResponse>();
    match session_state {
        3 =>  view! {
            <span>{start}</span>
            <span>{end}</span>
            <span>"pending time"</span>
            <span></span>
            <span>"reason"</span>
            <span class="reason">{correction.reason}</span>
            <ActionForm action=handle_correction_response>
                <input type="hidden" name="id" value=correction.id.to_string()/>
                <div>
                    <label for="response">"response"</label>
                    <textarea id="response" name="response"></textarea>
                </div>
                <fieldset>
                    <div>
                        <legend>"Response Status"</legend>
                        <input type="radio" id="accepted" name="status" value="4" checked=true/>
                        <label for="accepted">"accepted"</label>
                    </div>
                    <div>
                        <input type="radio" id="rejected" name="status" value="5"/>
                        <label for="rejected">"rejected"</label>
                    </div>
                </fieldset>
                <button type="submit">"submit"</button>
            </ActionForm>
        }.into_view(),
        4 | 5 => view! {
            <span>"response"</span>
            <span class="reason">{correction.response}</span>
        }.into_view(),
        _ => view! { }.into_view(),
    }
}

#[server]
async fn handle_correction_response(response: String, status: u32, id: Uuid) -> Result<(), ServerFnError> {
    use crate::models::corrections::correction_response;
    match correction_response(id, status, &response).await {
        Ok(_) => Ok(()),
        Err(e) => Err( ServerFnError::ServerError(e.to_string())),
    }
}
