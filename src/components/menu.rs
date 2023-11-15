use leptos::*;
use leptos_router::*;
use crate::screens::authenticate::Logout;

#[component]
pub fn Menu<F>(status: F, log_out: Action<Logout, Result<(), ServerFnError>>) -> impl IntoView where F: Fn() -> bool + 'static {

    view! {
        <nav aria-label="Main menu" id="nav">
            <div id="menu" popover anchor="nav-button">
                <span>
                    <button class="close" popovertarget="menu">
                        "close"
                    </button>
                </span>
                <menu>
                    <li>
                        <A href="/app" class="link" exact=true>
                            "dashboard"
                        </A>
                    </li>
                    <li>
                        <A href="/app/check_in" class="link">
                            {move || if status() { "check out" } else { "check in" }}
                        </A>
                    </li>
                    <li>
                        <A href="/app/timesheet" class="link">
                            "timesheet"
                        </A>
                    </li>
                    <li>
                        <A href="/app/vacations" class="link">
                            "vacations"
                        </A>
                    </li>
                    <li>
                        <A href="/app/users" class="link">
                            "users"
                        </A>
                    </li>
                    <li>
                        <A href="/settings" class="link">
                            "settings"
                        </A>
                    </li>
                    // <li>
                    //     <A href="/admin/timesheets" class="link">
                    //         "timesheets"
                    //     </A>
                    // </li>
                </menu>

                <ActionForm action=log_out>
                    <button type="submit">
                        <span>"logout"</span>
                        <img href="/icons.svg#logout"/>
                    </button>
                </ActionForm>

            </div>
        </nav>
        <button id="nav-button" popovertarget="menu">open menu</button>
    }
}