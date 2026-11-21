use dioxus::prelude::*;

/// Groups related command results beneath a visible heading that names the group.
#[component]
pub fn CommandPaletteGroup(
    /// Default accessible name and visible heading.
    heading: String,
    /// Caller-owned result items.
    children: Element,
    /// Additional root classes.
    #[props(default)]
    class: String,
    /// Inline root styles.
    #[props(default)]
    style: Option<String>,
    /// Native root attributes, including overrides for the default role and name.
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let role = (!attributes.iter().any(|attr| attr.name == "role")).then_some("group");
    let label =
        (!attributes.iter().any(|attr| attr.name == "aria-label")).then_some(heading.as_str());
    let class = if class.is_empty() {
        "astryx-command-palette-group".to_owned()
    } else {
        format!("astryx-command-palette-group {class}")
    };
    rsx! {
        div { class, style, role, "aria-label": label, ..attributes,
            div { class: "astryx-command-palette-group-heading", "aria-hidden": "true", "{heading}" }
            {children}
        }
    }
}
