use crate::error_template::{AppError, ErrorTemplate};
use crate::models::user::UserPublic;
use crate::screens::authenticate::{PhoneNumber, PinNumber};
use crate::screens::home::{CheckIn, HomePage, Settings};
use crate::screens::timesheet::{DispalyTimeSheet, TimeSheetDisplay, TimeSheetMissing};
use crate::screens::timesheets::{
    TimeSheets, TimeSheetsAdjustment, TimeSheetsList, TimeSheetsPending,
};
use crate::screens::users::{UserCreate, UserUpdate, Users, UsersList};
use crate::screens::vacations::{
    VacationEdit, VacationRequest, Vacations, VacationsList, VacationsPending,
};
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
    let (show_menu, set_show_menu) = create_signal(false);
    let (user, set_user) = create_signal(None::<UserPublic>);

    provide_context(user);

    let user_fetch = create_resource(|| {}, |_| get_curent_user());

    create_effect(move |_| {
        if let Some(Ok(usr)) = user_fetch() {
            leptos_dom::log!("{:?}", usr);
            set_user(usr);
        } else {
            set_user(None);
        }
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/staff.css"/>

        // sets the document title
        <Title text="Dental Care"/>

        <div id="layout">
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors/> }.into_view()
            }>
                <header id="header">
                    <input type="checkbox" class="sr-only" id="menu" name="menu"/>
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
                    <h1>"Dental Care"</h1>
                </header>

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
                    <A href="" class="link" exact=true>
                        "dashboard"
                    </A>
                    <A href="/check_in" class="link">
                        "check in"
                    </A>
                    <A href="/timesheet" class="link">
                        "timesheet"
                    </A>
                    // <Show />
                    // <A href="/timesheets" class="link">
                    // "timesheets"
                    // </A>
                    // <A href="/vacations" class="link">
                    // "vacations"
                    // </A>
                    // <A href="/users" class="link">
                    // "users"
                    // </A>
                    // <A href="/settings" class="link">
                    // "settings"
                    // </A>
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
                <main id="main">
                    // Add protected routes
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/sign_in" view=|| view! { <Outlet/> }>
                            <Route path="/" view=PhoneNumber/>
                            <Route path="/:phone" view=PinNumber/>
                        </Route>
                        <Route path="/timesheet" view=DispalyTimeSheet>
                            <Route path="" view=TimeSheetDisplay/>
                            <Route path="/missing" view=TimeSheetMissing/>
                        </Route>
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
                        <Route path="/check_in" view=CheckIn/>
                        <Route path="/settings" view=Settings/>
                    </Routes>
                </main>
            </Router>
        </div>
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

    // redirect to the home page
    leptos_axum::redirect("/sign_in");
    Ok(())
}

#[server]
async fn get_curent_user() -> Result<Option<UserPublic>, ServerFnError> {
    use uuid::Uuid;
    use axum_session::SessionPgSession;

    let Some(session) = use_context::<SessionPgSession>() else {
        leptos::tracing::error!("Error getting settion");
        return Err(ServerFnError::ServerError("Error Finding Session 30".into()));
    };

    let Some(id) = session.get::<Uuid>("id") else {
        return Ok(None);
    };

    let Ok(user) = UserPublic::get(id).await else {
        leptos::tracing::error!("Could not find User for session");
        return Err(ServerFnError::ServerError("Could Not Find User.".into()));
    };

    Ok(Some(user))
}
