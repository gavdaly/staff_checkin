use leptos::*;
use leptos_router::*;

struct TimeSheetEntry {
    id: String,
    key: String,
    state: String,
    start_time: String,
    end_time: Option<String>,
    correction: Option<TimeSheetCorrection>,
}

struct TimeSheetCorrection {
    reason: String,
    start_time: String,
    end_time: String,
    original_start_time: String,
    original_end_time: String,
    response: Option<String>,
}

#[component]
fn Entry(entry: TimeSheetEntry) -> impl IntoView {
    let (show_edit, set_show_edit) = use_signal(false);
    let is_editable = entry.state == "pending" || entry.state == "editable";
    view! {
        <div class=format!("entry {entry.id} time state_{entry.state}")>
                  <div className="time_group">
                    <span className="original_start">
                      {entry.start_time}
                    </span>
                    <Show went=|| !entry.end_time.is_empty() fallback=|| view!{}>
                        " to "
                        <span className="original_end">
                          {entry.end_time}
                        </span>
                        <div>
                          <span>"Duration: "</span>
                          {entry.duration}
                          " h"
                        </div>
                      </Show>
                  </div>
                  <Show when=move || {entry.correction.is_some()} fallback=|| view!{}>
                      <div class="correction_reason">{correction.reason}</div>
                      <div class="time_group">
                        <span>"Requested: "</span>
                        <span class="requested_start">
                          {correction.start_time}
                        </span>
                        " to "
                        <span class="requested_end">
                          {correction.end_time}
                        </span>
                      </div>

                      <div class="time_group">
                        <span>"Original: "</span>
                        <span class="requested_start">
                          {correction.original_start_time}
                        </span>
                        " to "
                        <span class="requested_end">
                          {correction.original_end_time}
                        </span>
                      </div>
                        <div class="correction_explanation">
                          {correction.response}
                        </div>
                  </Show>

                  <button
                    disabled={!is_editable}
                    class="state"
                  >
                    {entry.state}
                  </button>
                </div>
                {show_edit && <EntryEdit entry={entry} set_show_edit={set_show_edit} />}
                // {entry.state === "pending" && currentUser.role === "admin" && (
                //   <AdminEntryPendingForm entry={entry} />
                // )}

    }
}

#[component]
fn EntryEdit(entry: TimeTableEntry) -> impl IntoView {
    view! {}
}
