use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;

/// Native element used by the static row.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ItemTag {
    #[default]
    Div,
    Li,
    Span,
}

/// Vertical alignment of a row's slots.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ItemAlign {
    #[default]
    Center,
    Start,
    FirstLine,
}

/// Padding density of an item.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ItemDensity {
    Compact,
    #[default]
    Balanced,
    Spacious,
}

/// Arrangement of the label and description.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ItemLayout {
    #[default]
    Stacked,
    Inline,
}

/// Text or rich inline item content. Plain text ellipsizes by default.
#[derive(Clone, PartialEq)]
pub enum ItemLabel {
    Text(String),
    Rich(Element),
}

impl From<&str> for ItemLabel {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

impl From<String> for ItemLabel {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<Element> for ItemLabel {
    fn from(value: Element) -> Self {
        Self::Rich(value)
    }
}

/// Presentational row with a marker, leading content, label, description, and trailing content.
/// The row adds no focus target; controls supplied in slots retain their native behavior.
#[component]
pub fn Item(
    #[props(into)] label: ItemLabel,
    #[props(default)] description: Option<ItemLabel>,
    #[props(default)] marker: Option<Element>,
    #[props(default)] start_content: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] tag: ItemTag,
    #[props(default)] align: ItemAlign,
    #[props(default)] density: ItemDensity,
    #[props(default)] layout: ItemLayout,
    #[props(default)] label_lines: Option<u32>,
    #[props(default)] description_lines: Option<u32>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let description = description.filter(|description| match description {
        ItemLabel::Text(text) => !text.is_empty(),
        ItemLabel::Rich(content) => is_renderable(content),
    });
    let is_inline = layout == ItemLayout::Inline && description.is_some();
    let density = match density {
        ItemDensity::Compact => "compact",
        ItemDensity::Balanced => "balanced",
        ItemDensity::Spacious => "spacious",
    };
    let align = match align {
        ItemAlign::Center => "center",
        ItemAlign::Start => "start",
        ItemAlign::FirstLine => "first-line",
    };
    let class = format!("astryx-item {class}");
    let attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class" | "style" | "data-density" | "data-align" | "data-layout"
            )
        })
        .collect::<Vec<_>>();
    let content = rsx! {
        {marker.filter(is_renderable)}
        if let Some(start) = start_content.filter(is_renderable) { span { class: "astryx-item__start", {start} } }
        span { class: "astryx-item__content",
            {render_label(label, if is_inline { Some(1) } else { label_lines }, "astryx-item__label")}
            if let Some(description) = description {
                {render_label(description, if is_inline { Some(1) } else { description_lines }, "astryx-item__description")}
            }
        }
        if let Some(end) = end_content.filter(is_renderable) { span { class: "astryx-item__end", {end} } }
    };
    let layout = if is_inline { "inline" } else { "stacked" };
    match tag {
        ItemTag::Div => {
            rsx! { div { class, style, "data-density": density, "data-align": align, "data-layout": layout, ..attributes, {content} } }
        }
        ItemTag::Li => {
            rsx! { li { class, style, "data-density": density, "data-align": align, "data-layout": layout, ..attributes, {content} } }
        }
        ItemTag::Span => {
            rsx! { span { class, style, "data-density": density, "data-align": align, "data-layout": layout, ..attributes, {content} } }
        }
    }
}

fn render_label(label: ItemLabel, lines: Option<u32>, class: &'static str) -> Element {
    let (content, lines) = match label {
        ItemLabel::Text(text) => (rsx! { "{text}" }, lines.or(Some(1))),
        ItemLabel::Rich(content) => (content, lines),
    };
    let clamp = match lines {
        Some(1) => Some("single"),
        Some(lines) if lines > 1 => Some("multi"),
        _ => None,
    };
    let style = lines
        .filter(|&lines| lines > 1)
        .map(|lines| format!("-webkit-line-clamp:{lines};"));
    rsx! { span { class, "data-clamp": clamp, style, {content} } }
}
