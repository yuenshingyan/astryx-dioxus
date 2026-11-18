// Adapted from Astryx Tooltip/useTooltip. See THIRD_PARTY_NOTICES.md.

use dioxus::prelude::*;

use crate::hooks::use_press_feedback::{PressFeedbackConfig, use_feedback_controller};

pub(crate) const TOOLTIP_CONTROLLER: &str = include_str!("use_tooltip.js");

/// Existing Button and Link may render this sibling for an explicit String
/// tooltip. Compose `{id}-tooltip` into aria-describedby unless it repeats the
/// accessible name; the DOM bridge preserves caller descriptions and reconciles
/// actual rendered names on mount. This is not the public Tooltip component.
pub(crate) fn tooltip_element(id: &str, tooltip: Option<&str>) -> Element {
    rsx! {
        if let Some(tooltip) = tooltip {
            div {
                id: format!("{id}-tooltip"),
                class: "astryx-tooltip astryx-button-tooltip",
                role: "tooltip",
                popover: "manual",
                "{tooltip}"
            }
        }
    }
}

/// Text integration: call unconditionally, with `is_enabled = max_lines > 0`,
/// and put `data-astryx-truncation-id=id` on the Text root.
/// Compose the returned handler into onmounted. No tab stop is added here;
/// Text owns its keyboard-interest/tabindex policy. Hover, tap and focus within
/// a focusable parent reveal actual width/height overflow on interest only.
/// Automatic tips never add aria-describedby (their text is already the name).
pub(crate) fn use_truncation_tooltip(
    id: String,
    text: String,
    is_enabled: bool,
) -> EventHandler<MountedEvent> {
    use_feedback_controller(
        PressFeedbackConfig {
            id,
            label: text,
            tooltip: None,
            is_icon_only: false,
            is_loading: false,
            is_disabled: false,
        },
        true,
        None,
        is_enabled,
    )
}
