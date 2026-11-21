use dioxus::prelude::*;

use crate::{Button, ButtonSize, ButtonVariant, Icon, IconSize};

/// Caller-owned collapse state shared by an external toggle and its rail.
#[derive(Clone, Copy, PartialEq)]
pub struct SideNavControlledCollapsible {
    /// Whether the rail is currently collapsed.
    pub is_collapsed: bool,
    /// Requests the next state without updating the rail directly.
    pub on_collapsed_change: EventHandler<bool>,
}

/// Icon-only ghost button for an externally controlled navigation rail.
/// Without controlled state no button is rendered. Rail context and AppShell
/// integration are not yet available.
#[component]
pub fn SideNavCollapseButton(
    #[props(default)] collapsible: Option<SideNavControlledCollapsible>,
    #[props(default)] label: Option<String>,
    #[props(default)] size: Option<ButtonSize>,
    #[props(default)] children: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] onmounted: EventHandler<MountedEvent>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let Some(state) = collapsible else {
        return VNode::empty();
    };
    let SideNavControlledCollapsible {
        is_collapsed,
        on_collapsed_change,
    } = state;
    let label = label.unwrap_or_else(|| {
        if is_collapsed {
            "Expand sidebar"
        } else {
            "Collapse sidebar"
        }
        .to_string()
    });
    let icon = children.unwrap_or_else(|| rsx! {
        span { class: "astryx-side-nav-collapse-button__mirror", "aria-hidden": "true",
            Icon { class: "astryx-side-nav-collapse-button__chevron", size: IconSize::Small,
                icon: rsx! {
                    svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
                        path { d: "M15 19l-7-7 7-7" }
                    }
                },
            }
        }
    });
    let mut attributes = attributes;
    attributes.retain(|attribute| attribute.name != "data-collapsed");
    if is_collapsed {
        attributes.push(Attribute::new("data-collapsed", "true", None, false));
    }
    rsx! {
        Button { label, size, class: "astryx-side-nav-collapse-button {class}", style,
            variant: ButtonVariant::Ghost, is_icon_only: true, icon, onmounted,
            onclick: move |event: MouseEvent| {
                if let Some(handler) = onclick { handler.call(event.clone()); }
                if event.default_action_enabled() { on_collapsed_change.call(!is_collapsed); }
            },
            attributes,
        }
    }
}
