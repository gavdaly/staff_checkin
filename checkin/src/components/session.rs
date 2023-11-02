use leptos::*;
use crate::models::sessions::Session;

#[component]
pub fn Session<'a>(session: &'a Session) -> impl IntoView {
    view! {
        <div id=session.id.to_string()>
            <span>"Start: " <time>{session.start_time.to_string()}</time></span>
            {match session.end_time {
                Some(t) => {
                    view! {
                        <span>
                            <span>"End: " <time>{t.to_string()}</time></span>
                            "   "
                            <span>{(t - session.start_time).to_string()}</span>
                        </span>
                    }
                }
                None => view! { <span>"Session not closed yet!"</span> },
            }}

        </div>
    }
}