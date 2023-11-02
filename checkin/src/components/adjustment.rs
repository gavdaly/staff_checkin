use leptos::*;
use crate::models::adjustments::Adjustment as AdjustmentState;

#[component]
pub fn Adjustment<'a>(adjustment: &'a AdjustmentState) -> impl IntoView {
    let category_string = |cat| match cat {
        0 => "Adjustment",
        1 => "Vacation",
        2 => "Statutory",
        _ => "Unknown"
    };
    let hours = match adjustment.duration {
        0 => "".to_string(),
        a => format!(" for {a} miliseconds")
    };
    view! {
        <div id=adjustment.id.to_string() data-category=category_string(adjustment.category)>
            {category_string(adjustment.category)}
            {hours}
        </div>
    }
}