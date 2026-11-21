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
    let class = if class.is_empty() {
        "astryx-table-header-cell".to_owned()
    } else {
        format!("astryx-table-header-cell {class}")
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| attr.name != "scope")
        .collect();
    rsx! { th { class, style, scope: scope.map(TableCellScope::as_str), ..attributes, {children} } }
}
