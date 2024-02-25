use leptos::*;
use crate::models::time_sheets::Entry as Entry;
use crate::components::session::Session;
use crate::components::adjustment::Adjustment;


/// Renders a view based on the type of the model `time_sheets::Entry`. Which displays a `Session` or `Adjustment`.
#[component]
pub fn Entry(entry: Entry) -> impl IntoView {
    match entry {        
        Entry::Session(session) => view! { <Session session/> },
        Entry::Adjustment(adjustment) => view! { <Adjustment adjustment/> }
    }
}
