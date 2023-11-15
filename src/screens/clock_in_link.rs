use leptos::*;
use leptos_router::*;
use uuid::Uuid;

#[derive(Clone, Params, PartialEq)]
struct ClockInLinkParams {
    link: String,
}

#[component]
pub fn ClockInLink() -> impl IntoView {
    let params = use_params::<ClockInLinkParams>();
    let clock_in = create_server_action::<ClockInLinkInitiateSession>();
    match params() {
        Ok(ClockInLinkParams {link}) => { 
            view! {
            <div>"Loading..." {link}</div>
        }},
        Err(e) => view! {
            <div>"Something went wrong: "{e.to_string()}</div>    
        }
    }
}

#[server]
async fn clock_in_link_initiate_session(link: String, user_id: Uuid) -> Result<(), ServerFnError> {
    
    Ok(())
}