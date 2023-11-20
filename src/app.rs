use crate::components::loading_progress::Loading;
use crate::error_template::{AppError, ErrorTemplate};
use crate::models::user::UserPublic;
use crate::screens::clock_in_link::{ClockInLink, ClockInLinkInitiateSession};
use crate::screens::home::{ HomePage, Settings};
use crate::screens::magic_link::MagicLink;
use crate::screens::timesheet::{TimeSheetDisplay, TimeSheetMissing, TimeSheetEdit};
use crate::screens::timesheets::{
    TimeSheets, TimeSheetsAdjustment, TimeSheetsList, TimeSheetsPending,
};
use crate::screens::users::{UserCreate, UserUpdate, Users, UsersList};
use crate::screens::vacations::{
    VacationEdit, VacationRequest, Vacations, VacationsList, VacationsPending,
};
use crate::components::check_in::CheckInView;
use crate::components::menu::Menu;
use crate::screens::authenticate::{Auth, Authenticate, Logout};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

pub static VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let log_out = create_server_action::<Logout>();
    let check_in = create_server_action::<CheckIn>();
    let authenticate = create_server_action::<Authenticate>();
    let clock_in_link = create_server_action::<ClockInLinkInitiateSession>();

    let user_fetch = create_resource(move || (log_out.version().get(), authenticate.version().get()), |_| get_curent_user());
    let session_status = create_resource(move || (check_in.version().get(), clock_in_link.version().get()), |_| get_session_status());

    let _error = move || match user_fetch() {
        Some(Err(e)) => Some(e),
        Some(Ok(_)) => None,
        _ => None
    };

    let user = move || match user_fetch() {
        Some(Ok(user)) => user,
        _ => None
    };

    let status = move || match session_status() {
        Some(Ok(status)) => status,
        Some(Err(_)) => false,
        None => false,
    };

    let (show_menu, set_show_menu) = create_signal(false);

    view! {
        <Stylesheet id="leptos" href="/pkg/staff.css"/>

        // sets the document title
        <Title text="Dental Care"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Suspense fallback=Loading>
                <header id="header">
                    <h1>
                        <span>"Click "</span>
                        <span class="version">{VERSION}</span>
                    </h1>
                </header>

                <Show when=move || user().is_some()>
                    <Menu status log_out show_menu set_show_menu/>
                </Show>
                <main id="main">
                    <Routes>
                        <Route path="/p/:phone" view=move || view! { <Auth authenticate/> }/>
                        <Route path="/l/:link" view=MagicLink/>
                        <Route
                            path=""
                            view=move || {
                                view! {
                                    <Show when=move || user().is_some() fallback=PhoneNumber>
                                        <Outlet/>
                                    </Show>
                                }
                            }
                        >

                            <Route path="" view=move || view! { <HomePage status/> }/>
                            <Route
                                path="/c/:link"
                                view=move || view! { <ClockInLink clock_in_link/> }
                            />
                            <Route path="/app" view=move || view! { <Outlet/> }>
                                <Route path="" view=move || view! { <HomePage status/> }/>
                                <Route path="/timesheet" view=TimeSheetDisplay/>
                                <Route path="/timesheet/edit/:uuid" view=TimeSheetEdit/>
                                <Route path="/timesheet/missing" view=TimeSheetMissing/>
                                <Route path="/vacations" view=Vacations>
                                    <Route path="" view=VacationsList/>
                                    <Route path="/request" view=VacationRequest/>
                                    <Route path="/:id" view=VacationEdit/>
                                </Route>
                                <Route path="/users" view=Users/>
                                <Route
                                    path="/check_in"
                                    view=move || view! { <CheckInView check_in status/> }
                                />
                            </Route>
                            <Route path="/admin" view=move || view! { <Outlet/> }>
                                <Route path="/vacations" view=Vacations>
                                    <Route path="" view=VacationsList/>
                                    <Route path="/pending" view=VacationsPending/>
                                    <Route path="/request" view=VacationRequest/>
                                    <Route path="/:id" view=VacationEdit/>
                                </Route>
                                <Route path="/timesheets" view=TimeSheets>
                                    <Route path="" view=TimeSheetsList/>
                                    <Route path="/adjustment" view=TimeSheetsAdjustment/>
                                    <Route path="/pending" view=TimeSheetsPending/>
                                </Route>
                                <Route path="/users" view=Users>
                                    <Route path="" view=UsersList/>
                                    <Route path="/create" view=UserCreate/>
                                    <Route path="/edit/:id" view=UserUpdate/>
                                </Route>
                            </Route>
                            <Route path="/settings" view=Settings/>
                        </Route>
                    </Routes>
                </main>
            </Suspense>
        </Router>
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Status {
    user_name: String,
    checked_in_time: Option<u64>,
    user_type: String,
}

#[server]
pub async fn get_curent_user() -> Result<Option<UserPublic>, ServerFnError> {
    use uuid::Uuid;
    use axum_session::SessionPgSession;

    let Some(session) = use_context::<SessionPgSession>() else {
        leptos::tracing::error!("| * Error getting settion");
        return Err(ServerFnError::ServerError("Error Finding Session 30".into()));
    };

    let Some(id) = session.get::<Uuid>("id") else {
        leptos::tracing::info!("| * User not signed in");
        return Ok(None);
    };

    let Ok(user) = UserPublic::get(id).await else {
        leptos::tracing::error!("| * Could not find User for session");
        return Err(ServerFnError::ServerError("Could Not Find User.".into()));
    };

    Ok(Some(user))
}

#[server]
async fn get_session_status() -> Result<bool, ServerFnError> {
    use uuid::Uuid;
    use crate::models::sessions::get_open_sessions;
    use axum_session::SessionPgSession;

    let session = use_context::<SessionPgSession>()
        .ok_or_else(|| ServerFnError::ServerError("Session missing.".into()))?;
    let id = session
        .get::<Uuid>("id")
        .ok_or_else(|| ServerFnError::ServerError("Error getting Session!".into()))?;
    match get_open_sessions(&id).await {
        Ok(a) => Ok(a.len() != 0),
        Err(_) => Err(ServerFnError::ServerError("Error getting one".into()))
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
async fn submit_phone_number(phone: String) -> Result<(), ServerFnError> {
    use crate::models::user::get_user_by_phone;
    // use crate::service::sms::send_message;
    use crate::models::pins::Pin;

    let phone = crate::utils::filter_phone_number(&phone);

    leptos::tracing::info!("**| phone: {:?}", phone);

    let Ok(user) = get_user_by_phone(&phone).await else {
        leptos::tracing::warn!("Could not find phone number: {:?}", phone);    
        return Err(ServerFnError::Deserialization(
            "Could not Find Phone Number!".into(),
        ));
    };

    leptos::tracing::info!("**| user: {:?}", user);

    let Ok(pin) = Pin::create_pin_for(user.id).await else {
        leptos::tracing::error!("Could not create pin: {}", user.id.to_string());
        return Err(ServerFnError::ServerError("Error Creating Pin!".into()));
    };

    // send_message(pin.number.to_string(), format!("+1{phone}")).await;

    leptos_axum::redirect(&("/p/".to_string() + &phone));

    Ok(())
}

#[component]
pub fn PhoneNumber() -> impl IntoView {
    let submit = create_server_action::<SubmitPhoneNumber>();
    let value = submit.value();
    view! {
        <Title text="Dental Care | Authentication"/>

        <ActionForm class="center-center solo-action" action=submit>
            <label>"Phone Number"</label>
            <input
                id="phone"
                label="Phone Number"
                type="tel"
                name="phone"
                autoComplete="tel"
                placeholder="+1 (893) 234-2345"
                inputMode="tel"
                required
            />
            <button type="submit">"Get Pin"</button>
        </ActionForm>
        <Show when=submit.pending()>
            <div>
                <Loading/>
            </div>
        </Show>
        <Show when=move || {
            value().is_some()
        }>
            {match value() {
                Some(Err(e)) => view! { <div data-state="error">"Error: " {e.to_string()}</div> },
                _ => view! { <div data-state="error">"something is messed up"</div> },
            }}

        </Show>
    }
}
