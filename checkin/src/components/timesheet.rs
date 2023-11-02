use leptos::*;
use leptos_router::*;
use crate::components::entry::Entry;
use crate::models::sessions::Session;
use crate::models::time_sheets::TimeSheet;

#[component]
pub fn TimeSheetDisplay(timesheet: TimeSheet) -> impl IntoView {
    view! {
        {timesheet
            .entries
            .iter()
            .map(|(day, entries)| {
                view! {
                    <div>
                        <div>{day.to_string()}</div>
                        {entries
                            .into_iter()
                            .map(|entry| view! { <Entry entry=entry/> })
                            .collect_view()}
                    </div>
                }
            })
            .collect_view()}
    }
}

#[component]
fn EntryEdit(entry: Session) -> impl IntoView {
    view! { <div></div> }
}

