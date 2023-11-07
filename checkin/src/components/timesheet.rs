use leptos::*;

use crate::components::entry::Entry;
use crate::models::sessions::Session;
use crate::models::time_sheets::TimeSheet;
use crate::utils::miliseconds_to_string;

#[component]
pub fn TimeSheetDisplay(timesheet: TimeSheet) -> impl IntoView {
    view! {
        <table>
            <thead>
                <tr>
                    <th data-title="day">"day"</th>
                    <th data-title="checkins">"checkins"</th>
                    <th data-title="adjustments">"adjustments"</th>
                    <th data-title="subtotal">"subtotal"</th>
                    <th data-title="statutory">"statutory"</th>
                    <th data-title="vacation">"vacation"</th>
                    <th data-title="total">"total"</th>
                </tr>
            </thead>
            {timesheet
                .summary
                .iter()
                .map(|(day, (time, b, c, d))| {
                    view! {
                        <tr>
                            <td data-title="day">{day.to_string()}</td>
                            <td data-title="checkins">{miliseconds_to_string(time)}</td>
                            <td data-title="adjustments">{miliseconds_to_string(b)}</td>
                            <td data-title="subtotal">{miliseconds_to_string(&(time + b))}</td>
                            <td data-title="statutory">{miliseconds_to_string(c)}</td>
                            <td data-title="vacation">{miliseconds_to_string(d)}</td>
                            <td data-title="total">{miliseconds_to_string(&(time + b + c + d))}</td>
                        </tr>
                    }
                })
                .collect_view()}
        </table>
        <table>
            <thead>
                <tr>
                    <th>"day"</th>
                    <th>"entries"</th>
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
fn EntryEdit(_entry: Session) -> impl IntoView {
    view! { <div></div> }
}

