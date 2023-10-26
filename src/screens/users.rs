use leptos::*;
use leptos_router::*;

/// Renders the home page of your application.
#[component]
pub fn Users() -> impl IntoView {
    view! {
        <nav class="subWrapper">
            <A href="" exact=true>
                "Users List"
            </A>
            <A href="create" exact=true>
                "Add New User"
            </A>
        </nav>
        <section class="stack">
            <Outlet/>
        </section>
    }
}

#[component]
pub fn UsersList() -> impl IntoView {
    view! { <h1>"Users"</h1> }
}

#[component]
pub fn UserCreate() -> impl IntoView {
    view! { <h1>"User Create"</h1> }
}

#[component]
pub fn UserUpdate() -> impl IntoView {
    view! { <h1>"User Update"</h1> }
}
