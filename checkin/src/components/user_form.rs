use leptos::*;

#[component]
pub fn UserForm() -> impl IntoView {
    view! {
        <form>
            <div>
                <input type="text" placeholder="First Name"/>
            </div>
            <input type="text" placeholder="Last Name"/>
            <input type="prefered" placeholder="Prefered Name"/>

            <input type="phone" placeholder="Phone Number"/>

            <select>
                <option>"Inactive"</option>
                <option>"Hourly"</option>
                <option>"Salary"</option>
            </select>
        </form>
    }
}
