//! CSS Grid layout, adapted from Astryx Grid. See THIRD_PARTY_NOTICES.md.

use std::fmt::Write;

use dioxus::prelude::*;

use crate::layout::padding::spacing_step_css;
use crate::utils::types::{SizeValue, SpacingStep};

/// Item alignment along either logical grid axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridAlignment {
    /// Align to the start edge.
    Start,
    /// Center the item.
    Center,
    /// Align to the end edge.
    End,
    /// Stretch the item to fill its grid area.
    Stretch,
}

impl GridAlignment {
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Stretch => "stretch",
        }
    }
}

/// How responsive tracks treat unoccupied columns.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GridRepeat {
    /// Reserve space for empty tracks.
    #[default]
    Fill,
    /// Collapse empty tracks.
    Fit,
}

/// Fixed or width-dependent column configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridColumns {
    /// Exactly this many `1fr` tracks. Nonpositive counts use the default single track.
    Fixed(i32),
    /// Columns that reflow according to available width.
    Responsive {
        /// Minimum track width in pixels; must be finite and nonnegative.
        min_width: f64,
        /// Optional positive column cap; nonpositive values are ignored.
        max: Option<i32>,
        /// Whether empty tracks occupy space.
        repeat: GridRepeat,
    },
}

impl From<i32> for GridColumns {
    fn from(value: i32) -> Self {
        Self::Fixed(value)
    }
}

fn track_template(columns: Option<GridColumns>, gap: Option<SpacingStep>) -> Option<String> {
    match columns {
        Some(GridColumns::Fixed(count)) if count > 0 => Some(format!("repeat({count}, 1fr)")),
        Some(GridColumns::Responsive {
            min_width,
            max,
            repeat,
        }) => {
            if !min_width.is_finite() || min_width < 0.0 {
                return None;
            }
            let mode = match repeat {
                GridRepeat::Fill => "auto-fill",
                GridRepeat::Fit => "auto-fit",
            };
            let floor = if let Some(max) = max.filter(|value| *value > 0) {
                let share = match gap {
                    Some(gap) => format!(
                        "calc((100% - {} * {}) / {max})",
                        max - 1,
                        spacing_step_css(gap)
                    ),
                    None => format!("calc(100% / {max})"),
                };
                format!("min(100%, max({min_width}px, {share}))")
            } else {
                format!("{min_width}px")
            };
            Some(format!("repeat({mode}, minmax({floor}, 1fr))"))
        }
        _ => Some("1fr".into()),
    }
}

/// Native CSS Grid container. Children retain their DOM order and semantics.
#[component]
pub fn Grid(
    /// Fixed count or responsive column tracks.
    #[props(default, into)]
    columns: Option<GridColumns>,
    /// Implicit row height in pixels. Negative and non-finite values are omitted.
    #[props(default)]
    row_height: Option<f64>,
    /// Explicit width, overriding caller inline width.
    #[props(default, into)]
    width: Option<SizeValue>,
    /// Explicit height, overriding caller inline height.
    #[props(default, into)]
    height: Option<SizeValue>,
    /// Explicit maximum width.
    #[props(default, into)]
    max_width: Option<SizeValue>,
    /// Explicit minimum height.
    #[props(default, into)]
    min_height: Option<SizeValue>,
    /// Shared spacing for both axes.
    #[props(default)]
    gap: Option<SpacingStep>,
    /// Overrides the shared gap for rows.
    #[props(default)]
    row_gap: Option<SpacingStep>,
    /// Overrides the shared gap for columns and capped-track arithmetic.
    #[props(default)]
    column_gap: Option<SpacingStep>,
    /// Block-axis item alignment.
    #[props(default)]
    align: Option<GridAlignment>,
    /// Inline-axis item alignment.
    #[props(default)]
    justify: Option<GridAlignment>,
    /// Caller CSS classes.
    #[props(default)]
    class: String,
    /// Caller inline CSS.
    #[props(default)]
    style: String,
    /// Unmodified grid items.
    #[props(default = rsx! {})]
    children: Element,
    /// Native root attributes and event handlers, including `onmounted`.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let mut css = String::new();
    if let Some(template) = track_template(columns, column_gap.or(gap)) {
        let _ = write!(css, "--_grid-columns:{template};");
    }
    if let Some(height) = row_height.filter(|value| value.is_finite() && *value >= 0.0) {
        let _ = write!(css, "--_grid-rows:{height}px;");
    }
    for (name, step) in [
        ("row-gap", row_gap.or(gap)),
        ("column-gap", column_gap.or(gap)),
    ] {
        if let Some(step) = step {
            let _ = write!(css, "--_grid-{name}:{};", spacing_step_css(step));
        }
    }
    css.push_str(&style);
    css.push(';');
    for (name, value) in [
        ("width", width),
        ("height", height),
        ("max-width", max_width),
        ("min-height", min_height),
    ] {
        if let Some(value) = value {
            let _ = write!(css, "{name}:{};", value.as_css()?);
        }
    }
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "class" | "style" | "data-columns" | "data-gap" | "data-align" | "data-justify"
            )
        })
        .collect();
    rsx! {
        div {
            class: "astryx-grid {class}", style: css,
            "data-columns": match columns {
                Some(GridColumns::Fixed(count)) => Some(count.to_string()),
                _ => None,
            },
            "data-gap": gap.map(SpacingStep::as_str),
            "data-align": align.map(GridAlignment::as_str),
            "data-justify": justify.map(GridAlignment::as_str),
            ..attributes,
            {children}
        }
    }
}
