use dioxus::core::AttributeValue;
use dioxus::prelude::*;

use crate::{ScrollAxis, ScrollOverscroll, ScrollableArea, ScrollableAreaRole};

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
    /// Accessible name shared by the table and its scroll region.
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
    rsx! {
        ScrollableArea { label: label.clone().unwrap_or_else(|| "Table".into()), "aria-labelledby": labelled_by.clone(),
            axis: ScrollAxis::Inline, overscroll: ScrollOverscroll::Contain, role: ScrollableAreaRole::Group, class: "astryx-table-scroll-wrapper",
            table { class: "astryx-table {class}", style, "aria-label": label, "aria-labelledby": labelled_by,
                "data-density": density, "data-dividers": dividers, "data-vertical-align": align, "data-text-overflow": overflow,
                "data-striped": is_striped.then_some("true"), "data-hover": has_hover.then_some("true"), ..attributes, {children}
            }
        }
    }
}
