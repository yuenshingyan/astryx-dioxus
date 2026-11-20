//! Decorative single-selection mark, adapted from Astryx Indicator/CheckIndicator.tsx.

use dioxus::prelude::*;

use crate::icon::global_icon_registry::resolve_icon;
use crate::icon::{IconRequest, IconSize, render_icon};

/// Whether a single-selection item is marked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckIndicatorState {
    /// Do not reserve space for a mark.
    Unchecked,
    /// Draw the decorative check.
    Checked,
}

/// Supported indicator sizes. Both use the small Icon box for a check.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IndicatorSize {
    /// Small control.
    Small,
    /// Medium control.
    #[default]
    Medium,
}

/// Paint a decorative check or a caller-provided replacement without adding control semantics.
#[component]
pub fn CheckIndicator(
    /// Selection state owned by the caller.
    state: CheckIndicatorState,
    /// Size of the composing control.
    #[props(default)]
    size: IndicatorSize,
    /// Use the disabled icon foreground.
    #[props(default)]
    is_disabled: bool,
    /// Replacement content, including a deliberately empty element. `None` retains the mark.
    #[props(default)]
    replacement: Option<Element>,
    /// Host styling target and caller classes.
    #[props(default)]
    class: String,
    /// Caller inline CSS.
    #[props(default)]
    style: String,
    /// Native attributes and callbacks for whichever root renders.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| !matches!(attribute.name, "class" | "style" | "aria-hidden"))
        .collect();
    let color = if is_disabled {
        "var(--color-icon-disabled)"
    } else {
        "var(--color-accent)"
    };
    if let Some(replacement) = replacement {
        return rsx! { span {
            class,
            style: "display:inline-flex;align-items:center;justify-content:center;flex-shrink:0;width:1rem;height:1rem;color:{color};{style}",
            "aria-hidden": "true",
            ..attributes,
            {replacement}
        } };
    }
    if state == CheckIndicatorState::Unchecked {
        return rsx! {};
    }
    let icon_size = match size {
        IndicatorSize::Small | IndicatorSize::Medium => IconSize::Small,
    };
    let icon = resolve_icon(
        "check",
        IconRequest {
            size: icon_size,
            variant: None,
        },
        rsx! {
            svg { view_box: "0 0 24 24", width: "1em", height: "1em", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                path { d: "M20 6 9 17l-5-5" }
            }
        },
    );
    render_icon(
        icon,
        icon_size,
        Some(color.to_owned()),
        None,
        class,
        style,
        attributes,
    )
}
