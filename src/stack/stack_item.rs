use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use super::component::{StackElement, StackRenderProps};
use crate::hooks::use_scroll_region_props::use_scroll_region;

/// Flex sizing for an item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackItemSize {
    /// Keep intrinsic size without growing or shrinking.
    Static,
    /// Grow into remaining space.
    Fill,
}

/// Overrides the stack's cross-axis alignment for one item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackItemCrossAlignSelf {
    /// Align to the start of the cross axis.
    Start,
    /// Center on the cross axis.
    Center,
    /// Align to the end of the cross axis.
    End,
    /// Stretch across the cross axis.
    Stretch,
}

/// StackItem root selection.
#[derive(Clone, Copy, Default, PartialEq)]
pub enum StackItemRoot {
    /// Render a div with optional automatic scroll semantics.
    #[default]
    Div,
    /// Render a span with optional automatic scroll semantics.
    Span,
    /// Pass attributes and children to one caller-rendered root without adding semantics.
    Custom(StackElement),
}

/// Flex participation and optional overflow for one child of a Stack.
#[component]
pub fn StackItem(
    /// Defaults to static sizing without reflecting a size attribute.
    #[props(default)]
    size: Option<StackItemSize>,
    /// Override cross-axis alignment.
    #[props(default)]
    cross_align_self: Option<StackItemCrossAlignSelf>,
    /// Scroll on overflow.
    #[props(default)]
    is_scrollable: bool,
    /// Native or caller-rendered root.
    #[props(default)]
    root: StackItemRoot,
    /// Localized fallback name for an overflowing item with no caller label.
    #[props(default = "Content".into())]
    scroll_label: String,
    /// Caller CSS classes.
    #[props(default)]
    class: String,
    /// Caller inline CSS.
    #[props(default)]
    style: String,
    /// Caller children.
    #[props(default = rsx! {})]
    children: Element,
    /// Native root attributes and callbacks.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let has_attribute = |name| {
        attributes
            .iter()
            .any(|attr| attr.name == name && !matches!(attr.value, AttributeValue::None))
    };
    let should_measure =
        is_scrollable && !matches!(root, StackItemRoot::Custom(_)) && !has_attribute("role");
    let id = format!("stack-item-{}", current_scope_id().0);
    let is_group = use_scroll_region(
        "data-astryx-item-scroll",
        id.clone(),
        matches!(root, StackItemRoot::Span),
        should_measure,
    );
    let size_name = size.map(|size| match size {
        StackItemSize::Static => "static",
        StackItemSize::Fill => "fill",
    });
    let alignment = match cross_align_self {
        Some(StackItemCrossAlignSelf::Start) => " astryx-stack-item--start",
        Some(StackItemCrossAlignSelf::Center) => " astryx-stack-item--center",
        Some(StackItemCrossAlignSelf::End) => " astryx-stack-item--end",
        Some(StackItemCrossAlignSelf::Stretch) => " astryx-stack-item--stretch",
        None => "",
    };
    let scroll = if is_scrollable {
        " astryx-stack-item--scrollable"
    } else {
        ""
    };
    let mut classes = format!("astryx-stack-item{alignment}{scroll}");
    if !class.is_empty() {
        classes.push(' ');
        classes.push_str(&class);
    }
    let mut attrs = vec![Attribute::new("class", classes, None, false)];
    if !style.is_empty() {
        attrs.push(Attribute::new("style", style, None, false));
    }
    if let Some(name) = size_name {
        attrs.push(Attribute::new("data-size", name, None, false));
    }
    if should_measure {
        attrs.push(Attribute::new("data-astryx-item-scroll", id, None, false));
    }
    let has_label = has_attribute("aria-label");
    if is_group {
        attrs.push(Attribute::new("role", "group", None, false));
        if !has_label {
            attrs.push(Attribute::new("aria-label", scroll_label, None, false));
        }
    }
    attrs.extend(attributes.into_iter().filter(|attr| {
        !matches!(attr.name, "class" | "style" | "data-astryx-item-scroll")
            && !(is_group
                && matches!(attr.value, AttributeValue::None)
                && (attr.name == "role" || (attr.name == "aria-label" && !has_label)))
    }));
    attrs.sort_by_key(|attr| attr.name);
    match root {
        StackItemRoot::Div => rsx! { div { ..attrs, {children} } },
        StackItemRoot::Span => rsx! { span { ..attrs, {children} } },
        StackItemRoot::Custom(render) => render.call(StackRenderProps {
            attributes: attrs,
            children,
        }),
    }
}
