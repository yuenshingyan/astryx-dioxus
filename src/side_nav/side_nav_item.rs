use dioxus::prelude::*;

use crate::link::{LinkComponentType, render_navigation_anchor_with_click, use_link_component};
use crate::utils::disabled_interaction::{cancel_disabled_click, cancel_disabled_key};
use crate::utils::is_renderable::is_renderable;
use crate::{ElementSize, use_size};

/// An expanded-rail leaf link or action. End content must not contain controls.
#[component]
pub fn SideNavItem(
    label: String,
    #[props(default)] href: Option<String>,
    #[props(default)] render_link: Option<LinkComponentType>,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] icon: Option<Element>,
    #[props(default)] selected_icon: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] is_selected: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] size: Option<ElementSize>,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let renderer = use_link_component(render_link);
    let size = use_size(size).as_str();
    let icon = selected_icon
        .filter(|icon| is_selected && is_renderable(icon))
        .or_else(|| icon.filter(is_renderable));
    let content = rsx! {
        if let Some(icon) = icon { span { class: "astryx-side-nav-item__icon", "aria-hidden": "true", {icon} } }
        span { class: "astryx-side-nav-item__label", "{label}" }
        if let Some(end_content) = end_content.filter(is_renderable) {
            span { class: "astryx-side-nav-item__end", {end_content} }
        }
    };
    let mut attributes = vec![
        Attribute::new(
            "class",
            format!("astryx-side-nav-item {class}"),
            None,
            false,
        ),
        Attribute::new("data-size", size, None, false),
    ];
    if !style.is_empty() {
        attributes.push(Attribute::new("style", style, None, false));
    }
    if is_selected {
        attributes.push(Attribute::new("aria-current", "page", None, false));
    }
    rsx! {
        div { class: "astryx-side-nav-item-root",
            {if let Some(href) = href.filter(|href| !is_disabled && !href.is_empty()) {
                render_navigation_anchor_with_click(href, renderer, attributes, content, None, onclick)
            } else {
                rsx! {
                    button { r#type: "button", "aria-disabled": is_disabled.then_some("true"),
                        onclick: move |event| {
                            if is_disabled { cancel_disabled_click(&event); }
                            else if let Some(handler) = onclick { handler.call(event); }
                        },
                        onkeydown: move |event| { if is_disabled { cancel_disabled_key(&event); } },
                        ..attributes, {content}
                    }
                }
            }}
        }
    }
}
