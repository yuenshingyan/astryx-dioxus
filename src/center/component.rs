//! Native Center layout adapted from Astryx Center/Center.tsx.

use std::fmt::Write;

use dioxus::prelude::*;

use crate::layout::padding::spacing_step_css;
use crate::utils::types::{SizeValue, SpacingStep};

/// Centering mode. The single-axis modes follow the released flex axes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CenterAxis {
    /// Center on both flex axes.
    #[default]
    Both,
    /// Center on the flex main axis (horizontal in horizontal writing).
    Horizontal,
    /// Center on the flex cross axis (vertical in horizontal writing).
    Vertical,
}

impl CenterAxis {
    fn as_str(self) -> &'static str {
        match self {
            Self::Both => "both",
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// A nonsemantic flex container that centers its children without wrapping them.
#[component]
pub fn Center(
    /// Axis or axes on which to center.
    #[props(default)]
    axis: CenterAxis,
    /// Use inline-flex instead of flex.
    #[props(default)]
    is_inline: bool,
    /// Width in pixels or as a CSS sizing value.
    #[props(default, into)]
    width: Option<SizeValue>,
    /// Height in pixels or as a CSS sizing value.
    #[props(default, into)]
    height: Option<SizeValue>,
    /// Maximum width.
    #[props(default, into)]
    max_width: Option<SizeValue>,
    /// Minimum height.
    #[props(default, into)]
    min_height: Option<SizeValue>,
    /// Default padding for all logical edges.
    #[props(default)]
    padding: Option<SpacingStep>,
    /// Padding for the inline axis.
    #[props(default)]
    padding_inline: Option<SpacingStep>,
    /// Padding at inline start.
    #[props(default)]
    padding_inline_start: Option<SpacingStep>,
    /// Padding at inline end.
    #[props(default)]
    padding_inline_end: Option<SpacingStep>,
    /// Padding for the block axis.
    #[props(default)]
    padding_block: Option<SpacingStep>,
    /// Padding at block start.
    #[props(default)]
    padding_block_start: Option<SpacingStep>,
    /// Padding at block end.
    #[props(default)]
    padding_block_end: Option<SpacingStep>,
    /// Caller CSS classes.
    #[props(default)]
    class: String,
    /// Caller inline CSS, which can override component styles.
    #[props(default)]
    style: String,
    /// Native caller content in DOM order.
    children: Element,
    /// Native root attributes and callbacks, including `onmounted`.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let mut css = String::new();
    for (property, value) in [
        ("width", width),
        ("height", height),
        ("max-width", max_width),
        ("min-height", min_height),
    ] {
        if let Some(value) = value {
            let _ = write!(css, "--_center-{property}:{};", value.as_css()?);
        }
    }
    for (edge, step) in [
        (
            "inline-start",
            padding_inline_start.or(padding_inline).or(padding),
        ),
        (
            "inline-end",
            padding_inline_end.or(padding_inline).or(padding),
        ),
        (
            "block-start",
            padding_block_start.or(padding_block).or(padding),
        ),
        ("block-end", padding_block_end.or(padding_block).or(padding)),
    ] {
        if let Some(step) = step {
            let _ = write!(css, "--_center-padding-{edge}:{};", spacing_step_css(step));
        }
    }
    css.push_str(&style);
    let modifier = if is_inline {
        " astryx-center--inline"
    } else {
        ""
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| !matches!(attribute.name, "class" | "style" | "data-axis"))
        .collect();
    rsx! {
        div {
            class: "astryx-center{modifier} {class}",
            "data-axis": axis.as_str(),
            style: (!css.is_empty()).then_some(css),
            ..attributes,
            {children}
        }
    }
}
