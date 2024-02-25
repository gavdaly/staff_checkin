use crate::components::loading_progress::Loading;
use leptos::server_fn::error::NoCustomError;
use leptos::*;
use leptos_router::*;

#[derive(Clone, Params, PartialEq)]
struct ClockInLinkParams {
    link: String,
}

#[component]
pub fn ClockInLink(
    clock_in_link: Action<ClockInLinkInitiateSession, Result<(), ServerFnError>>,
) -> impl IntoView {
    let params = use_params::<ClockInLinkParams>();
    match params() {
        Ok(ClockInLinkParams { link }) => {
            clock_in_link.dispatch(ClockInLinkInitiateSession { link: link.clone() });
            view! {
                <div>
                    <Loading/>
                    <Redirect path="/app"/>
                </div>
            }
        }
        Err(e) => view! { <div>"Something went wrong: " {e.to_string()}</div> },
    }
}

#[server]
pub async fn clock_in_link_initiate_session(link: String) -> Result<(), ServerFnError> {
    use crate::models::sessions::{close_session, get_open_session, new_session};
    use uuid::Uuid;
    // Get User
    use axum_session::SessionPgSession;
    let session = use_context::<SessionPgSession>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Session missing.".into()))?;
    let id = session.get::<Uuid>("id").ok_or_else(|| {
        ServerFnError::<NoCustomError>::ServerError("Error getting Session!".into())
    })?;

    // check to see if link is valid!!
    leptos::logging::log!("link: {link}");

    // check for existing session
    match get_open_session(&id).await {
        Ok(sess) => {
            // if no session create new session
            let _ = close_session(&sess.id).await;
        }
        Err(_) => {
            // else close exsiting session
            let _ = new_session(&id).await;
        }
    };
    leptos_axum::redirect("/app");

    Ok(())
}
