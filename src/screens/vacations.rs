use leptos::*;
use leptos_router::*;

/// Renders the home page of your application.
#[component]
pub fn Vacations() -> impl IntoView {
    view! {
        <nav class="subWrapper">
            <A href="" exact=true>
                "Vacation List"
            </A>
            <A href="pending" exact=true>
                "Pending Vacations"
            </A>
            <A href="create" exact=true>
                "Request Vacation"
            </A>
        </nav>
        <section class="stack">
            <Outlet/>
        </section>
    }
}

#[component]
pub fn VacationsList() -> impl IntoView {
    view! { <h1>"Vacations List | To Do"</h1> }
}

#[component]
pub fn VacationsPending() -> impl IntoView {
    view! { <h1>"Vacations Pending"</h1> }
}

#[component]
pub fn VacationRequest() -> impl IntoView {
    view! { <h1>"Vacation Request"</h1> }
}

#[component]
pub fn VacationEdit() -> impl IntoView {
    view! { <h1>"Vacation Edit"</h1> }
}
