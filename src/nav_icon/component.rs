//! Circular navigation mark from the pinned Astryx NavIcon.

use dioxus::prelude::*;

/// Centers caller-provided artwork on a circular accent surface.
///
/// The container adds no semantics; the artwork owns its accessible name or
/// decorative status.
#[component]
pub fn NavIcon(
    /// Artwork to display in the circle.
    icon: Element,
    /// Additional root classes.
    #[props(default)]
    class: String,
    /// Inline declarations on the root.
    #[props(default)]
    style: Option<String>,
    /// Native attributes on the root span, including mount events.
    #[props(extends = span, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let class = if class.is_empty() {
        "astryx-nav-icon".to_string()
    } else {
        format!("astryx-nav-icon {class}")
    };
    rsx! { span { class, style, ..attributes, {icon} } }
}
