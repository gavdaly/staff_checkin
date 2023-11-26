use leptos::*;
use leptos_router::*;
use uuid::Uuid;
use crate::components::icon::Icon;
use super::timesheets::load_hourly_users;
use crate::components::user_form::UserForm;

/// Renders the home page of your application.
#[component]
pub fn Users() -> impl IntoView {
    let users = create_resource(move || {}, move |_| load_hourly_users());
    view! {
        <section class="stack users_list">
            <table>
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Phone Number</th>
                    </tr>
                </thead>
                {move || match users() {
                    Some(Ok(users)) => {
                        users
                            .into_iter()
                            .map(|user| {
                                view! {
                                    <tr>
                                        <td>{user.last_name} ", " {user.first_name}</td>
                                        <td>{user.phone_number}</td>
                                    </tr>
                                }
                            })
                            .collect_view()
                    }
                    Some(Err(e)) => {
                        view! { <div>format!("Error: {}", e.to_string())</div> }.into_view()
                    }
                    None => view! {}.into_view(),
                }}

            </table>
        </section>
    }
}

#[component]
pub fn AdminUsers() -> impl IntoView {
    view! {
        <nav class="subWrapper">
            <A href="" exact=true>
                "Users List"
            </A>
            <A href="create" exact=true>
                "Add New User"
            </A>
        </nav>
        <section class="stack admin users_list">
            <Outlet/>
        </section>
    }
}

#[component]
pub fn UsersList() -> impl IntoView {
    let users = create_resource(move || {}, move |_| load_hourly_users());
    view! {
        <section class="stack">
            {move || match users() {
                Some(Ok(users)) => {
                    view! {
                        <table>
                            <thead>
                                <tr>
                                    <th>Name</th>
                                    <th>Phone Number</th>
                                    <th>Edit</th>
                                </tr>
                            </thead>
                            {users
                                .into_iter()
                                .map(|user| {
                                    view! {
                                        <div class="user_list">
                                            <span>{user.last_name} ", " {user.first_name}</span>
                                            <span>{user.phone_number}</span>
                                            <span>
                                                <A href=format!("/admin/user/edit/{}", user.id.to_string())>
                                                    <Icon name="pencil"/>
                                                </A>
                                            </span>
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </table>
                    }
                        .into_view()
                }
                Some(Err(e)) => view! { <div>format!("Error: {}", e.to_string())</div> }.into_view(),
                None => view! {}.into_view(),
            }}

        </section>
    }
}


#[component]
pub fn UserCreate() -> impl IntoView {
    view! { <UserForm uuid=None/> }
}

#[derive(Clone, Params, PartialEq)]
struct UserUpdateP {
    uuid: Uuid
}

#[component]
pub fn UserUpdate() -> impl IntoView {
    let params = use_params::<UserUpdateP>();
    {move || match params() {
        Ok(UserUpdateP {uuid}) => view! { <UserForm uuid=Some(uuid)/> }.into_view(),
        Err(e) => view! { <div data-state="error">{e.to_string()}</div> }.into_view(),
    }}
}
