use leptos::*;
use leptos_router::*;
use crate::components::entry::Entry;
use crate::models::sessions::Session;
use crate::models::time_sheets::TimeSheet;
use crate::utils::miliseconds_to_string;
// summary: {2023-10-30: (1552232, 0, 0, 0)}

#[component]
pub fn TimeSheetDisplay(timesheet: TimeSheet) -> impl IntoView {
    view! {
        <table>
            <thead>
                <tr>
                    <th>"day"</th>
                    <th>"checkins"</th>
                    <th>"adjustments"</th>
                    <th>"subtotal"</th>
                    <th>"statutory"</th>
                    <th>"vacation"</th>
                    <th>"total"</th>
                </tr>
            </thead>
            {timesheet
                .summary
                .iter()
                .map(|(day, (time, b, c, d))| {
                    view! {
                        <tr>
                            <td>{day.to_string()}</td>
                            <td>{miliseconds_to_string(time)}</td>
                            <td>{miliseconds_to_string(b)}</td>
                            <td>{miliseconds_to_string(&(time + b))}</td>
                            <td>{miliseconds_to_string(c)}</td>
                            <td>{miliseconds_to_string(d)}</td>
                            <td>{miliseconds_to_string(&(time + b + c + d))}</td>
                        </tr>
                    }
                })
                .collect_view()}
        </table>
        <table>
            {timesheet
                .entries
                .iter()
                .map(|(day, entries)| {
                    view! {
                        <tr class="entry">
                            <td>{day.to_string()}</td>
                            <td>
                                {entries
                                    .into_iter()
                                    .map(|entry| view! { <Entry entry=entry/> })
                                    .collect_view()}
                            </td>
                        </tr>
                    }
                })
                .collect_view()}
        </table>
    }
}

#[component]
fn EntryEdit(entry: Session) -> impl IntoView {
    view! { <div></div> }
}

