use dioxus::prelude::*;

/// Displays empty results without adding roles, focus stops, or announcements.
/// Keep actionable content outside listbox and option semantics.
#[component]
pub fn CommandPaletteEmpty(
    /// Message or caller-owned empty-state content.
    children: Element,
    /// Additional root classes.
    #[props(default)]
    class: String,
    /// Inline declarations for the root.
    #[props(default)]
    style: Option<String>,
    /// Native root attributes and events.
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let class = if class.is_empty() {
        "astryx-command-palette-empty".to_owned()
    } else {
        format!("astryx-command-palette-empty {class}")
    };
    rsx! { div { class, style, ..attributes, {children} } }
}
