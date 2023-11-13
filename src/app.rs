use crate::error_template::{AppError, ErrorTemplate};
use crate::models::user::UserPublic;
use crate::screens::home::{ HomePage, Settings};
use crate::screens::timesheet::{TimeSheetDisplay, TimeSheetMissing};
use crate::screens::timesheets::{
    TimeSheets, TimeSheetsAdjustment, TimeSheetsList, TimeSheetsPending,
};
use crate::screens::users::{UserCreate, UserUpdate, Users, UsersList};
use crate::screens::vacations::{
    VacationEdit, VacationRequest, Vacations, VacationsList, VacationsPending,
};
use crate::components::check_in::CheckInView;
use crate::components::menu::Menu;
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

    let user_fetch = create_resource(move || (log_out.version().get(), authenticate.version().get()), |_| get_curent_user());
    let session_status = create_resource(move || check_in.version().get(), |_| get_session_status());

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

    view! {
        <Stylesheet id="leptos" href="/pkg/staff.css"/>

        // sets the document title
        <Title text="Dental Care"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Suspense fallback=|| {
                view! { "Loading..." }
            }>
                <header id="header">
                    <h1>
                        <span>"Click "</span>
                        <span class="version">{VERSION}</span>
                    </h1>
                </header>

                <Show when=move || user().is_some()>
                    <Menu status log_out/>
                </Show>
                <main id="main">
                    <Routes>
                        <Route path="/p/:phone" view=move || view! { <Auth authenticate/> }/>
                        <Route path="/l/:link" view=move || view! { <div>"TODO"</div> }/>
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
                            <Route path="/app" view=move || view! { <Outlet/> }>
                                <Route path="" view=move || view! { <HomePage status/> }/>
                                <Route path="/timesheet" view=TimeSheetDisplay/>
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
async fn logout() -> Result<(), ServerFnError> {
    use axum_session::SessionPgSession;
    let session = use_context::<SessionPgSession>()
        .ok_or_else(|| ServerFnError::ServerError("Session missing.".into()))?;
    session.clear();

    leptos_axum::redirect("/");
    Ok(())
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

#[server]
async fn authenticate(pin: i32, phone: String) -> Result<(), ServerFnError> {
    use crate::models::user::get_user_by_phone;
    use axum_session::SessionPgSession;
    use crate::models::pins::Pin;

    let Ok(pin) = Pin::get_pin(pin).await else {
        return Err(ServerFnError::ServerError("Internal Server Error".into()));
    };

    let Ok(user) = get_user_by_phone(&phone).await else {
        return Err(ServerFnError::ServerError("Internal Server Error".into()));
    };

    let session = use_context::<SessionPgSession>()
        .ok_or_else(|| ServerFnError::ServerError("Session missing.".into()))?;

    if pin.user_id != user.id {
        return Err(ServerFnError::Request("Unauthorized Try Again!".into()));
    }
    session.set_longterm(true);
    session.set("id", user.id);
    leptos_axum::redirect("/");
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

#[derive(Clone, Params, PartialEq)]
struct PhoneParams {
    phone: String,
}

#[component]
pub fn Auth(authenticate: Action<Authenticate, Result<(), ServerFnError>>) -> impl IntoView {
    let (pin_input, set_pin_input) = create_signal(String::with_capacity(6));

    let phone_params = use_params::<PhoneParams>();
    let pattern = "[0-9]{6}";

    let value = authenticate.value();

    create_effect(move |_| {
        if pin_input().len() == 6 {
            leptos::logging::log!("Reached Max Length")
        }
    });

    view! {
        <Title text="Dental Care | Authenticating"/>
        <section class="center-center">

            <Show
                when=move || phone_params().is_ok()
                fallback=move || {
                    view! { <div>"Should not see"</div> }
                }
            >

                {move || match phone_params() {
                    Ok(query) => {
                        view! {
                            <ActionForm action=authenticate class="center-center solo-action">
                                <input type="hidden" value=query.phone name="phone"/>
                                <label id="pin">"Enter Pin From SMS"</label>
                                <input
                                    type="number"
                                    name="pin"
                                    pattern=pattern
                                    inputMode="numeric"
                                    on:input=move |v| set_pin_input(event_target_value(&v))
                                />
                                <button type="submit" disabled=move || authenticate.pending()>
                                    "Log In"
                                </button>
                                <Show when=authenticate.pending()>
                                    <div>"Loading..."</div>
                                </Show>
                                <Show when=move || value.with(Option::is_some)>
                                    <div>{value}</div>
                                </Show>
                            </ActionForm>
                        }
                            .into_view()
                    }
                    Err(_e) => {
                        view! {
                            <div>
                                <input type="hidden" value="" name="phone"/>
                                <input type="hidden" name="pin"/>
                                <Show when=move || value.with(Option::is_some)>
                                    <div>{value}</div>
                                </Show>
                            </div>
                        }
                            .into_view()
                    }
                }}

            </Show>
        </section>
    }
}

#[server]
async fn get_pin(phone: String) -> Result<(), ServerFnError> {
    use crate::models::user::get_user_by_phone;
    use crate::service::sms::send_message;
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

    send_message(pin.number.to_string(), format!("+1{phone}")).await;

    leptos_axum::redirect(&("/p/".to_string() + &phone));

    Ok(())
}

#[component]
pub fn PhoneNumber() -> impl IntoView {
    let get_pin = create_server_action::<GetPin>();
    let value = get_pin.value();
    view! {
        <Title text="Dental Care | Authentication"/>

        <ActionForm class="center-center solo-action" action=get_pin>
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
            <button type="submit" disabled=move || get_pin.pending()>
                "Get Pin"
            </button>
        </ActionForm>
        <Show when=get_pin.pending()>
            <div>"Loading..."</div>
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