use dioxus::prelude::*;

use crate::utils::disabled_interaction::{cancel_disabled_click, cancel_disabled_key};
use crate::utils::safe_url::is_safe_url;

use super::link_context::LinkContext;
use super::types::{LinkComponentType, LinkRenderProps};

/// Resolves an explicit renderer before the nearest provider. None uses an anchor.
pub fn use_link_component(component: Option<LinkComponentType>) -> Option<LinkComponentType> {
    let inherited = try_consume_context::<LinkContext>().map(|context| (context.0)());
    component.or(inherited)
}

/// Renders a navigation heading anchor without the inline Link treatment.
pub(crate) fn render_navigation_anchor(
    href: String,
    renderer: Option<LinkComponentType>,
    attributes: Vec<Attribute>,
    children: Element,
    onmounted: Option<EventHandler<MountedEvent>>,
) -> Element {
    render_navigation_anchor_with_click(href, renderer, attributes, children, onmounted, None)
}

pub(crate) fn render_navigation_anchor_with_click(
    href: String,
    renderer: Option<LinkComponentType>,
    attributes: Vec<Attribute>,
    children: Element,
    onmounted: Option<EventHandler<MountedEvent>>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let is_safe = is_safe_url(&href);
    if let Some(renderer) = renderer.filter(|_| is_safe) {
        return renderer.call(LinkRenderProps {
            href: Some(href),
            target: None,
            rel: None,
            attributes,
            children,
            onclick: onclick.unwrap_or_default(),
            onkeydown: EventHandler::default(),
            onmounted: onmounted.unwrap_or_default(),
        });
    }
    rsx! {
        a { href: is_safe.then_some(href), role: (!is_safe).then_some("link"),
            tabindex: (!is_safe).then_some("0"), "aria-disabled": (!is_safe).then_some("true"),
            onclick: move |event| {
                if !is_safe { cancel_disabled_click(&event); }
                else if let Some(handler) = onclick { handler.call(event); }
            },
            onkeydown: move |event| { if !is_safe { cancel_disabled_key(&event); } },
            onmounted: move |event| { if let Some(handler) = onmounted { handler.call(event); } },
            ..attributes, {children}
        }
    }
}
