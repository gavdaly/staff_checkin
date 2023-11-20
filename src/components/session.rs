use leptos::*;
use leptos_router::A;
use crate::models::sessions::Session;
use crate::utils::miliseconds_to_string;

#[component]
pub fn Session<'a>(session: &'a Session) -> impl IntoView {
    view! {
        <tr id=session.id.to_string()>
            <td>
                <time datetime=session
                    .start_time
                    .to_string()>{session.start_time.format("%I:%M %P").to_string()}</time>
            </td>
            {match session.end_time {
                Some(t) => {
                    view! {
                        <td>

                            <time datetime=t.to_string()>{t.format("%I:%M %P").to_string()}</time>
                        </td>

                        <td>
                            {miliseconds_to_string(&(t - session.start_time).num_milliseconds())
                                .to_string()}
                        </td>

                        {if session.state == 1 {
                            view! {
                                <td>
                                    <A href=format!("/app/timesheet/edit/{}", session.id)>edit</A>
                                </td>
                            }
                                .into_view()
                        } else {
                            view! {}.into_view()
                        }}
                    }
                        .into_view()
                }
                None => view! { <td>"Session not closed yet!"</td> }.into_view(),
            }}

        </tr>
    }
}