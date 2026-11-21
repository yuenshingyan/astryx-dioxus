use dioxus::prelude::*;

use super::overlay_scrim::{
    OverlayAlign, OverlayPosition, OverlayScrim, OverlayScrimMode, OverlayShowOn,
};

/// Places a scrim and content over media. Automatic reveal supports desktop
/// hover and focus; touch callers can control visibility with `is_open`.
#[component]
pub fn Overlay(
    /// Content above the base media.
    content: Element,
    /// Base media or other content.
    #[props(default)]
    children: Element,
    /// Automatic reveal policy.
    #[props(default)]
    show_on: OverlayShowOn,
    /// Overrides automatic reveal when supplied.
    #[props(default)]
    is_open: Option<bool>,
    /// Backdrop and media theme.
    #[props(default)]
    scrim: OverlayScrimMode,
    /// Scrim placement.
    #[props(default)]
    position: OverlayPosition,
    /// Content alignment.
    #[props(default)]
    align: OverlayAlign,
    /// Additional classes on the container.
    #[props(default)]
    class: String,
    /// Inline container styles, including any clipping radius.
    #[props(default)]
    style: Option<String>,
    /// Native container attributes and events.
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let class = if class.is_empty() {
        "astryx-overlay".to_owned()
    } else {
        format!("astryx-overlay {class}")
    };
    rsx! {
        div { class, style, ..attributes, {children}
            OverlayScrim { scrim, position, align, show_on, is_open, {content} }
        }
    }
}
