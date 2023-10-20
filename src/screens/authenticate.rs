use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[server]
async fn get_pin(phone: String) -> Result<String, ServerFnError> {
    let phone = crate::utils::filter_phone_number(&phone);
    // find users phone number and make a pin
    //
    leptos_axum::redirect(&format!("/sign_in/{phone}"));
    Ok(phone)
}

#[component]
pub fn PhoneNumber() -> impl IntoView {
    let (error_text, _set_error_text) = create_signal::<String>(String::new());
    let get_pin = create_server_action::<GetPin>();
    view! {
        <Title text="Dental Care | Authentication" />

        <ActionForm class="center-center" action=get_pin>
            <div data-state="error">{error_text}</div>
            <label>"Phone Number"</label>
            <input id="phone" label="Phone Number"
                type="tel"
                name="phone"
                autoComplete="tel"
                placeholder="+1 (893) 234-2345"
                inputMode="tel"
                required
            />
            <button type="submit" disabled=get_pin.pending()>"Get Pin"</button>
            <Show when=get_pin.pending()>
                <div>"Loading..."</div>
            </Show>
        </ActionForm>
    }
}

#[server]
async fn authenticate(pin: String) -> Result<String, ServerFnError> {
    // use axum_session::{SessionConfig, SessionLayer, SessionStore};
    // use axum_session_auth::{AuthConfig, AuthSessionLayer, SessionSqlitePool};
    // let params = ParamsMap::get("");
    // match the phone number and pin
    //
    println!("{pin}");

    // use actix_web::{cookie::Cookie, http::header, http::header::HeaderValue};
    // use leptos_actix::ResponseOptions;

    // pull ResponseOptions from context
    // let response = expect_context::<ResponseOptions>();

    // set the HTTP status code
    // response.set_status(StatusCode::IM_A_TEAPOT);

    // set a cookie in the HTTP response
    // let mut cookie = Cookie::build("biscuits", "yes").finish();
    // if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
    //     res.insert_header(header::SET_COOKIE, cookie);
    // }

    // let headers = Headers([(SET_COOKIE, "auth=Authorized")]);
    // auth.login_user(user.id);
    // auth.remember_user(remember.is_some());

    // redirect to the home page
    leptos_axum::redirect("/");

    Ok("Ok".to_string())
}

#[component]
pub fn PinNumber() -> impl IntoView {
    let (pin_input, set_pin_input) = create_signal(String::with_capacity(6));
    let authenticate = create_server_action::<Authenticate>();
    let pattern = "[0-9]{6}";
    let options = PinPadOptions {
        ..Default::default()
    };

    // let input_length = move || {
    //     let a = pin_input();
    //     let length = a.chars().count();
    //     u8::try_from(length).unwrap_or(0)
    // };

    let value = authenticate.value();

    //
    view! {
        <Title text="Dental Care | Authenticating" />
        <section class="center-center">
            // <PinPad active={pin_input} options=&options />
            <ActionForm action=authenticate class="center-center">
                <label id="pin">"Enter Pin From SMS"</label>
                <input
                    type="number"
                    name="pin"
                    pattern=pattern
                    inputMode="numeric"
                    on:input=move |v| set_pin_input(event_target_value(&v))
                />
                <button type="submit" disabled=authenticate.pending()>"Log In"</button>
                <Show when=authenticate.pending()>
                    <div>"Loading..."</div>
                </Show>
                <Show when=move || value.with(Option::is_some)>
                    <div>{value}</div>
                </Show>
            </ActionForm>
        </section>
    }
}

pub struct PinPadOptions {
    length: u8,
    complete: &'static str,
    incomplete: &'static str,
}

impl Default for PinPadOptions {
    fn default() -> PinPadOptions {
        PinPadOptions {
            length: 6,
            complete: "🌟",
            incomplete: "⭐️",
        }
    }
}

#[component]
pub fn PinPad<'a>(active: ReadSignal<String>, options: &'a PinPadOptions) -> impl IntoView {
    view! {
        <div>
            {(0..active().chars().count())
                .map(|_| {
                    view! {
                        <span role="img" aria-label="completed">
                            {options.complete}
                        </span>
                    }
                })
                .collect::<Vec<_>>()}
            {(0..(options.length as usize - active().chars().count()))
                .map(|_| {
                    view! {
                        <span role="img" aria-label="completed">
                            {options.incomplete}
                        </span>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
