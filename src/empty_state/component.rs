//! Adapted from Astryx EmptyState/EmptyState.tsx.

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::heading::HeadingLevel;
use crate::hooks::use_announce_rendered_text::use_announce_rendered_text;
use crate::utils::is_renderable::is_renderable;

/// An empty-content message with optional decorative artwork and actions.
#[component]
pub fn EmptyState(
    /// Primary message, rendered as a native heading.
    title: String,
    /// Optional supporting copy.
    #[props(default)]
    description: Option<String>,
    /// Decorative illustration; do not include focusable descendants.
    #[props(default)]
    icon: Option<Element>,
    /// Caller-owned actions, laid out in a row or, when compact, a column.
    #[props(default)]
    actions: Option<Element>,
    /// Semantic heading level, independent of visual size.
    #[props(default = HeadingLevel::Three)]
    heading_level: HeadingLevel,
    /// Reduce insets and stack the actions vertically.
    #[props(default)]
    is_compact: bool,
    /// Caller classes on the root.
    #[props(default)]
    class: String,
    /// Caller inline CSS on the root.
    #[props(default)]
    style: String,
    /// Native root attributes and events.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let id = format!("empty-state-{}", current_scope_id().0);
    let mounted = use_announce_rendered_text(
        format!("[data-astryx-empty-state-text='{id}']"),
        true,
        String::new(),
    );
    let variant = is_compact.then_some("compact");
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style" | "data-variant"))
        .collect();
    macro_rules! heading {
        ($tag:ident) => {
            rsx! { $tag { class: "astryx-empty-state-title", "data-variant": variant, "{title}" } }
        };
    }
    let heading = match heading_level {
        HeadingLevel::One => heading!(h1),
        HeadingLevel::Two => heading!(h2),
        HeadingLevel::Three => heading!(h3),
        HeadingLevel::Four => heading!(h4),
        HeadingLevel::Five => heading!(h5),
        HeadingLevel::Six => heading!(h6),
    };
    rsx! {
        div { class: "astryx-empty-state {class}", "data-variant": variant, style: (!style.is_empty()).then_some(style), ..attributes,
            if let Some(icon) = icon.filter(is_renderable) {
                div { "aria-hidden": "true", {icon} }
            }
            div { class: "astryx-empty-state__text", "data-astryx-empty-state-text": id, onmounted: mounted,
                {heading}
                if let Some(description) = description.filter(|description| !description.is_empty()) {
                    div { class: "astryx-empty-state-description", "data-variant": variant, "{description}" }
                }
            }
            if let Some(actions) = actions.filter(is_renderable) {
                div { class: "astryx-empty-state__actions", {actions} }
            }
        }
    }
}
