use dioxus::prelude::*;

use crate::{Button, ButtonVariant, Icon, use_app_shell_mobile};

/// Opens or closes the navigation surface owned by the mobile shell.
#[component]
pub fn MobileNavToggle(
    #[props(default = "Open navigation".to_owned())] label: String,
    #[props(default)] children: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] onmounted: EventHandler<MountedEvent>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let state = use_app_shell_mobile();
    if !state.is_mobile || !state.is_mobile_nav_enabled {
        return VNode::empty();
    }
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "aria-expanded" | "aria-controls"))
        .collect();
    attributes.push(Attribute::new(
        "aria-expanded",
        if state.is_mobile_nav_open {
            "true"
        } else {
            "false"
        },
        None,
        false,
    ));
    if let Some(id) = state.mobile_nav_id.filter(|id| !id.trim().is_empty()) {
        attributes.push(Attribute::new("aria-controls", id, None, false));
    }
    if !attributes.iter().any(|attr| attr.name == "data-testid") {
        attributes.push(Attribute::new(
            "data-testid",
            "mobile-nav-toggle",
            None,
            false,
        ));
    }
    let icon = children.unwrap_or_else(|| rsx! {
        Icon { icon: rsx! { svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", path { d: "M4 6h16M4 12h16M4 18h16" } } } }
    });
    let toggle = state.toggle_mobile_nav;
    rsx! {
        Button { variant: ButtonVariant::Ghost, is_icon_only: true, label, icon, class, style, attributes, onmounted,
            onclick: move |event: MouseEvent| {
                if let Some(toggle) = toggle { toggle.call(()); }
                if event.default_action_enabled() && let Some(handler) = onclick { handler.call(event); }
            },
        }
    }
}
