use dioxus::prelude::*;

use crate::Kbd;

/// Presents keyboard hints or caller-provided content without handling shortcuts.
#[component]
pub fn CommandPaletteFooter(
    /// Replaces the default hints when supplied.
    #[props(default)]
    children: Option<Element>,
    /// Localized navigation hint.
    #[props(default = "Navigate".to_owned())]
    navigate_label: String,
    /// Localized selection hint.
    #[props(default = "Select".to_owned())]
    select_label: String,
    /// Localized dismissal hint.
    #[props(default = "Close".to_owned())]
    close_label: String,
    /// Additional root classes.
    #[props(default)]
    class: String,
    /// Inline root styles.
    #[props(default)]
    style: Option<String>,
    /// Native root attributes, including mount events.
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let class = if class.is_empty() {
        "astryx-command-palette-footer".to_owned()
    } else {
        format!("astryx-command-palette-footer {class}")
    };
    let content = children.unwrap_or_else(|| rsx! {
        span { class: "astryx-command-palette-footer__hint", Kbd { keys: "up" } Kbd { keys: "down" } "{navigate_label}" }
        span { class: "astryx-command-palette-footer__hint", Kbd { keys: "enter" } "{select_label}" }
        span { class: "astryx-command-palette-footer__hint", Kbd { keys: "escape" } "{close_label}" }
    });
    rsx! { div { class, style, ..attributes, {content} } }
}
