use leptos::*;
use crate::models::time_sheets::Entry as Entry;
use crate::components::session::Session;
use crate::components::adjustment::Adjustment;


#[component]
pub fn Entry<'a>(entry: &'a Entry) -> impl IntoView {
    match entry {        
        Entry::Session(session) => view! { <Session session/> },
        Entry::Adjustment(adjustment) => view! { <Adjustment adjustment/> }
    }
}
