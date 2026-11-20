//! Grid participation modifier, adapted from Astryx Grid/GridSpan.tsx.

use std::fmt::Write;

use dioxus::prelude::*;

/// Column placement for a [`GridSpan`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridSpanColumns {
    /// Span this many columns. Zero leaves caller placement unchanged.
    Count(u32),
    /// Span all explicit columns, from line 1 to line -1.
    Full,
}

/// A nonsemantic grid item wrapper that can span multiple tracks.
#[component]
pub fn GridSpan(
    /// Column span. Unset leaves placement to the browser or caller CSS.
    #[props(default)]
    columns: Option<GridSpanColumns>,
    /// Row span. Zero leaves caller placement unchanged.
    #[props(default)]
    rows: Option<u32>,
    /// Caller CSS classes, replacing StyleX styling inputs.
    #[props(default)]
    class: String,
    /// Caller inline CSS. Explicit spans override matching placement properties.
    #[props(default)]
    style: String,
    /// Unmodified caller content.
    #[props(default = rsx! {})]
    children: Element,
    /// Native root attributes and events, including `onmounted`.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let mut css = style;
    if !css.is_empty() && !css.ends_with(';') {
        css.push(';');
    }
    match columns {
        Some(GridSpanColumns::Count(count)) if count > 0 => {
            let _ = write!(css, "grid-column:span {count};");
        }
        Some(GridSpanColumns::Full) => css.push_str("grid-column:1 / -1;"),
        _ => {}
    }
    if let Some(rows) = rows.filter(|rows| *rows > 0) {
        let _ = write!(css, "grid-row:span {rows};");
    }
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| !matches!(attribute.name, "class" | "style"))
        .collect();
    rsx! {
        div {
            class: "astryx-grid-span {class}",
            style: (!css.is_empty()).then_some(css),
            ..attributes,
            {children}
        }
    }
}
