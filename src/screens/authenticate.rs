use leptos::*;
use leptos_router::*;
use crate::components::icon::Icon;

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
                                <button type="submit">
                                    <Icon name="login"/>
                                    <span>"Log In"</span>
                                </button>

                            </ActionForm>
                            <Show when=authenticate.pending()>
                                <div>"Loading..."</div>
                            </Show>
                            <Show when=move || value.with(Option::is_some)>
                                <div>{value}</div>
                            </Show>
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
    leptos_axum::redirect("/app");
    Ok(())
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use axum_session::SessionPgSession;
    let session = use_context::<SessionPgSession>()
        .ok_or_else(|| ServerFnError::ServerError("Session missing.".into()))?;
    session.clear();

    leptos_axum::redirect("/");
    Ok(())
}