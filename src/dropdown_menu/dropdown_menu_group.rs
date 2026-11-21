//! Menu grouping adapted from Astryx DropdownMenuGroup.tsx.

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;

/// Groups related menu rows, optionally naming the group from a visible title.
#[component]
pub fn DropdownMenuGroup(
    /// Noninteractive heading content for the group.
    #[props(default)]
    title: Option<Element>,
    /// Menu rows belonging to this group.
    #[props(default = rsx! {})]
    children: Element,
    /// Classes on the group root.
    #[props(default)]
    class: String,
    /// Inline declarations on the group root.
    #[props(default)]
    style: String,
    /// Native root attributes and callbacks. A visible title owns `aria-labelledby`.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let title = title.filter(is_renderable);
    let has_title = title.is_some();
    let heading_id = format!("astryx-dropdown-menu-heading-{}", current_scope_id().0);
    let labelled_by = has_title.then(|| heading_id.clone());
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(attr.name, "role" | "class" | "style")
                && !(has_title && attr.name == "aria-labelledby")
        })
        .collect();
    rsx! {
        div { role: "group", class: (!class.is_empty()).then_some(class), style: (!style.is_empty()).then_some(style),
            "aria-labelledby": labelled_by, ..attributes,
            if let Some(title) = title {
                div { id: heading_id, class: "astryx-dropdown-menu-section-heading", {title} }
            }
            {children}
        }
    }
}
