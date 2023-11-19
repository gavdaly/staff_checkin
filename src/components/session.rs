use leptos::*;
use leptos_router::A;
use crate::models::sessions::Session;
use crate::utils::miliseconds_to_string;

#[component]
pub fn Session<'a>(session: &'a Session) -> impl IntoView {
    view! {
        <div id=session.id.to_string()>
            <span>
                <time datetime=session.start_time.to_string()>{session.start_time.format("%I:%M %P").to_string()}</time>
            </span>
            {match session.end_time {
                Some(t) => {
                    view! {
                        <span>
                            <span>" to " <time datetime=t.to_string()>{t.format("%I:%M %P").to_string()}</time></span>
                            " = "
                            <span>
                                {miliseconds_to_string(&(t - session.start_time).num_milliseconds())
                                    .to_string()}
                            </span>
                        </span>" "
                        <A href=format!("/app/timesheet/edit/{}", session.id)>edit</A>
                    }
                        .into_view()
                }
                None => view! { <span>"Session not closed yet!"</span> }.into_view(),
            }}

        </div>
    }
}