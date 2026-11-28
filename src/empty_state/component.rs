//! Adapted from Astryx EmptyState/EmptyState.tsx.

use dioxus::core::{DynamicNode, TemplateNode, current_scope_id};
use dioxus::prelude::*;

use crate::heading::HeadingLevel;
use crate::hooks::use_announce_rendered_text::use_announce_rendered_text;

fn has_slot_content(slot: &Element) -> bool {
    // Preserve errors for the caller's error boundary, as with other slots.
    slot.as_ref().map_or(true, |node| {
        node.template.roots.iter().any(|root| match root {
            TemplateNode::Text { text } => !text.is_empty(),
            TemplateNode::Dynamic { id } => match node.dynamic_root(*id) {
                Some(DynamicNode::Placeholder(_)) | None => false,
                Some(DynamicNode::Text(text)) => !text.value.is_empty(),
                // Upstream checks containers shallowly. Dioxus also uses a
                // one-placeholder fragment for a direct RSX `if false`.
                Some(DynamicNode::Fragment(nodes)) => match nodes.as_slice() {
                    [] => false,
                    [child] => *child != VNode::placeholder(),
                    _ => true,
                },
                Some(DynamicNode::Component(_)) => true,
            },
            TemplateNode::Element { .. } => true,
        })
    })
}

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
            if let Some(icon) = icon.filter(has_slot_content) {
                div { "aria-hidden": "true", {icon} }
            }
            div { class: "astryx-empty-state__text", "data-astryx-empty-state-text": id, onmounted: mounted,
                {heading}
                if let Some(description) = description.filter(|description| !description.is_empty()) {
                    div { class: "astryx-empty-state-description", "data-variant": variant, "{description}" }
                }
            }
            if let Some(actions) = actions.filter(has_slot_content) {
                div { class: "astryx-empty-state__actions", {actions} }
            }
        }
    }
}
