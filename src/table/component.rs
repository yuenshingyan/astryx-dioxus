use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use crate::hooks::use_scrollable_area::use_scrollable_area;

/// Padding density for table cells.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TableDensity {
    Compact,
    #[default]
    Balanced,
    Spacious,
}

/// Dividers drawn between table cells.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TableDividers {
    #[default]
    Rows,
    Columns,
    Grid,
    None,
}

/// Vertical alignment for data cells.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TableVerticalAlign {
    Top,
    #[default]
    Middle,
    Bottom,
}

/// Text overflow policy for data cells.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TableTextOverflow {
    #[default]
    Wrap,
    Truncate,
}

/// A styled native table for caller-supplied sections and rows.
#[component]
pub fn Table(
    /// Caption and table sections in native HTML order.
    children: Element,
    /// Accessible label for the table and, when `labelled_by` is absent, its scroll region.
    #[props(default)]
    label: Option<String>,
    /// IDs of elements that name the table and its scroll region.
    #[props(default)]
    labelled_by: Option<String>,
    #[props(default)] density: TableDensity,
    #[props(default)] dividers: TableDividers,
    #[props(default)] vertical_align: TableVerticalAlign,
    #[props(default)] text_overflow: TableTextOverflow,
    #[props(default)] is_striped: bool,
    #[props(default)] has_hover: bool,
    #[props(default)] class: String,
    #[props(default)] style: Option<String>,
    /// Attributes on the native `table`, not on the scroll region.
    #[props(extends = table, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let mut label = label;
    let mut labelled_by = labelled_by;
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| match attr.name {
            "aria-label" | "aria-labelledby" => {
                if let AttributeValue::Text(value) = &attr.value {
                    if attr.name == "aria-label" {
                        label.get_or_insert_with(|| value.clone());
                    } else {
                        labelled_by.get_or_insert_with(|| value.clone());
                    }
                }
                false
            }
            "data-density"
            | "data-dividers"
            | "data-vertical-align"
            | "data-text-overflow"
            | "data-striped"
            | "data-hover" => false,
            _ => true,
        })
        .collect();
    let density = match density {
        TableDensity::Compact => "compact",
        TableDensity::Balanced => "balanced",
        TableDensity::Spacious => "spacious",
    };
    let dividers = match dividers {
        TableDividers::Rows => "rows",
        TableDividers::Columns => "columns",
        TableDividers::Grid => "grid",
        TableDividers::None => "none",
    };
    let align = match vertical_align {
        TableVerticalAlign::Top => "top",
        TableVerticalAlign::Middle => "middle",
        TableVerticalAlign::Bottom => "bottom",
    };
    let overflow = match text_overflow {
        TableTextOverflow::Wrap => "wrap",
        TableTextOverflow::Truncate => "truncate",
    };
    // Table owns the viewport name; the shared hook still manages scrolling and focus.
    let viewport_label = labelled_by
        .is_none()
        .then(|| label.as_deref().unwrap_or("Table"));
    let viewport_id = format!("scrollable-area-{}", current_scope_id().0);
    let mounted = use_scrollable_area(viewport_id.clone(), ("inline", false, true));
    rsx! {
        div {
            class: "astryx-scrollable-area astryx-table-scroll-wrapper",
            role: "group", "aria-label": viewport_label, "aria-labelledby": labelled_by.clone(),
            "data-axis": "inline", "data-scroll-axis": "inline", "data-astryx-scroll-viewport": viewport_id,
            onmounted: mounted,
            div {
                class: "astryx-scrollable-area__content", "data-scroll-content": "",
                style: "--container-padding-inline-start:var(--spacing-0);\
                        --container-padding-inline-end:var(--spacing-0);\
                        --container-padding-block-start:var(--spacing-0);\
                        --container-padding-block-end:var(--spacing-0);",
                table { class: "astryx-table {class}", style, "aria-label": label, "aria-labelledby": labelled_by,
                    "data-density": density, "data-dividers": dividers, "data-vertical-align": align, "data-text-overflow": overflow,
                    "data-striped": is_striped.then_some("true"), "data-hover": has_hover.then_some("true"), ..attributes, {children}
                }
            }
        }
    }
}
