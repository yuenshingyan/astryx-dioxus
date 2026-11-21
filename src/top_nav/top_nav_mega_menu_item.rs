use dioxus::prelude::*;

use crate::link::{LinkComponentType, render_navigation_anchor_with_click, use_link_component};
use crate::utils::is_renderable::is_renderable;

/// A desktop mega-menu tile. Without a destination or action it is static.
#[component]
pub fn TopNavMegaMenuItem(
    /// Primary item text.
    title: String,
    /// Supporting text beneath the title.
    #[props(default)]
    description: Option<String>,
    /// Caller-supplied artwork.
    #[props(default)]
    icon: Option<Element>,
    /// Link destination, if the tile navigates.
    #[props(default)]
    href: Option<String>,
    /// Optional activation callback, including for link tiles.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    /// Overrides the nearest LinkProvider for link tiles.
    #[props(default)]
    render_link: Option<LinkComponentType>,
    /// Additional root classes.
    #[props(default)]
    class: String,
    /// Inline root declarations.
    #[props(default)]
    style: Option<String>,
    /// Mounted callback for the selected root element.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native root attributes, including caller-managed tabindex.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let renderer = use_link_component(render_link);
    let content = rsx! {
        if let Some(icon) = icon.filter(is_renderable) {
            span { class: "astryx-top-nav-mega-menu-item__icon", {icon} }
        }
        span { class: "astryx-top-nav-mega-menu-item__content",
            span { class: "astryx-top-nav-mega-menu-item__title", "{title}" }
            if let Some(description) = description.filter(|text| !text.is_empty()) {
                span { class: "astryx-top-nav-mega-menu-item__description", "{description}" }
            }
        }
    };
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "class" | "style" | "href" | "type" | "onclick" | "onmounted"
            )
        })
        .collect();
    attributes.push(Attribute::new(
        "class",
        format!("astryx-top-nav-mega-menu-item {class}"),
        None,
        false,
    ));
    if let Some(style) = style {
        attributes.push(Attribute::new("style", style, None, false));
    }
    if let Some(href) = href.filter(|href| !href.is_empty()) {
        return render_navigation_anchor_with_click(
            href, renderer, attributes, content, onmounted, onclick,
        );
    }
    if let Some(onclick) = onclick {
        return rsx! { button { r#type: "button", onclick, onmounted: move |event| { if let Some(handler) = onmounted { handler.call(event); } }, ..attributes, {content} } };
    }
    rsx! { div { onmounted: move |event| { if let Some(handler) = onmounted { handler.call(event); } }, ..attributes, {content} } }
}
