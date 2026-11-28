use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::use_tooltip::tooltip_element;
use crate::hooks::use_press_feedback::{PressFeedbackConfig, use_feedback_controller};

/// Inline text with a hover- and keyboard-accessible plain-text description.
#[component]
pub fn Tooltip(
    /// Visible inline anchor text.
    text: String,
    /// Supplemental help, not an accessible name or interactive content.
    content: String,
    /// Draw a dashed underline to signal supplemental help, without changing keyboard access.
    #[props(default = true)]
    has_hover_indication: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-tooltip-text-{}", current_scope_id().0));
    let has_description = !content.trim().is_empty() && content.trim() != text.trim();
    let mounted = use_feedback_controller(
        PressFeedbackConfig {
            id: id.clone(),
            label: text.clone(),
            tooltip: has_description.then(|| content.clone()),
            is_icon_only: false,
            is_loading: false,
            is_disabled: false,
        },
        false,
        has_description,
    );
    rsx! {
        span { class: "astryx-tooltip-text", "data-astryx-feedback-id": id.clone(),
            "data-hover-indication": (has_description && has_hover_indication).then_some("true"),
            tabindex: has_description.then_some("0"),
            "aria-describedby": has_description.then(|| format!("{id}-tooltip")),
            onmounted: mounted,
            "{text}"
        }
        {tooltip_element(&id, has_description.then_some(content.as_str()))}
    }
}
