use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::tab_list_context::TabListContext;
use crate::popover::use_popover;
use crate::{CheckIndicator, CheckIndicatorState, NavHeadingMenu, use_size};

/// A named choice in a navigation tab menu.
#[derive(Clone, PartialEq)]
pub struct TabMenuOption {
    /// Unique value shared with the surrounding TabList.
    pub value: String,
    /// Visible and accessible option name.
    pub label: String,
    /// Optional decorative artwork.
    pub icon: Option<Element>,
}

/// A compact picker for choices in a navigation TabList.
#[component]
pub fn TabMenu(label: String, options: Vec<TabMenuOption>) -> Element {
    let id = use_hook(|| format!("astryx-tab-menu-{}", current_scope_id().0));
    let (is_open, request, onmounted) = use_popover(id.clone(), None, None);
    let size = use_size(None);
    let Some(ctx) = try_consume_context::<TabListContext>() else {
        return VNode::empty();
    };
    let state = (ctx.0)();
    if state.is_panel_list {
        return VNode::empty();
    }
    let selected = options.iter().find(|option| option.value == state.value);
    let is_selected = selected.is_some();
    let trigger_text = selected
        .map_or(label.as_str(), |option| option.label.as_str())
        .to_owned();
    let position = format!("position-anchor:--{id}");
    rsx! {
        button { id: "{id}-trigger", class: "astryx-tab astryx-tab-menu__trigger", r#type: "button",
            "data-tab-value": id.clone(), "data-size": size.as_str(), tabindex: if is_selected { 0 } else { -1 },
            "aria-current": is_selected.then_some("true"), "aria-expanded": is_open.to_string(), "aria-haspopup": "menu",
            "aria-controls": id.clone(), onclick: move |_| request.call(!is_open),
            "{trigger_text}" span { "aria-hidden": "true", "⌄" }
            span { class: "astryx-tab-indicator", "aria-hidden": "true" }
        }
        NavHeadingMenu { id: id.clone(), class: "astryx-tab-menu", style: position, popover: "manual",
            "aria-label": label.clone(), tabindex: -1, "data-open": is_open.to_string(), "data-popover-menu-focus": "true", onmounted,
            span { class: "astryx-tab-menu__heading", role: "presentation", "{label}" }
            for option in options {
                {
                    let checked = state.value == option.value;
                    let change = state.on_change;
                    rsx! {
                        button { key: "{option.value}", class: "astryx-nav-heading-menu-item astryx-tab-menu__option",
                            r#type: "button", role: "menuitemradio", tabindex: -1, "aria-checked": checked.to_string(),
                            // The shared typeahead reads aria-label before textContent, which can include decorative artwork.
                            "aria-label": option.label.clone(),
                            onclick: move |_| {
                                if !checked { change.call(option.value.clone()); }
                                request.call(false);
                            },
                            span { class: "astryx-tab-menu__option-content",
                                if let Some(icon) = option.icon { span { "aria-hidden": "true", {icon} } }
                                "{option.label}"
                            }
                            if checked {
                                CheckIndicator { state: CheckIndicatorState::Checked, class: "astryx-tab-menu__check" }
                            }
                        }
                    }
                }
            }
        }
    }
}
