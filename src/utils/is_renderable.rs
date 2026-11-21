//! Renderable slots may contain elements, text, or fragments of either.

use dioxus::core::{DynamicNode, TemplateNode, VNode};
use dioxus::prelude::Element;

pub(crate) fn is_renderable(node: &Element) -> bool {
    // Let error boundaries see errors rather than treating them as empty slots.
    node.as_ref().map_or(true, has_node)
}

fn has_node(node: &VNode) -> bool {
    node.template.roots.iter().any(|root| match root {
        TemplateNode::Text { text } => !text.is_empty(),
        TemplateNode::Dynamic { id } => match node.dynamic_root(*id) {
            Some(DynamicNode::Placeholder(_)) | None => false,
            Some(DynamicNode::Fragment(nodes)) => nodes.iter().any(has_node),
            Some(DynamicNode::Text(text)) => !text.value.is_empty(),
            _ => true,
        },
        _ => true,
    })
}
