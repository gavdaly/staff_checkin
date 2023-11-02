use crate::models::user::UserPublic;
use leptos::*;
use leptos_router::A;

/// Renders the home page of your application.
#[component]
pub fn HomePage<F>(user: F) -> impl IntoView where F: Fn() -> Option<UserPublic> {

    // get settings
    // show week summary
    // upcomming vacations
    view! {
        <section class="stack">
            {match user() {
                Some(u) => {
                    view! {
                        <div id=u
                            .id
                            .to_string()>
                            {move || match u.checked_in {
                                Some(t) => {
                                    view! {
                                        <A href="/check_in">
                                            <aside id="checked_in" data-checked-in=t.to_string()>
                                                {if t { "In" } else { "Out" }}

                                            </aside>
                                        </A>
                                    }
                                }
                                None => {
                                    view! {
                                        <A href="/">
                                            <aside data-state="warning">"You are Checked Out"</aside>
                                        </A>
                                    }
                                }
                            }}

                        </div>
                    }
                }
                None => view! { <div>"You Are Not Logged In"</div> },
            }}

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
