use leptos::*;
use leptos_router::*;
use crate::app::CheckIn;

#[component]
pub fn CheckInView<F>(check_in: Action<CheckIn, Result<(), ServerFnError>>, status: F) -> impl IntoView 
where F: Fn() -> bool + 'static {
    use leptos_use::UseWindow;

    let value = move || check_in.value();

    let window = leptos_use::use_window();

    // Checks to see if the component is SSR or Hydrated. Stops the crashing on SSR
    match window.is_some() {
        true => view! {
            <section class="center-center">
                <GeoCheckIn check_in status/>
            </section>
        },
        false => view! {
            <section class="center-center">
                // The window doesn't reprocess so send a link back to the route so there is no failure state
                <A href="/">"Refresh"</A>
            </section>
        }
    }
}

#[component]
fn GeoCheckIn<F>(check_in: Action<CheckIn, Result<(), ServerFnError>>, status: F) -> impl IntoView where F: Fn() -> bool + 'static {
    use leptos_use::{use_geolocation_with_options, UseGeolocationReturn};

    let options = leptos_use::UseGeolocationOptions::default().enable_high_accuracy(true);

    let UseGeolocationReturn {
        coords,
        located_at: _,
        error,
        resume: _,
        pause: _,
    } = use_geolocation_with_options(options);

    let stat = status();

    view! {
        <Show when=move || {
            coords.with(Option::is_some)
        }>
            {match coords() {
                Some(coords) => {
                    view! {
                        <div class="center-center">
                            <ActionForm class="center-center" action=check_in>
                                <input type="hidden" value=coords.latitude() name="latitude"/>
                                <input type="hidden" value=coords.longitude() name="longitude"/>
                                <input type="hidden" value=coords.accuracy() name="accuracy"/>
                                <button type="submit" data-size="huge" disable=check_in.pending()>
                                    "Check "
                                    {if stat { "Out" } else { "In" }}
                                </button>
                            </ActionForm>
                        </div>
                    }
                }
                None => {
                    view! { <div>"?"</div> }
                }
            }}

        </Show>

        <Show when=move || error().is_some()>
            <div data-state="error" class="center-center">
                {move || match error() {
                    Some(error) => location_error(error.code()),
                    None => "No Error code given".to_string(),
                }}

            </div>
        </Show>
    }

}

fn location_error(error_number: u16) -> String {
    match error_number {
        1 => "Location Services are disabled, please enable and try again.".to_string(),
        2 => "Error getting a signal for your location.".to_string(),
        3 => "Finding Location Took too long please try again.".to_string(),
        _ => "Unknow Error".to_string(),
    }
}