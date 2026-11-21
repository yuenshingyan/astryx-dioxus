//! Description-list composition adapted from Astryx MetadataList.tsx.

use std::num::NonZeroU32;

use dioxus::core::{DynamicNode, Template, TemplateNode, VNode, current_scope_id};
use dioxus::prelude::*;

use super::metadata_list_context::MetadataListContext;
use crate::utils::is_renderable::is_renderable;
use crate::utils::types::SizeValue;

/// Column behavior for a vertical metadata list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MetadataListColumns {
    /// One column of items.
    #[default]
    Single,
    /// Fill available space with columns at least 280 pixels wide.
    Multi,
    /// Fixed nonzero count of item columns.
    Fixed(NonZeroU32),
}

/// Placement of each label relative to its value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetadataListLabelPosition {
    /// Put the label at the inline start of its value.
    Start,
    /// Put the label above the value.
    Top,
}

/// Optional vertical-list label overrides.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MetadataListLabelConfig {
    /// Defaults to Start for a single column and Top for multiple columns.
    pub position: Option<MetadataListLabelPosition>,
    /// Width of the label track when applicable.
    pub width: Option<SizeValue>,
}

/// Flow of the list items.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MetadataListOrientation {
    /// Arrange items in a grid.
    #[default]
    Vertical,
    /// Wrap stacked pairs in a row; ignore column and collapse settings.
    Horizontal,
}

// A compiled RSX fragment can contain multiple component roots in a single
// template. Isolate those dynamic roots without adding nodes to the <dl>.
const ITEM_TEMPLATE: Template = Template {
    roots: &[TemplateNode::Dynamic { id: 0 }],
    node_paths: &[&[0]],
    attr_paths: &[],
};

fn collect_items(node: &VNode, path: &str, items: &mut Vec<VNode>) {
    if !node.template.is_completely_dynamic() {
        // Preserve caller-authored static markup as an indivisible group.
        items.push(VNode::new(
            Some(path.into()),
            node.template,
            node.dynamic_nodes.clone(),
            node.dynamic_attrs.clone(),
        ));
        return;
    }
    for (index, root) in node.template.roots.iter().enumerate() {
        let TemplateNode::Dynamic { id } = root else {
            continue;
        };
        let Some(dynamic) = node.dynamic_root(*id) else {
            continue;
        };
        match dynamic {
            DynamicNode::Placeholder(_) => {}
            DynamicNode::Text(text) if text.value.is_empty() => {}
            DynamicNode::Fragment(nodes) => {
                for (child_index, child) in nodes.iter().enumerate() {
                    let segment = child.key.as_ref().map_or_else(
                        || format!("i{child_index}"),
                        |key| format!("k{}:{key}", key.len()),
                    );
                    collect_items(child, &format!("{path}/{index}/{segment}"), items);
                }
            }
            _ => items.push(VNode::new(
                Some(format!("{path}/{index}")),
                ITEM_TEMPLATE,
                Box::new([dynamic.clone()]),
                Box::new([]),
            )),
        }
    }
}

/// A semantic description list for key-value metadata.
#[component]
pub fn MetadataList(
    /// MetadataListItem children, including conditionals and keyed loops.
    children: Element,
    /// Vertical grid column mode.
    #[props(default)]
    columns: MetadataListColumns,
    /// Vertical label placement and width overrides.
    #[props(default)]
    label: MetadataListLabelConfig,
    /// Maximum visible items in collapsed vertical mode.
    #[props(default)]
    max_num_of_items: Option<usize>,
    /// Vertical or wrapping horizontal layout.
    #[props(default)]
    orientation: MetadataListOrientation,
    /// Optional content above the list.
    #[props(default)]
    title: Option<Element>,
    /// Localized label for the expand button.
    #[props(default = "Show more".into())]
    show_more_label: String,
    /// Localized label for the collapse button.
    #[props(default = "Show less".into())]
    show_less_label: String,
    /// Caller classes on the root.
    #[props(default)]
    class: String,
    /// Caller inline CSS on the root.
    #[props(default)]
    style: String,
    /// Native attributes and callbacks on the root.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let mut is_expanded = use_signal(|| false);
    let content_id = format!("astryx-metadata-list-{}", current_scope_id().0);
    let controls_id = content_id.clone();
    let is_horizontal = orientation == MetadataListOrientation::Horizontal;
    let (column_name, is_multi, count) = match columns {
        MetadataListColumns::Single => ("single".into(), false, None),
        MetadataListColumns::Multi => ("multi".into(), true, None),
        MetadataListColumns::Fixed(count) => {
            (count.to_string(), count.get() > 1, Some(count.get()))
        }
    };
    let is_stacked = is_horizontal
        || label.position.map_or(is_multi, |position| {
            position == MetadataListLabelPosition::Top
        });
    let mut context = use_context_provider(|| MetadataListContext {
        is_stacked: Signal::new(is_stacked),
    });
    if *context.is_stacked.peek() != is_stacked {
        context.is_stacked.set(is_stacked);
    }
    let track_style = if is_horizontal {
        None
    } else if let Some(count) = count.filter(|count| *count > 1) {
        let tracks = if is_stacked {
            "1fr"
        } else {
            "auto minmax(0, 1fr)"
        };
        Some(format!("grid-template-columns:repeat({count}, {tracks});"))
    } else if !is_stacked {
        label
            .width
            .map(|width| {
                width
                    .as_css()
                    .map(|css| format!("grid-template-columns:{css} minmax(0, 1fr);"))
            })
            .transpose()?
    } else {
        None
    };
    let mode = if is_horizontal {
        "horizontal"
    } else if is_multi {
        "multi"
    } else if is_stacked {
        "stacked"
    } else {
        "inline"
    };
    let mut has_toggle = false;
    let visible = match (is_horizontal, children, max_num_of_items) {
        (false, Ok(node), Some(limit)) => {
            let mut items = Vec::new();
            collect_items(&node, "item", &mut items);
            has_toggle = items.len() > limit;
            if !is_expanded() {
                items.truncate(limit);
            }
            rsx! { {items.into_iter()} }
        }
        (_, children, _) => children,
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class" | "style" | "data-columns" | "data-orientation"
            )
        })
        .collect();
    rsx! {
        div { class: "astryx-metadata-list {class}", style: (!style.is_empty()).then_some(style),
            "data-columns": column_name, "data-orientation": if is_horizontal { "horizontal" } else { "vertical" }, ..attributes,
            if let Some(title) = title.filter(is_renderable) { div { class: "astryx-metadata-list__title", {title} } }
            dl { id: content_id, class: "astryx-metadata-list__items astryx-metadata-list__items--{mode}", style: track_style, {visible} }
            if has_toggle {
                button { r#type: "button", class: "astryx-metadata-list__toggle", "aria-controls": controls_id,
                    "aria-expanded": if is_expanded() { "true" } else { "false" }, onclick: move |_| is_expanded.toggle(),
                    if is_expanded() { {show_less_label} } else { {show_more_label} }
                }
            }
        }
    }
}
