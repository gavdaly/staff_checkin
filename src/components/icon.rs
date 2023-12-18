use leptos::*;

/// Creates an SVG icon element with the specified class and href attributes.
///
/// # Arguments
///
/// * `name` - The name of the icon.
#[component]
pub fn Icon<'a>(name: &'a str) -> impl IntoView {
    view! {
        <svg class=format!("icon {name}")>
            <use_ href=format!("/icons.svg#{name}")></use_>
        </svg>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_creation() {
        let name = "test";
        let binding = leptos::ssr::render_to_string(|| view!{ <Icon name />});
        let icon = binding.as_str();
        assert!(&icon.contains(r#"class=" icon test""#));
    }
}