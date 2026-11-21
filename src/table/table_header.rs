use dioxus::prelude::*;

/// Native table header section. Place it inside a table and pass rows as children.
#[component]
pub fn TableHeader(
    /// Header rows, normally containing column header cells.
    children: Element,
    /// Additional classes on the header section.
    #[props(default)]
    class: String,
    /// Inline declarations on the header section.
    #[props(default)]
    style: Option<String>,
    /// Native attributes and events on the `thead`.
    #[props(extends = thead, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let class = if class.is_empty() {
        "astryx-table-header".to_owned()
    } else {
        format!("astryx-table-header {class}")
    };
    rsx! { thead { class, style, ..attributes, {children} } }
}
