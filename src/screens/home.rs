use leptos::*;
use leptos_router::ActionForm;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // get user
    // get settings
    // account state

    // show logged in status
    // show week summary
    // upcomming vacations
    view! {
        <section class="stack">
            "checking"
        </section>
    }
}

#[server]
async fn check_in() -> Result<String, ServerFnError> {
    leptos_axum::redirect("/");
    Ok("CheckInResult::Ok".to_string())
}

// #[server]
// async fn get_status() -> Result<String, ServerFnError> {
//     Ok("CheckedStatus::In".to_string())
// }

#[component]
pub fn CheckIn() -> impl IntoView {
    let check_in = create_server_action::<CheckIn>();
    let value = check_in.value();
    view! {
        <section class="center-center">
            <ActionForm action=check_in>
                <button type="submit" data-size="huge" disable=check_in.pending()>"Check In"</button>
            </ActionForm>
            <Show when=check_in.pending()>
                <div>"Loading..."</div>
            </Show>
            <Show when=move || value.with(Option::is_some)>
                <div>{value}</div>
            </Show>
        </section>
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
