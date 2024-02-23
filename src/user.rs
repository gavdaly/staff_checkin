use leptos::{server_fn::error::NoCustomError, *};
use serde::{Deserialize, Serialize};

use crate::models::user::UserDisplay;

pub fn provide_user_context() {
    if use_context::<User>().is_none() {
        provide_context(User::new());
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct User {
    pub data: Option<UserDisplay>,
}

impl User {
    pub fn new() -> Self {
        let log_out = create_server_action::<Logout>();
        let check_in = create_server_action::<CheckIn>();
        let authenticate = create_server_action::<Authenticate>();
        let clock_in_link = create_server_action::<ClockInLinkInitiateSession>();

        let user_fetch = create_resource(move || (log_out.version().get(), authenticate.version().get()), |_| get_curent_user());

        let data = match user_fetch() {
            Some(Ok(user)) => user,
            _ => None,
        
        };

        Self {
            data,
        }
    }
    pub fn log_out(&self) -> Result<(), ServerFnError> {
        let log_out = create_server_action::<Logout>();
        log_out.dispatch(Logout {  });
        Ok(())
    }

}

#[server]
pub async fn get_curent_user() -> Result<Option<UserDisplay>, ServerFnError> {
    use uuid::Uuid;
    use axum_session::SessionPgSession;

    let Some(session) = use_context::<SessionPgSession>() else {
        leptos::tracing::error!("| * Error getting settion");
        return Err(ServerFnError::ServerError("Error Finding Session".into()));
    };

    let Some(id) = session.get::<Uuid>("id") else {
        leptos::tracing::info!("| * User not signed in");
        return Ok(None);
    };

    let Ok(user) = UserDisplay::get(id).await else {
        leptos::tracing::error!("| * Could not find User for session");
        return Err(ServerFnError::ServerError("Could Not Find User.".into()));
    };

    Ok(Some(user))
}

#[server]
async fn authenticate(pin: i32, phone: String) -> Result<(), ServerFnError> {
    use crate::models::user::get_user_by_phone;
    use axum_session::SessionPgSession;
    use crate::models::pins::Pin;

    let Ok(pin) = Pin::get_pin(pin).await else {
        return Err(ServerFnError::<NoCustomError>::ServerError("Internal Server Error".into()));
    };

    let Ok(user) = get_user_by_phone(&phone).await else {
        return Err(ServerFnError::ServerError("Internal Server Error".into()));
    };

    let session = use_context::<SessionPgSession>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Session missing.".into()))?;

    if pin.user_id != user.id {
        return Err(ServerFnError::Request("Unauthorized Try Again!".into()));
    }
    session.set_longterm(true);
    session.set("id", user.id);
    leptos_axum::redirect("/app");
    Ok(())
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use axum_session::SessionPgSession;
    let session = use_context::<SessionPgSession>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Session missing.".into()))?;
    session.clear();

    leptos_axum::redirect("/");
    Ok(())
}

#[server]
async fn check_in(latitude: f64, longitude: f64, accuracy: f64) -> Result<(), ServerFnError> {
    use crate::models::sessions::{close_session, get_open_session, new_session};
    use uuid::Uuid;
    // Get User
    use axum_session::SessionPgSession;
    let session = use_context::<SessionPgSession>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Session missing.".into()))?;
    let id = session
        .get::<Uuid>("id")
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Error getting Session!".into()))?;

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

    leptos_axum::redirect("/app");

    Ok(())
}

#[cfg(feature = "ssr")] 
async fn is_close(latitude: f64, longitude: f64, accuracy: f64) -> Result<(), ServerFnError> {
    use crate::models::location_trackers::insert;
    use crate::utils::caluclate_distance;
    use std::env;

    let base_latitude: f64 = env::var("LATITUDE").expect("To have ENV VAR: LATITUDE".into()).parse::<f64>().expect("`LATITUDE` to be a floating point number".into());
    let base_longitude: f64 = env::var("LONGITUDE").expect("To have ENV VAR: LONGITUDE".into()).parse::<f64>().expect("`LONGITUDE` to be a floating point number".into());
    let base_accuracy: f64 = env::var("ACCURACY").expect("To have ENV VAR: ACCURACY".into()).parse::<f64>().expect("`ACCURACY` to be a floating point number".into());

    let _ = insert(latitude, longitude, accuracy).await.map_err(|e|
        leptos::tracing::error!("Insert Tracing Error: {:?}", e)
    );
    if caluclate_distance(latitude, longitude, base_latitude, base_longitude) > base_accuracy {
        return Err(ServerFnError::Request("You are too far away.".into()));
    };
    if accuracy > base_accuracy {
        return Err(ServerFnError::Request(
            "The location is not accurate enough.".into(),
        ));
    };
    Ok(())
}


#[server]
pub async fn clock_in_link_initiate_session(link: String) -> Result<(), ServerFnError> {
    use crate::models::sessions::{close_session, get_open_session, new_session};
    use uuid::Uuid;
    // Get User
    use axum_session::SessionPgSession;
    let session = use_context::<SessionPgSession>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Session missing.".into()))?;
    let id = session
        .get::<Uuid>("id")
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Error getting Session!".into()))?;

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
