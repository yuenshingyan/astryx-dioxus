use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::popover::use_popover;

/// Desktop disclosure for a group of navigation links and optional featured content.
#[component]
pub fn TopNavMegaMenu(
    /// Visible trigger and accessible group name.
    label: String,
    #[props(default)] items: Option<Element>,
    #[props(default)] featured: Option<Element>,
    #[props(default)] on_open_change: Option<EventHandler<bool>>,
) -> Element {
    let id = use_hook(|| format!("astryx-top-nav-mega-menu-{}", current_scope_id().0));
    let (is_open, request, onmounted) = use_popover(id.clone(), None, on_open_change);
    let trigger_id = format!("{id}-trigger");
    rsx! {
        button { id: trigger_id.clone(), class: "astryx-top-nav-mega-menu__trigger", r#type: "button",
            "aria-expanded": is_open.to_string(), "aria-controls": id.clone(),
            onclick: move |_| request.call(!is_open),
            "{label}"
            span { "aria-hidden": "true", "⌄" }
        }
        div { id: id.clone(), class: "astryx-top-nav-mega-menu", popover: "manual", tabindex: -1,
            role: "group", "aria-labelledby": trigger_id, "data-open": is_open.to_string(),
            style: "position-anchor:--{id}", onmounted,
            if let Some(items) = items { div { class: "astryx-top-nav-mega-menu__items", {items} } }
            if let Some(featured) = featured { div { class: "astryx-top-nav-mega-menu__featured", {featured} } }
        }
    }
}
