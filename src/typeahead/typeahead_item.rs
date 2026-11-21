use dioxus::prelude::*;

/// Content of a standalone search result. Search identity belongs to the source.
#[derive(Clone, PartialEq)]
pub struct TypeaheadItemContent {
    /// Default visible label.
    pub label: String,
    /// Complete replacement for the default layout. The replacement owns its root.
    pub element: Option<Element>,
}

/// Displays a search result without adding option semantics or activation.
/// The surrounding typeahead owns selection, focus, and disabled behavior.
#[component]
pub fn TypeaheadItem(
    /// Result content.
    item: TypeaheadItemContent,
    /// Leading icon or avatar.
    #[props(default)]
    icon: Option<Element>,
    /// Supporting line beneath the label.
    #[props(default)]
    description: Option<String>,
    /// Dim the default layout without changing its semantics.
    #[props(default)]
    is_disabled: bool,
    /// Additional classes on the default root.
    #[props(default)]
    class: String,
    /// Inline declarations on the default root.
    #[props(default)]
    style: Option<String>,
    /// Native attributes and events for the default root.
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    if let Some(element) = item.element {
        return element;
    }
    let class = if class.is_empty() {
        "astryx-typeahead-item".to_owned()
    } else {
        format!("astryx-typeahead-item {class}")
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| attr.name != "data-disabled")
        .collect();
    rsx! {
        div { class, style, "data-disabled": is_disabled.then_some("true"), ..attributes,
            {icon}
            div { class: "astryx-typeahead-item__content",
                span { class: "astryx-typeahead-item__label", "{item.label}" }
                if let Some(description) = description.filter(|text| !text.is_empty()) {
                    span { class: "astryx-typeahead-item__description", "{description}" }
                }
            }
        }
    }
}
