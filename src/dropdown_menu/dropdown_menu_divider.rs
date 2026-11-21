//! Menu separator adapted from Astryx DropdownMenuDivider.tsx.

use dioxus::prelude::*;

use crate::divider::Divider;

/// A nonfocusable horizontal separator between groups of menu items.
#[component]
pub fn DropdownMenuDivider(
    /// Caller classes appended to the separator's theme targets.
    #[props(default)]
    class: String,
    /// Caller inline declarations on the separator.
    #[props(default)]
    style: String,
    /// Mounted callback on the separator element.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native root attributes, except owned semantics and focusability.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let class = if class.is_empty() {
        "astryx-dropdown-menu-divider".into()
    } else {
        format!("astryx-dropdown-menu-divider {class}")
    };
    let attributes = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "role" | "aria-orientation" | "tabindex" | "contenteditable" | "autofocus"
            )
        })
        .collect();
    rsx! { Divider { class, style, onmounted, attributes } }
}
