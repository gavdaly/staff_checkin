use leptos::*;
use uuid::Uuid;

use crate::models::user::UserUpdate;

#[server]
async fn submit_user_form(user_id: Option<Uuid>, first_name: String, last_name: String, phone_number: String, state: i32) -> Result<UserUpdate, ServerFnError> {
    match user_id {
        Some(id) => UserUpdate { id, first_name, last_name, phone_number, state }.update().await.or_else(|_| Err(ServerFnError::Request("Error Updating User".into()))),
        None => UserUpdate::insert(&first_name, &last_name, &phone_number, state).await.or_else(|_| Err(ServerFnError::Request("".into())))
    }
}

#[component]
pub fn UserForm(uuid: Option<Uuid>) -> impl IntoView {

    view! {
        <form>
            {match uuid {
                Some(u) => {
                    view! { <input name="user_id" type="hidden" value=u.to_string()/> }.into_view()
                }
                None => view! {}.into_view(),
            }}
            <div>
                <label>"First Name"</label>
                <input type="text" placeholder="First Name"/>
            </div> <div>
                <label>"Last Name"</label>
                <input type="text" placeholder="Last Name"/>
            </div> <div>
                <label>"Prefered Name"</label>
                <input type="prefered" placeholder="Prefered Name"/>
            </div> <div>
                <label>"Phone Number"</label>
                <input type="phone" placeholder="Phone Number"/>
            </div> <fieldset class="picklist" name="state">
                <label>"User Type"</label>
                <div>
                    <label for="user">"User"</label>
                    <input id="user" name="state" type="radio" value="2"/>
                </div>
                <div>
                    <label for="admin">"Admin"</label>
                    <input id="admin" name="state" type="radio" value="1"/>
                </div>
                <div>
                    <label for="inactive">"Inactive"</label>
                    <input id="inactive" name="state" type="radio" value="0"/>
                </div>
            </fieldset>
        </form>
    }
}
