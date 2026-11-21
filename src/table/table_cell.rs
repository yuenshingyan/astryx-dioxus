use dioxus::prelude::*;

/// Native table cell scope values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableCellScope {
    Column,
    Row,
    ColumnGroup,
    RowGroup,
}

impl TableCellScope {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Column => "col",
            Self::Row => "row",
            Self::ColumnGroup => "colgroup",
            Self::RowGroup => "rowgroup",
        }
    }
}

/// Native data cell. The standalone cell adds no styling or ARIA semantics.
/// Use native attributes such as `headers`, `colspan`, and `rowspan` to
/// describe its relationships to other cells.
#[component]
pub fn TableCell(
    /// Cell content.
    #[props(default)]
    children: Element,
    /// Space-separated IDs of associated headers.
    #[props(default)]
    headers: Option<String>,
    /// Native scope attribute; this does not turn the data cell into a header.
    #[props(default)]
    scope: Option<TableCellScope>,
    /// Additional classes on the cell.
    #[props(default)]
    class: String,
    /// Inline declarations on the cell.
    #[props(default)]
    style: Option<String>,
    /// Native attributes and events on the `td`.
    #[props(extends = td, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let class = if class.is_empty() {
        "astryx-table-cell".to_owned()
    } else {
        format!("astryx-table-cell {class}")
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "headers" | "scope"))
        .collect();
    rsx! { td { class, style, "headers": headers, "scope": scope.map(TableCellScope::as_str), ..attributes, {children} } }
}
