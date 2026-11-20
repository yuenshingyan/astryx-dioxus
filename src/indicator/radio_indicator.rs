//! Decorative radio visual, adapted from Astryx Indicator/RadioIndicator.tsx.

use dioxus::prelude::*;

use super::check_indicator::{CheckIndicatorState, IndicatorSize};

/// Selection state shared with the check indicator.
pub type RadioIndicatorState = CheckIndicatorState;

/// Paints a decorative radio circle. The host owns input and focus semantics.
#[component]
pub fn RadioIndicator(
    /// Caller-owned selection state.
    state: RadioIndicatorState,
    /// Circle and dot size.
    #[props(default)]
    size: IndicatorSize,
    /// Draw the disabled appearance.
    #[props(default)]
    is_disabled: bool,
    /// Replace the inner mark, including with intentionally empty content.
    #[props(default)]
    replacement: Option<Element>,
    /// Host theme targets and caller classes.
    #[props(default)]
    class: String,
    /// Caller inline CSS.
    #[props(default)]
    style: String,
    /// Native root attributes and callbacks, including `onmounted`.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let is_checked = state == RadioIndicatorState::Checked;
    let size = match size {
        IndicatorSize::Small => "sm",
        IndicatorSize::Medium => "md",
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "class" | "style" | "aria-hidden" | "data-size" | "data-checked" | "data-disabled"
            )
        })
        .collect();
    rsx! {
        span {
            class: "astryx-radio-indicator {class}",
            style: (!style.is_empty()).then_some(style),
            "aria-hidden": "true",
            "data-size": size,
            "data-checked": is_checked.then_some("checked"),
            "data-disabled": is_disabled.then_some("disabled"),
            ..attributes,
            if let Some(replacement) = replacement {
                {replacement}
            } else if is_checked {
                span { class: "astryx-radio-indicator-dot", "data-size": size }
            }
        }
    }
}
