use leptos::*;
use leptos_router::*;
use uuid::Uuid;
use crate::components::loading_progress::Loading;

#[derive(Clone, Params, PartialEq)]
struct MagicLinkParams {
    link: Uuid,
}

#[component]
pub fn MagicLink() -> impl IntoView {
    let params = use_params::<MagicLinkParams>();
    let magic_sign_in = create_server_action::<MagicSignIn>();
    move || {match params() {
        Ok(MagicLinkParams{link}) => {
            magic_sign_in.dispatch(MagicSignIn { link });
            view! {
                <div>
                    <Loading/>
                    <Redirect path="/app"/> // Fix: Import the `Redirect` component and use the `to` method to specify the redirect path.
                </div>
            }
        },
        Err(e) => view! { <div>"Error parsing Parameters: " {e.to_string()}</div> }
    }}
}

#[server]
async fn magic_sign_in(link: Uuid) -> Result<(), ServerFnError> {
    use axum_session::SessionPgSession;
    use crate::models::magic_link::MagicLink;

    let session = use_context::<SessionPgSession>().ok_or_else(|| ServerFnError::ServerError("Session missing.".into()))?;
    let user_id = MagicLink::get(link).await.or_else(|_| Err(ServerFnError::ServerError("Invalid Link".into())))?;

    // find session 
    leptos::logging::log!("magic_link: {link}");

    session.set_longterm(true);
    session.set("id", user_id);
    leptos_axum::redirect("/app");

    Ok(())
}