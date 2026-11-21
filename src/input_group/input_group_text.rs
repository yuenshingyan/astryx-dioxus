use dioxus::core::{DynamicNode, TemplateNode};
use dioxus::prelude::*;

use crate::utils::text_direction::text_direction;

fn plain_direction(children: &Element) -> Option<&'static str> {
    let node = children.as_ref().ok()?;
    match node.template.roots {
        [TemplateNode::Text { text }] => text_direction(text),
        [TemplateNode::Dynamic { id }] => match node.dynamic_root(*id) {
            Some(DynamicNode::Text(text)) => text_direction(&text.value),
            _ => None,
        },
        _ => None,
    }
}

/// Noninteractive prefix or suffix for an input group.
#[component]
pub fn InputGroupText(
    /// Text or caller-owned artwork.
    children: Element,
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
    let direction = plain_direction(&children);
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| attr.name != "data-astryx-input-group-item")
        .collect();
    rsx! {
        div { class: "astryx-input-group-text {class}", style, "data-astryx-input-group-item": "", ..attributes,
            if let Some(direction) = direction { span { dir: direction, {children} } }
            else { {children} }
        }
    }
}
