use crate::models::user::UserPublic;
use leptos::*;
use leptos_router::ActionForm;

#[server]
async fn get_user() -> Result<UserPublic, ServerFnError> {
    use crate::models::user::UserPublic;
    use uuid::Uuid;

    use axum_session::SessionPgSession;
    let session = use_context::<SessionPgSession>()
        .ok_or_else(|| ServerFnError::ServerError("Session missing.".into()))?;
    let id = session.get::<Uuid>("id").ok_or_else(|| ServerFnError::ServerError("Error getting Session!".into()))?;

    let Ok(user) = UserPublic::get(id).await else {
        return Err(ServerFnError::ServerError("Could Not Find User.".into()))
    };
    Ok(user)
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let user = create_resource(
        || {},
        move |_| get_user(),
    );

    // get user
    // get settings
    // account state

    // show logged in status
    // show week summary
    // upcomming vacations
    view! {
        <section class="stack">
            <Suspense fallback=|| {
                view! { "Loading..." }
            }>
                {move || {
                    view! {
                        {match user.get() {
                            Some(Ok(u)) => {
                                view! {
                                    <div id=u.id.to_string()>
                                        <h1>{u.first_name}</h1>
                                        <p>{u.phone_number}</p>
                                    </div>
                                }
                            }
                            _ => view! { <div>"error"</div> },
                        }}
                    }
                }}

            </Suspense>
        </section>
    }
}

#[server]
async fn check_in(latitude: f64, longitude: f64, accuracy: f64) -> Result<(), ServerFnError> {
    use uuid::Uuid;
    use crate::models::location_trackers::insert;
    use crate::models::sessions::{get_open_session, close_session, new_session};
    // Get User
    use axum_session::SessionPgSession;
    let session = use_context::<SessionPgSession>()
        .ok_or_else(|| ServerFnError::ServerError("Session missing.".into()))?;
    let id = session.get::<Uuid>("id").ok_or_else(|| ServerFnError::ServerError("Error getting Session!".into()))?;

    insert(latitude, longitude, accuracy).await;
    // check distance

    // check for existing session
    match get_open_session(&id).await {
        Ok(sess) => {
            // if no session create new session
            close_session(&sess.id).await;
        },
        Err(_) => {
            // else close exsiting session
            new_session(&id).await;
        }
    };

    leptos_axum::redirect("/");

    Ok(())
}

// #[server]
// async fn get_status() -> Result<String, ServerFnError> {
//     Ok("CheckedStatus::In".to_string())
// }

#[component]
pub fn CheckIn() -> impl IntoView {
    use leptos_use::{use_geolocation_with_options, UseGeolocationReturn};
    let check_in = create_server_action::<CheckIn>();
    let value = check_in.value();

    let options = leptos_use::UseGeolocationOptions::default().enable_high_accuracy(true);

    let UseGeolocationReturn {
        coords,
        located_at: _,
        error,
        resume: _,
        pause: _,
    } = use_geolocation_with_options(options);

    view! {
        <section class="center-center">
            <Show when=move || {
                coords.with(Option::is_some)
            }>
                {move || {
                    if let Some(coords) = coords() {
                        view! {
                            <div class="center-center">
                                <ActionForm class="center-center" action=check_in>
                                    <input type="hidden" value=coords.latitude() name="latitude"/>
                                    <input type="hidden" value=coords.longitude() name="longitude"/>
                                    <input type="hidden" value=coords.accuracy() name="accuracy"/>
                                    <button
                                        type="submit"
                                        data-size="huge"
                                        disable=check_in.pending()
                                    >
                                        "Check In"
                                    </button>
                                </ActionForm>
                            </div>
                        }
                    } else {
                        view! { <div>"?"</div> }
                    }
                }}

            </Show>

            <Show when=move || error().is_some()>
                <div data-state="error" class="center-center">
                    {move || {
                        if let Some(error) = error() {
                            location_error(error.code())
                        } else {
                            "".to_string()
                        }
                    }}

                </div>
            </Show>

            <Show when=move || value.with(Option::is_some)>
                <div class="center-center">{value}</div>
            </Show>
        </section>
    }
}

pub fn location_error(error_number: u16) -> String {
    match error_number {
        1 => "Location Services are disabled, please enable and try again.".to_string(),
        2 => "Error getting a signal for your location.".to_string(),
        3 => "Finding Location Took too long please try again.".to_string(),
        _ => "Unknow Error".to_string(),
    }
}

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <section class="stack">
            <h1>"Settings | To Do"</h1>
        </section>
    }
}
