use chrono::Local;
use leptos::*;
use leptos_router::A;
use crate::models::sessions::Session;
use crate::utils::miliseconds_to_string;

#[component]
pub fn Session<'a>(session: &'a Session) -> impl IntoView {
    let id = session.id.to_string();
    let start_time = session.start_time.with_timezone(&Local);
    let start_string = start_time.format("%I:%M %P").to_string();
    let start_dt = start_time.to_string();
    let end_time = match session.end_time {
        Some(t) => Some(t.with_timezone(&Local)),
        None => None,
    };
    view! {
        <tr id=id.to_string()>
            <td>
                <time datetime=start_dt>{start_string}</time>
            </td>
            {match end_time {
                Some(t) => {
                    view! {
                        <td>

                            <time datetime=t.to_string()>{t.format("%I:%M %P").to_string()}</time>
                        </td>

                        <td>
                            {miliseconds_to_string(&(t - start_time).num_milliseconds())
                                .to_string()}
                        </td>

                        {match session.state {
                            0 => view! { <td>"open"</td> }.into_view(),
                            1 => {
                                view! {
                                    <td>
                                        <A href=format!("/app/timesheet/edit/{}", id)>edit</A>
                                    </td>
                                }
                                    .into_view()
                            }
                            2 => {
                                view! {
                                    <td>
                                        <A href=format!("/app/timesheet/edit/{}", id)>error</A>
                                    </td>
                                }
                                    .into_view()
                            }
                            3 => view! { <td>"pending"</td> }.into_view(),
                            4 => view! { <td>"accepted"</td> }.into_view(),
                            5 => view! { <td>"rejected"</td> }.into_view(),
                            6 => view! { <td>"done"</td> }.into_view(),
                            _ => view! { <td data-state="error">"ERROR"</td> }.into_view(),
                        }}
                    }
                        .into_view()
                }
                None => view! { <td>"Session not closed yet!"</td> }.into_view(),
            }}

        </tr>
    }
}