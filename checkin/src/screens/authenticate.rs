use crate::models::pins::Pin;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[server]
async fn get_pin(phone: String) -> Result<Pin, ServerFnError> {
    use crate::models::user::UserPublic;
    use crate::service::sms::send_message;

    let phone = crate::utils::filter_phone_number(&phone);
    let Ok(user) = UserPublic::get_phone(&phone).await else {
        return Err(ServerFnError::Deserialization(
            "Could not Find Phone Number!".into(),
        ));
    };
    let Ok(pin) = Pin::create_pin_for(user.id).await else {
        leptos::tracing::error!("Could not create pin: {}", user.id.to_string());
        return Err(ServerFnError::ServerError("Error Creating Pin!".into()));
    };

    send_message(pin.number.to_string(), format!("+1{phone}")).await;

    leptos_axum::redirect(&format!("/sign_in/{phone}"));
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

#[server]
async fn authenticate(pin: i32, phone: String) -> Result<(), ServerFnError> {
    use crate::models::user::UserPublic;
    use axum_session::SessionPgSession;

    let Ok(pin) = Pin::get_pin(pin).await else {
        return Err(ServerFnError::ServerError("Internal Server Error".into()));
    };

    let Ok(user) = UserPublic::get_phone(&phone).await else {
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

#[derive(Clone, Params, PartialEq)]
struct PhoneParams {
    phone: String,
}

#[component]
pub fn PinNumber() -> impl IntoView {
    let (_pin_input, set_pin_input) = create_signal(String::with_capacity(6));
    let authenticate = create_server_action::<Authenticate>();
    let phone = use_params::<PhoneParams>();

    let PhoneParams { phone } = phone().expect("There should be a parameter");
    // let navigate = use_navigate();
    // navigate("/sign_in", NavigateOptions::default());

    let pattern = "[0-9]{6}";
    let _options = PinPadOptions {
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
        <Title text="Dental Care | Authenticating"/>
        <section class="center-center">
            // <PinPad active={pin_input} options=&options />
            <ActionForm action=authenticate class="center-center">
                <input type="hidden" value=phone name="phone"/>
                <label id="pin">"Enter Pin From SMS"</label>
                <input
                    type="number"
                    name="pin"
                    pattern=pattern
                    inputMode="numeric"
                    on:input=move |v| set_pin_input(event_target_value(&v))
                />
                <button type="submit" disabled=authenticate.pending()>
                    "Log In"
                </button>
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
