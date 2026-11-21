#[cfg(target_arch = "wasm32")]
use std::rc::Rc;

use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{HtmlElement, MouseEvent as DomMouseEvent};

#[cfg(target_arch = "wasm32")]
use crate::hooks::use_clickable_container::native::{
    allows_activation, delegate_action, navigate, use_native_event_listener, use_native_listener,
};

/// Delegate only background clicks; nested controls keep native ownership.
pub(super) fn use_token_surface(
    id: String,
    is_enabled: bool,
    href: Option<String>,
) -> EventHandler<MountedEvent> {
    #[cfg(target_arch = "wasm32")]
    {
        let selector = format!("[data-astryx-token-id='{id}']");
        let click = use_native_event_listener(
            selector.clone(),
            "click",
            false,
            Rc::new({
                let href = href.clone();
                move |event: web_sys::Event| {
                    let Some(mouse) = event.dyn_ref::<DomMouseEvent>() else {
                        return;
                    };
                    let Some(root) = event
                        .current_target()
                        .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                    else {
                        return;
                    };
                    if event.default_prevented() || !allows_activation(mouse, &root, !is_enabled) {
                        return;
                    }
                    let Some(action) = root
                        .query_selector(":scope > .astryx-token__action")
                        .ok()
                        .flatten()
                        .and_then(|node| node.dyn_into::<HtmlElement>().ok())
                    else {
                        return;
                    };
                    if let Some(href) = &href {
                        navigate(
                            mouse,
                            href,
                            action.get_attribute("target").as_deref(),
                            Some(&action),
                        );
                        event.stop_propagation();
                    } else {
                        delegate_action(mouse, &action);
                    }
                }
            }),
        );
        let middle = use_native_listener(
            selector,
            "auxclick",
            false,
            EventHandler::new(move |event: web_sys::Event| {
                let Some(href) = &href else { return };
                let Some(mouse) = event.dyn_ref::<DomMouseEvent>() else {
                    return;
                };
                let Some(root) = event
                    .current_target()
                    .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                else {
                    return;
                };
                if mouse.button() == 1
                    && !event.default_prevented()
                    && allows_activation(mouse, &root, !is_enabled)
                {
                    navigate(mouse, href, None, None);
                    event.prevent_default();
                }
            }),
        );
        EventHandler::new(move |event: MountedEvent| {
            click.call(event.clone());
            middle.call(event);
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (id, is_enabled, href);
        EventHandler::default()
    }
}
