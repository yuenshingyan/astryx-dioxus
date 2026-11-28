//! Decorative multi-selection visual, adapted from Astryx Indicator/CheckboxIndicator.tsx.

use dioxus::prelude::*;

use super::check_indicator::IndicatorSize;

/// Selection state supplied by the owning control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxIndicatorState {
    /// Draw an empty box.
    Unchecked,
    /// Draw a check mark.
    Checked,
    /// Draw a dash for mixed selection.
    Indeterminate,
}

/// Paints a checkbox without adding input, focus, or activation semantics.
#[component]
pub fn CheckboxIndicator(
    /// Caller-owned selection state.
    state: CheckboxIndicatorState,
    /// Box and mark size.
    #[props(default)]
    size: IndicatorSize,
    /// Use the disabled appearance.
    #[props(default)]
    is_disabled: bool,
    /// Replace both built-in marks without removing the box. Pass `None` to keep the
    /// state mark (for example, use `is_loading.then(|| rsx! { ... })`). `Some` replaces
    /// it even when the element renders nothing. Supply decorative, non-focusable content.
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
    let checked = match state {
        CheckboxIndicatorState::Unchecked => None,
        CheckboxIndicatorState::Checked => Some("checked"),
        CheckboxIndicatorState::Indeterminate => Some("indeterminate"),
    };
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
            class: "astryx-checkbox-indicator {class}",
            style: (!style.is_empty()).then_some(style),
            "aria-hidden": "true",
            "data-size": size,
            "data-checked": checked,
            "data-disabled": is_disabled.then_some("disabled"),
            ..attributes,
            if let Some(replacement) = replacement {
                {replacement}
            } else {
                svg { class: "astryx-checkbox-indicator-check", "data-size": size, view_box: "0 0 10 10",
                    path { d: "M8.5 2.5L4 7.5L1.5 5", stroke: "currentColor", stroke_width: "1.5", fill: "none", stroke_linecap: "round", stroke_linejoin: "round" }
                }
                span { class: "astryx-checkbox-indicator-dash", "data-size": size }
            }
        }
    }
}
