use crate::components::icon::Icon;
use crate::models::user::UserDisplay;
use crate::screens::authenticate::Logout;
use leptos::*;
use leptos_router::*;

/// Generates a menu component for a web application.
///
/// # Arguments
///
/// * `status` - A function that returns a boolean value indicating the current status.
/// * `log_out` - An action that triggers the logout functionality.
/// * `show_menu` - A signal variable that indicates whether the menu should be shown or hidden.
/// * `set_show_menu` - A signal variable that allows updating the value of `show_menu`.
#[component]
pub fn Menu<F, U>(
    status: F,
    user: U,
    log_out: Action<Logout, Result<(), ServerFnError>>,
    show_menu: ReadSignal<bool>,
    set_show_menu: WriteSignal<bool>,
) -> impl IntoView
where
    F: Fn() -> bool + 'static,
    U: Fn() -> UserDisplay + 'static,
{
    view! {
        <nav aria-label="Main Menu" id="nav" data-visible=move || show_menu().to_string()>
            <span>
                <button class="close nav-button" on:click=move |_| set_show_menu(false)>
                    <Icon name="close".into()/>
                </button>
            </span>
            <menu>
                <li>
                    <A href="/app" on:click=move |_| set_show_menu(false) class="link" exact=true>
                        "dashboard"
                    </A>
                </li>
                <li>
                    <A href="/app/check_in" class="link" on:click=move |_| set_show_menu(false)>
                        {move || if status() { "check out" } else { "check in" }}
                    </A>
                </li>
                <li>
                    <A href="/app/timesheet" class="link" on:click=move |_| set_show_menu(false)>
                        "timesheet"
                    </A>
                </li>
                <li>
                    <A href="/app/vacations" class="link" on:click=move |_| set_show_menu(false)>
                        "vacations"
                    </A>
                </li>
                <li>
                    <A href="/app/users" class="link" on:click=move |_| set_show_menu(false)>
                        "users"
                    </A>
                </li>
                <Show when=move || user().state == 1>
                    <h2>"Admin"</h2>
                    <li>
                        <A href="/admin/timesheets" class="link">
                            "timesheets"
                        </A>
                    </li>
                    <li>
                        <A href="/admin/vacations" class="link">
                            "vacations"
                        </A>
                    </li>
                </Show>
            </menu>

            <ActionForm action=log_out>
                <button type="submit">
                    <span>"Logout"</span>
                    <Icon name="logout".into()/>
                </button>
            </ActionForm>
        </nav>
        <button class="nav-button" on:click=move |_| set_show_menu(true)>
            <Icon name="horizontal-menu".into()/>
        </button>
    }
}
