use crate::app::CheckIn;
use leptos::*;
use leptos_router::*;

/// This function returns a view component based on whether it is being rendered on the server side or client side.
///
/// # Arguments
///
/// * `check_in` - An `Action` object representing a check-in action.
#[component]
pub fn CheckInView(check_in: Action<CheckIn, Result<(), ServerFnError>>) -> impl IntoView {    
    // use leptos_use::{use_geolocation_with_options, UseGeolocationReturn};

    // let options = leptos_use::UseGeolocationOptions::default().enable_high_accuracy(true);

    // let UseGeolocationReturn {
    //     coords,
    //     located_at: _,
    //     error,
    //     resume: _,
    //     pause: _,
    // } = use_geolocation_with_options(options);

    view! {
        <div class="center-center stack">
                            <ActionForm class="center-center" action=check_in>
                                <input type="hidden" value="2" name="latitude"/>
                                <input type="hidden" value="2" name="longitude"/>
                                <input type="hidden" value="2" name="accuracy"/>
                                <button type="submit" data-size="huge">
                                    "Click"
                                </button>
                            </ActionForm>
                        </div>
        // <Show when=move || {
        //     coords.with(Option::is_some)
        // }>
        //     {match coords() {
        //         Some(coords) => {
        //             view! {
        //                 <div class="center-center stack">
        //                     <ActionForm class="center-center" action=check_in>
        //                         <input type="hidden" value=coords.latitude() name="latitude"/>
        //                         <input type="hidden" value=coords.longitude() name="longitude"/>
        //                         <input type="hidden" value=coords.accuracy() name="accuracy"/>
        //                         <button type="submit" data-size="huge">
        //                             "Click"
        //                         </button>
        //                     </ActionForm>
        //                 </div>
        //             }
        //         }
        //         None => {
        //             view! { <div class="center-center stack">"Shound not show"</div> }
        //         }
        //     }}

        // </Show>

        // <Show when=move || { coords.with(Option::is_none) }>
        //     <div class="center-center">"Getting Location"</div>
        // </Show>

        // <Show when=move || error().is_some()>
        //     <div data-state="error" class="center-center stack">
        //         {move || match error() {
        //             Some(error) => location_error(error.code()),
        //             None => "No Error code given".to_string(),
        //         }}

        //     </div>
        // </Show>
    }
}


/// Returns a string message based on the given error number.
///
/// # Arguments
///
/// * `error_number` - An unsigned 16-bit integer representing the error code.
fn _location_error(error_number: u16) -> String {
    match error_number {
        1 => "Location Services are disabled, please enable and try again.".to_string(),
        2 => "Error getting a signal for your location.".to_string(),
        3 => "Finding Location Took too long please try again.".to_string(),
        _ => "Unknow Error".to_string(),
    }
}
