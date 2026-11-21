use dioxus::prelude::*;

/// Native table row. Place it inside a table header, body, or footer so the
/// browser does not insert an implicit section when parsing server output.
#[component]
pub fn TableRow(
    /// Header or data cells belonging to the row.
    children: Element,
    /// Additional classes on the row.
    #[props(default)]
    class: String,
    /// Inline declarations on the row.
    #[props(default)]
    style: Option<String>,
    /// Native attributes and events on the `tr`.
    #[props(extends = tr, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let class = if class.is_empty() {
        "astryx-table-row".to_owned()
    } else {
        format!("astryx-table-row {class}")
    };
    rsx! { tr { class, style, ..attributes, {children} } }
}
