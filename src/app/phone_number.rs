use leptos::*;
use leptos_router::ActionForm;
use leptos_meta::*;
use crate::components::loading_progress::Loading;

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

#[server]
pub(crate) async fn submit_phone_number(phone: String) -> Result<(), ServerFnError> {
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

    let pin_number = pin.number.to_string();
    let message = format!("Your verification code is: {pin_number}. Do not share it.");
    send_message(message, format!("+1{phone}")).await;

    leptos_axum::redirect(&("/p/".to_string() + &phone));

    Ok(())
}