use dioxus::prelude::*;

use crate::link::{LinkComponentType, LinkRenderProps, use_link_component};
use crate::utils::disabled_interaction::{cancel_disabled_click, cancel_disabled_key};
use crate::utils::is_renderable::is_renderable;
use crate::utils::safe_url::is_safe_url;

/// An item for the horizontal top navigation bar.
#[component]
pub fn TopNavItem(
    label: String,
    #[props(default)] href: Option<String>,
    #[props(default)] target: Option<String>,
    #[props(default)] rel: Option<String>,
    #[props(default)] render_link: Option<LinkComponentType>,
    #[props(default)] is_selected: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_icon_only: bool,
    #[props(default)] icon: Option<Element>,
    #[props(default)] children: Option<Element>,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let renderer = use_link_component(render_link);
    let icon = icon.filter(is_renderable);
    let is_icon_only = is_icon_only && icon.is_some();
    let is_action = href.is_none() && onclick.is_some();
    let is_disabled = is_disabled || href.as_ref().is_some_and(|url| !is_safe_url(url));
    let href = href.filter(|_| !is_disabled);
    let class = format!("astryx-top-nav-item {class}");
    let mut attributes = vec![Attribute::new("class", class, None, false)];
    if !style.is_empty() {
        attributes.push(Attribute::new("style", style, None, false));
    }
    if is_icon_only || children.as_ref().is_some_and(is_renderable) {
        attributes.push(Attribute::new("aria-label", label.clone(), None, false));
    }
    if is_icon_only {
        attributes.push(Attribute::new("data-icon-only", "true", None, false));
    }
    if is_selected {
        attributes.push(Attribute::new("aria-current", "page", None, false));
    }
    if is_disabled {
        attributes.push(Attribute::new("aria-disabled", "true", None, false));
    }
    let content = rsx! {
        if let Some(icon) = icon { span { class: "astryx-top-nav-item__icon", "aria-hidden": "true", {icon} } }
        if !is_icon_only {
            {children.filter(is_renderable).unwrap_or_else(|| rsx! { "{label}" })}
        }
    };
    let activate = move |event: MouseEvent| {
        if is_disabled {
            cancel_disabled_click(&event);
        } else if let Some(handler) = onclick {
            handler.call(event);
        }
    };
    if is_action {
        return rsx! { button { r#type: "button", onclick: activate, onkeydown: move |event| { if is_disabled { cancel_disabled_key(&event); } }, ..attributes, {content} } };
    }
    if href.is_some()
        && let Some(renderer) = renderer
    {
        return renderer.call(LinkRenderProps {
            href,
            target,
            rel,
            attributes,
            children: content,
            onclick: Callback::new(activate),
            onkeydown: EventHandler::default(),
            onmounted: EventHandler::default(),
        });
    }
    let target = href.as_ref().and(target);
    let rel = href.as_ref().and(rel);
    rsx! {
        a { href, target, rel, role: is_disabled.then_some("link"), tabindex: is_disabled.then_some(0),
            onclick: activate, onkeydown: move |event| { if is_disabled { cancel_disabled_key(&event); } },
            ..attributes, {content}
        }
    }
}
