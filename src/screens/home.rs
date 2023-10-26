use crate::models::user::UserPublic;
use leptos::*;
use leptos_router::ActionForm;
use uuid::{uuid, Uuid};

#[server]
async fn get_user(id: Uuid) -> Result<UserPublic, ServerFnError> {
    use crate::models::user::UserPublic;

    let _ = UserPublic::get(id);
    Ok(UserPublic {
        id: Uuid::new_v4(),
        first_name: Some("Test".to_string()),
        last_name: Some("User".to_string()),
        phone_number: Some("2341234567".to_string()),
        state: Some(2),
        role: Some(3),
    })
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let user = create_resource(
        || {},
        move |_| get_user(uuid! {"67e55044-10b1-426f-9247-bb680e5fe0c8"}),
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
async fn check_in(latitude: f64, longitude: f64, accuracy: f64) -> Result<String, ServerFnError> {
    // Get User

    Ok(format!("{latitude} | {longitude} | {accuracy}"))

    // leptos_axum::redirect("/");
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
                            <div>
                                <ActionForm action=check_in>
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
                <div data-state="error">
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
                <div>{value}</div>
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
