use crate::models::pins::Pin;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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

pub struct PinPadOptions {
    pub length: u8,
    pub complete: &'static str,
    pub incomplete: &'static str,
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
