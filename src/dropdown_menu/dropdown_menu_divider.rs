//! Menu separator adapted from Astryx DropdownMenuDivider.tsx.

use dioxus::prelude::*;

use crate::divider::Divider;

/// A nonfocusable horizontal separator between groups of menu items.
#[component]
pub fn DropdownMenuDivider(
    /// Caller classes appended to the separator's theme targets.
    #[props(default)]
    class: String,
    /// Caller inline declarations, including overrides for the default menu spacing.
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
            // Spread attributes may use mixed case, which HTML treats as equivalent.
            ![
                "role",
                "aria-orientation",
                "tabindex",
                "contenteditable",
                "autofocus",
            ]
            .iter()
            .any(|name| attribute.name.eq_ignore_ascii_case(name))
        })
        .collect();
    rsx! { Divider { class, style, onmounted, attributes } }
}
