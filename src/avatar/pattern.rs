// Adapted from Avatar/Avatar.tsx. Private name-only/md slice used by
// StackFillItem, not an incomplete public Avatar API or image-loading port.
use dioxus::prelude::*;

use super::pattern_tooltip::use_pattern_tooltip;

#[component]
pub(crate) fn PatternAvatar(name: String, initials: String) -> Element {
    let id = use_hook(|| format!("stack-avatar-{}", name.replace(' ', "-")));
    let onmounted = use_pattern_tooltip(id.clone(), name.clone());
    // These closed demo names are source-owned; do not accept arbitrary CSS content.
    let initials_style = format!("--_avatar-initials:'{initials}'");
    rsx! {
        div { class: "astryx-avatar", "data-size": "md", "data-shape": "circle",
            "data-astryx-feedback-id": id.clone(), role: "img", "aria-label": name.clone(),
            tabindex: "0", onmounted,
            div { class: "astryx-avatar__content",
                div { class: "astryx-avatar-fallback", "data-size": "md", style: initials_style }
            }
        }
        div { id: "{id}-tooltip", class: "astryx-tooltip astryx-button-tooltip",
            role: "tooltip", popover: "manual", "{name}"
        }
    }
}
