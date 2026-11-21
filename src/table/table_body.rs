use dioxus::prelude::*;

/// Native table body section. Place it inside a table and pass rows as children.
#[component]
pub fn TableBody(
    /// Body rows.
    children: Element,
    /// Additional classes on the body section.
    #[props(default)]
    class: String,
    /// Inline declarations on the body section.
    #[props(default)]
    style: Option<String>,
    /// Native attributes and events on the `tbody`.
    #[props(extends = tbody, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let class = if class.is_empty() {
        "astryx-table-body".to_owned()
    } else {
        format!("astryx-table-body {class}")
    };
    rsx! { tbody { class, style, ..attributes, {children} } }
}
