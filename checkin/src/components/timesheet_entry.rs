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

// #[component]
// fn Entry(entry: TimeSheetEntry) -> impl IntoView {
//     let (show_edit, set_show_edit) = create_signal(false);
//     let is_editable = entry.state == "pending" || entry.state == "editable";
//     view! {
//         <div class=format!("entry {} time state_{}", entry.id, entry.state)>
//                   <div className="time_group">
//                     <span className="original_start">
//                       {entry.start_time}
//                     </span>
//                     <Show when=|| entry.end_time.is_some() fallback=|| view!{}>
//                         " to "
//                         <span className="original_end">
//                           {entry.end_time}
//                         </span>
//                         <div>
//                           <span>"Duration: "</span>
//                           {entry.end_time}" - "{entry.start_time}
//                           " h"
//                         </div>
//                       </Show>
//                   </div>
//                   // <Show when=move || {entry.correction.is_some()} fallback=|| view!{}>
//                   //     <div class="correction_reason">{entry.correction.reason}</div>
//                   //     <div class="time_group">
//                   //       <span>"Requested: "</span>
//                   //       <span class="requested_start">
//                   //         {entry.correction.start_time}
//                   //       </span>
//                   //       " to "
//                   //       <span class="requested_end">
//                   //         {entry.correction.end_time}
//                   //       </span>
//                   //     </div>

//                   //     <div class="time_group">
//                   //       <span>"Original: "</span>
//                   //       <span class="requested_start">
//                   //         {entry.correction.original_start_time}
//                   //       </span>
//                   //       " to "
//                   //       <span class="requested_end">
//                   //         {entry.correction.original_end_time}
//                   //       </span>
//                   //     </div>
//                   //       <div class="correction_explanation">
//                   //         {entry.correction.response}
//                   //       </div>
//                   // </Show>

                //   <button
                //     disabled={!is_editable}
                //     class="state"
                //   >
                //     {entry.state}
                //   </button>
                // </div>
                // {<EntryEdit entry={entry} set_show_edit={set_show_edit} />}
                // {entry.state === "pending" && currentUser.role === "admin" && (
                //   <AdminEntryPendingForm entry={entry} />
                // )}


//     }
// }

#[component]
fn EntryEdit(entry: TimeSheetEntry) -> impl IntoView {
    view! { <div></div> }
}

