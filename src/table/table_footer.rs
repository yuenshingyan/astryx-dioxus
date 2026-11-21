use dioxus::prelude::*;

/// Native table footer section. Place it inside a table and pass rows as children.
#[component]
pub fn TableFooter(
    /// Footer rows, such as a summary or total.
    children: Element,
    /// Additional classes on the footer section.
    #[props(default)]
    class: String,
    /// Inline declarations on the footer section.
    #[props(default)]
    style: Option<String>,
    /// Native attributes and events on the `tfoot`.
    #[props(extends = tfoot, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let class = if class.is_empty() {
        "astryx-table-footer".to_owned()
    } else {
        format!("astryx-table-footer {class}")
    };
    rsx! { tfoot { class, style, ..attributes, {children} } }
}
