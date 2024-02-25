use leptos::*;
use crate::models::adjustments::Adjustment as AdjustmentState;

/// This function generates a view component that displays information about an adjustment.
///
/// # Arguments
///
/// * `adjustment` - A reference to an `AdjustmentState` object that contains information about the adjustment.
///
/// # Returns
///
/// An implementation of the `IntoView` trait representing the view component.
#[component]
pub fn Adjustment(adjustment: AdjustmentState) -> impl IntoView {
    let category_string = |cat| match cat {
        0 => "Adjustment",
        1 => "Vacation",
        2 => "Statutory",
        _ => "Unknown"
    };
    let hours = match adjustment.duration {
        0 => String::new(),
        a => format!(" for {a} miliseconds")
    };
    view! {
        <div id=adjustment.id.to_string() data-category=category_string(adjustment.category)>
            {category_string(adjustment.category)}
            {hours}
        </div>
    }
}