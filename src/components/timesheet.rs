use leptos::*;

use crate::components::entry::Entry;
use crate::models::sessions::Session;
use crate::models::time_sheets::TimeSheet;
use crate::utils::miliseconds_to_string;

#[component]
pub fn TimeSheetDisplay(timesheet: TimeSheet) -> impl IntoView {
    view! {
        <table id="timesheet_summary">
            <thead>
                <tr>
                    <th>"Day"</th>
                    <th>"Checkins"</th>
                    <th>"Adjustments"</th>
                    <th>"Subtotal"</th>
                    <th>"Statutory"</th>
                    <th>"Vacation"</th>
                    <th>"Total"</th>
                </tr>
            </thead>
            {timesheet
                .summary
                .iter()
                .map(|(day, (time, b, c, d))| {
                    view! {
                        <tr>
                            <td data-title="Day">{day.to_string()}</td>
                            <td data-title="Checkins">{miliseconds_to_string(time)}</td>
                            <td data-title="Adjustments">{miliseconds_to_string(b)}</td>
                            <td data-title="Subtotal">{miliseconds_to_string(&(time + b))}</td>
                            <td data-title="Statutory">{miliseconds_to_string(c)}</td>
                            <td data-title="Vacation">{miliseconds_to_string(d)}</td>
                            <td data-title="Total">{miliseconds_to_string(&(time + b + c + d))}</td>
                        </tr>
                    }
                })
                .collect_view()}
        </table>

        <table>
            <thead>
                <tr>
                    <th>"Day"</th>
                    <th>"Entries"</th>
                </tr>
            </thead>

            {timesheet
                .entries
                .iter()
                .map(|(day, entries)| {
                    view! {
                        <tr class="entry">
                            <td>{day.to_string()}</td>
                            <td>
                                {entries
                                    .iter()
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
fn EntryEdit(_entry: Session) -> impl IntoView {
    view! { <div></div> }
}
