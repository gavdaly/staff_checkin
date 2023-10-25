use crate::models::user::UserPublic;
use leptos::*;
use leptos_router::ActionForm;
use uuid::{uuid, Uuid};

#[server]
async fn get_user(id: Uuid) -> Result<UserPublic, ServerFnError> {
    use crate::models::user::UserPublic;

    // let _ = UserPublic::get(id);
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
            <Suspense fallback=|| view! {"Loading..."}>
                {move || view!{
                    {match user.get() {
                        Some(Ok(u)) => {
                            view!{
                                <div id={u.id.to_string()}>
                                    <h1>{u.first_name}</h1>
                                    <p>{u.phone_number}</p>
                                </div>
                            }

                        },
                        _ => view!{<div>"error"</div>}
                    }}
                }}
            </Suspense>
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
