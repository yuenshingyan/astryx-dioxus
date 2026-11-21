use dioxus::prelude::*;

/// Scrollable standalone command results. Provide options or groups of options
/// as children; the caller owns selection and keyboard navigation.
#[component]
pub fn CommandPaletteList(
    /// Results within the listbox.
    children: Element,
    /// Accessible name for the results.
    #[props(default = "Commands".to_owned())]
    label: String,
    /// Additional root classes.
    #[props(default)]
    class: String,
    /// Inline root styles.
    #[props(default)]
    style: Option<String>,
    /// Native root attributes, including the listbox ID and mount events.
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "role" | "aria-label"))
        .collect();
    let class = if class.is_empty() {
        "astryx-command-palette-list".to_owned()
    } else {
        format!("astryx-command-palette-list {class}")
    };
    rsx! { div { class, style, role: "listbox", "aria-label": label, ..attributes, {children} } }
}
