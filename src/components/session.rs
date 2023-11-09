use leptos::*;
use crate::models::sessions::Session;
use crate::utils::miliseconds_to_string;

#[component]
pub fn Session<'a>(session: &'a Session) -> impl IntoView {
    view! {
        <div id=session.id.to_string()>
            <span>
                <time>{session.start_time.time().to_string()}</time>
            </span>
            {match session.end_time {
                Some(t) => {
                    view! {
                        <span>
                            <span>" to " <time>{t.time().to_string()}</time></span>
                            " = "
                            <span>
                                {miliseconds_to_string(&(t - session.start_time).num_milliseconds())
                                    .to_string()}
                            </span>
                        </span>
                    }
                }
                None => view! { <span>"Session not closed yet!"</span> },
            }}

        </div>
    }
}