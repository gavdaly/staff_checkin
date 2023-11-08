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
use crate::models::pins::Pin;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

static VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

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
            <Suspense fallback=|| {
                view! { "Loading..." }
            }>
                <header id="header">
                    // <input type="checkbox" class="sr-only" id="menu" name="menu"/>
                    <Show when=move || user().is_some()>
                        <label for="menu" class="button" aria-hidden="true">
                            <button class="hamburger" on:click=move |_| { set_show_menu(true) }>
                                <svg
                                    aria-hidden="true"
                                    focusable="false"
                                    data-prefix="fas"
                                    data-icon="bars"
                                    role="img"
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 448 512"
                                    width="1em"
                                >
                                    <path
                                        fill="currentColor"
                                        d="M16 132h416c8.837 0 16-7.163 16-16V76c0-8.837-7.163-16-16-16H16C7.163 60 0 67.163 0 76v40c0 8.837 7.163 16 16 16zm0 160h416c8.837 0 16-7.163 16-16v-40c0-8.837-7.163-16-16-16H16c-8.837 0-16 7.163-16 16v40c0 8.837 7.163 16 16 16zm0 160h416c8.837 0 16-7.163 16-16v-40c0-8.837-7.163-16-16-16H16c-8.837 0-16 7.163-16 16v40c0 8.837 7.163 16 16 16z"
                                    ></path>
                                </svg>
                            </button>
                        </label>
                    </Show>
                    <h1>"Click"</h1>
                </header>

                <Show when=move || user().is_some()>
                    <nav aria-label="Main menu" id="nav" data-show=show_menu>
                        <label for="menu" aria-hidden="true">
                            <button class="close" on:click=move |_| { set_show_menu(false) }>
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 30 30"
                                    width="30px"
                                    height="30px"
                                >
                                    <path d="M 7 4 C 6.744125 4 6.4879687 4.0974687 6.2929688 4.2929688 L 4.2929688 6.2929688 C 3.9019687 6.6839688 3.9019687 7.3170313 4.2929688 7.7070312 L 11.585938 15 L 4.2929688 22.292969 C 3.9019687 22.683969 3.9019687 23.317031 4.2929688 23.707031 L 6.2929688 25.707031 C 6.6839688 26.098031 7.3170313 26.098031 7.7070312 25.707031 L 15 18.414062 L 22.292969 25.707031 C 22.682969 26.098031 23.317031 26.098031 23.707031 25.707031 L 25.707031 23.707031 C 26.098031 23.316031 26.098031 22.682969 25.707031 22.292969 L 18.414062 15 L 25.707031 7.7070312 C 26.098031 7.3170312 26.098031 6.6829688 25.707031 6.2929688 L 23.707031 4.2929688 C 23.316031 3.9019687 22.682969 3.9019687 22.292969 4.2929688 L 15 11.585938 L 7.7070312 4.2929688 C 7.5115312 4.0974687 7.255875 4 7 4 z"></path>
                                </svg>
                            </button>
                        </label>
                        <A
                            href=""
                            class="link"
                            exact=true
                            on:click=move |_| { set_show_menu(false) }
                        >
                            "dashboard"
                        </A>
                        <A href="/check_in" class="link" on:click=move |_| { set_show_menu(false) }>
                            "check "
                            {move || if status() { "out" } else { "in" }}
                        </A>
                        <A
                            href="/timesheet"
                            class="link"
                            on:click=move |_| { set_show_menu(false) }
                        >
                            "timesheet"
                        </A>

                        <A href="/timesheets" class="link">
                            "timesheets"
                        </A>
                        <A href="/vacations" class="link">
                            "vacations"
                        </A>
                        <A href="/users" class="link">
                            "users"
                        </A>
                        <A href="/settings" class="link">
                            "settings"
                        </A>

                        <ActionForm action=log_out>
                            <button type="submit">
                                <span>"logout"</span>
                                <span>
                                    <svg
                                        aria-hidden="true"
                                        focusable="false"
                                        data-prefix="fad"
                                        data-icon="sign-out-alt"
                                        className="svg-inline--fa fa-sign-out-alt fa-w-16"
                                        role="img"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 512 512"
                                        width="1em"
                                    >
                                        <g className="fa-group">
                                            <path
                                                className="fa-secondary"
                                                fill="currentColor"
                                                d="M64 160v192a32 32 0 0 0 32 32h84a12 12 0 0 1 12 12v40a12 12 0 0 1-12 12H96a96 96 0 0 1-96-96V160a96 96 0 0 1 96-96h84a12 12 0 0 1 12 12v40a12 12 0 0 1-12 12H96a32 32 0 0 0-32 32z"
                                                opacity="0.4"
                                            ></path>
                                            <path
                                                className="fa-primary"
                                                fill="currentColor"
                                                d="M288 424v-96H152a23.94 23.94 0 0 1-24-24v-96a23.94 23.94 0 0 1 24-24h136V88c0-21.4 25.9-32 41-17l168 168a24.2 24.2 0 0 1 0 34L329 441c-15 15-41 4.52-41-17z"
                                            ></path>
                                        </g>
                                    </svg>
                                </span>
                            </button>
                        </ActionForm>
                        <span>{VERSION}</span>
                    </nav>
                </Show>
                <main id="main">
                    // Add protected routes
                    <Routes>
                        <Route
                            path=""
                            view=move || {
                                view! {
                                    <Show
                                        when=move || user().is_some()
                                        fallback=move || view! { <Auth authenticate/> }
                                    >
                                        <Outlet/>
                                    </Show>
                                }
                            }
                        >

                            <Route path="/" view=move || view! { <HomePage status/> }/>
                            <Route path="/timesheet" view=TimeSheetDisplay/>
                            <Route path="/timesheet/missing" view=TimeSheetMissing/>
                            <Route path="/timesheets" view=TimeSheets>
                                <Route path="" view=TimeSheetsList/>
                                <Route path="/adjustment" view=TimeSheetsAdjustment/>
                                <Route path="/pending" view=TimeSheetsPending/>
                            </Route>
                            <Route path="/vacations" view=Vacations>
                                <Route path="" view=VacationsList/>
                                <Route path="/pending" view=VacationsPending/>
                                <Route path="/request" view=VacationRequest/>
                                <Route path="/:id" view=VacationEdit/>
                            </Route>
                            <Route path="/users" view=Users>
                                <Route path="" view=UsersList/>
                                <Route path="/create" view=UserCreate/>
                                <Route path="/:id" view=UserUpdate/>
                            </Route>
                            <Route
                                path="/check_in"
                                view=move || view! { <CheckInView check_in status/> }
                            />
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
    use crate::models::sessions::{close_session, get_open_sessions};
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

    leptos_axum::redirect("/");

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
struct PhoneQuery {
    phone: String,
}

#[component]
pub fn Auth(authenticate: Action<Authenticate, Result<(), ServerFnError>>) -> impl IntoView {
    let (_pin_input, set_pin_input) = create_signal(String::with_capacity(6));

    let phone_query = use_query::<PhoneQuery>();

    let (error_text, _set_error_text) = create_signal::<String>(String::new());
    let get_pin = create_server_action::<GetPin>();

    let pattern = "[0-9]{6}";

    let value = authenticate.value();

    view! {
        <Title text="Dental Care | Authenticating"/>
        <section class="center-center">

            <Show
                when=move || phone_query().is_ok()
                fallback=move || {
                    view! {
                        <ActionForm class="center-center" action=get_pin>
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
                            <button type="submit" disabled=get_pin.pending()>
                                "Get Pin"
                            </button>
                            <Show when=get_pin.pending()>
                                <div>"Loading..."</div>
                            </Show>
                            <div data-state="error">{error_text}</div>
                        </ActionForm>
                    }
                }
            >

                {move || match phone_query() {
                    Ok(query) => {
                        view! {
                            <ActionForm action=authenticate class="center-center">
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
                    }
                    Err(_e) => {
                        view! {
                            <ActionForm action=authenticate class="center-center">
                                <input type="hidden" value="" name="phone"/>
                                <input type="hidden" name="pin"/>
                                <Show when=move || value.with(Option::is_some)>
                                    <div>{value}</div>
                                </Show>
                            </ActionForm>
                        }
                    }
                }}

            </Show>
        </section>
    }
}

#[server]
async fn get_pin(phone: String) -> Result<Pin, ServerFnError> {
    use crate::models::user::get_user_by_phone;
    use crate::service::sms::send_message;

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

    leptos_axum::redirect(&("/?phone=".to_string() + &phone));

    Ok(pin)
}

#[component]
pub fn PhoneNumber() -> impl IntoView {
    let (error_text, _set_error_text) = create_signal::<String>(String::new());
    let get_pin = create_server_action::<GetPin>();
    view! {
        <Title text="Dental Care | Authentication"/>

        <ActionForm class="center-center" action=get_pin>
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
            <Show when=get_pin.pending()>
                <div>"Loading..."</div>
            </Show>
            <div data-state="error">{error_text}</div>
        </ActionForm>
    }
}