
use std::sync::OnceLock;

use crate::models::user::UserPublic;
use cfg_if::cfg_if;
use leptos::*;
use leptos_router::ActionForm;
use uuid::uuid;

#[server]
async fn get_user() -> Result<UserPublic, ServerFnError> {
    use crate::models::user::UserPublic;
    use uuid::Uuid;
    use axum_session::SessionPgSession;


    match UserPublic::get(uuid!("d6fe6b08-23b4-4e14-b108-c2f020194f49")).await {
        Ok(_) => (),
        Err(_) => leptos::tracing::warn!("********** Error getting accessing DB!!!!!!!")

    };


    let Some(session) = use_context::<SessionPgSession>() else {
        return Err(ServerFnError::ServerError("Session missing.".into()));
    };

    leptos::tracing::error!("*||* SESSION: {:?}", session);

    let Some(id) = session.get::<Uuid>("id") else {
        leptos::tracing::warn!("*| Error getting SESSION: {:?}", session);
        leptos_axum::redirect("/sign_in");
        return Err(ServerFnError::ServerError("*Error getting SESSION!".into()));
    };

    let Ok(user) = UserPublic::get(id).await else {
        return Err(ServerFnError::ServerError("Could Not Find User.".into()));
    };
    Ok(user)
}



cfg_if! {
if #[cfg(feature = "ssr")] {



async fn is_close(latitude: f64, longitude: f64, accuracy: f64) -> Result<(), ServerFnError> {
    use crate::models::location_trackers::insert;
    use crate::utils::caluclate_distance;
    use std::env;

    let LATITUDE: f64 = env::var("LATITUDE").expect("To have ENV VAR: LATITUDE".into()).parse::<f64>().expect("`LATITUDE` to be a floating point number".into());
    let LONGITUDE: f64 = env::var("LONGITUDE").expect("To have ENV VAR: LONGITUDE".into()).parse::<f64>().expect("`LONGITUDE` to be a floating point number".into());
    let ACCURACY: f64 = env::var("ACCURACY").expect("To have ENV VAR: ACCURACY".into()).parse::<f64>().expect("`ACCURACY` to be a floating point number".into());

    let _ = insert(latitude, longitude, accuracy).await.map_err(|e|
        leptos::tracing::error!("Insert Tracing Error: {:?}", e)
    );

    if caluclate_distance(latitude, longitude, LATITUDE, LONGITUDE) > ACCURACY {
        return Err(ServerFnError::Request("You are too far away.".into()));
    };
    if accuracy > ACCURACY {
        return Err(ServerFnError::Request(
            "The location is not accurate enough.".into(),
        ));
    };
    Ok(())
}

}}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let user = create_resource(|| {}, move |_| get_user());

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
                                    <div id=u
                                        .id
                                        .to_string()>
                                        {move || match u.check_in {
                                            Some(t) => {
                                                view! {
                                                    <div
                                                        id="checked_in"
                                                        data-time=t.to_string()
                                                        data-state="success"
                                                    >
                                                        "You are Checked In"
                                                    </div>
                                                }
                                            }
                                            None => {
                                                view! {
                                                    <div id="checked_out" data-state="warning">
                                                        "You are Checked Out"
                                                    </div>
                                                }
                                            }
                                        }}
                                        <h1>{u.first_name}</h1>
                                    </div>
                                }
                            }
                            Some(Err(e)) => view! { <div>"some error e:" {e.to_string()}</div> },
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
    use crate::models::sessions::{close_session, get_open_session, new_session};
    use uuid::Uuid;
    // Get User
    use axum_session::SessionPgSession;
    let session = use_context::<SessionPgSession>()
        .ok_or_else(|| ServerFnError::ServerError("Session missing.".into()))?;
    let id = session
        .get::<Uuid>("id")
        .ok_or_else(|| ServerFnError::ServerError("Error getting Session!".into()))?;

    match is_close(latitude, longitude, accuracy).await {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

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

    leptos_axum::redirect("/");

    Ok(())
}

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
