use leptos::*;
use leptos_router::*;
use crate::screens::authenticate::Logout;
use crate::components::icon::Icon;

#[component]
pub fn Menu<F>(status: F, log_out: Action<Logout, Result<(), ServerFnError>>, show_menu: ReadSignal<bool>, set_show_menu: WriteSignal<bool>) -> impl IntoView where F: Fn() -> bool + 'static {

    view! {
        <nav aria-label="Main menu" id="nav" data-visible=move || show_menu().to_string()>
            <span>
                <button class="close nav-button" on:click=move |_| set_show_menu(false)>
                    <Icon name="close"/>
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
                <li>
                    <A href="/settings" class="link" on:click=move |_| set_show_menu(false)>
                        "settings"
                    </A>
                </li>
                <Show when=|| false>
                    <li>
                        <A href="/admin/timesheets" class="link">
                            "timesheets"
                        </A>
                    </li>
                </Show>
            </menu>

            <ActionForm action=log_out>
                <button type="submit">
                    <span>"Logout"</span>
                    <Icon name="logout"/>
                </button>
            </ActionForm>

        </nav>
        <button class="nav-button" on:click=move |_| set_show_menu(true)>
            <Icon name="horizontal-menu"/>
        </button>
    }
}