use dioxus::core::AttributeValue;
use dioxus::prelude::*;

use super::table_cell::TableCellScope;

/// Native header cell. The standalone cell adds no styling or ARIA role.
#[component]
pub fn TableHeaderCell(
    /// Header content.
    #[props(default)]
    children: Element,
    /// Which rows or columns this header describes.
    #[props(default)]
    scope: Option<TableCellScope>,
    /// Additional classes on the header cell.
    #[props(default)]
    class: String,
    /// Inline declarations on the header cell.
    #[props(default)]
    style: Option<String>,
    /// Native attributes and events on the `th`.
    #[props(extends = th, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let mut class = if class.is_empty() {
        "astryx-table-header-cell".to_owned()
    } else {
        format!("astryx-table-header-cell {class}")
    };
    let mut style = style;
    // Emit each native styling attribute once, keeping the target and letting
    // later caller declarations win just as they would in the spread.
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter_map(|attr| match (attr.namespace, attr.name) {
            (None, "scope") => None,
            (None, "class") => {
                if let AttributeValue::Text(value) = attr.value
                    && !value.trim().is_empty()
                {
                    class.push(' ');
                    class.push_str(&value);
                }
                None
            }
            (None, "style") => {
                if let AttributeValue::Text(value) = attr.value
                    && !value.trim().is_empty()
                {
                    if let Some(style) = &mut style {
                        if !style.trim_end().ends_with(';') {
                            style.push(';');
                        }
                        style.push_str(&value);
                    } else {
                        style = Some(value);
                    }
                }
                None
            }
            _ => Some(attr),
        })
        .collect();
    rsx! { th { class, style, scope: scope.map(TableCellScope::as_str), ..attributes, {children} } }
}
