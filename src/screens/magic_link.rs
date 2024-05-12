use crate::components::loading_progress::Loading;
use leptos::server_fn::error::NoCustomError;
use leptos::*;
use leptos_router::*;
use uuid::Uuid;

#[derive(Clone, Params, PartialEq)]
struct MagicLinkParams {
    link: Uuid,
}

#[component]
pub fn MagicLink() -> impl IntoView {
    let params = use_params::<MagicLinkParams>();
    let magic_sign_in = create_server_action::<MagicSignIn>();
    move || match params() {
        Ok(MagicLinkParams { link }) => {
            magic_sign_in.dispatch(MagicSignIn { link });
            view! {
                <div>
                    <Loading/>
                    <Redirect path="/app"/>
                </div>
            }
        }
        Err(e) => view! { <div>"Error parsing Parameters: " {e.to_string()}</div> },
    }
}

#[server]
async fn magic_sign_in(link: Uuid) -> Result<(), ServerFnError> {
    use crate::models::magic_link::MagicLink;
    use axum_session::SessionAnySession;

    let session = use_context::<SessionAnySession>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Session missing.".into()))?;
    let user_id = MagicLink::get(link)
        .await
        .map_err(|_| ServerFnError::<NoCustomError>::ServerError("Invalid Link".into()))?;

    // find session
    leptos::logging::log!("magic_link: {link}");

    session.set_longterm(true);
    session.set("id", user_id);
    leptos_axum::redirect("/app");

    Ok(())
}
