//! Non-menu breadcrumb item adapted from Astryx Breadcrumbs/BreadcrumbItem.tsx.

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::component::BreadcrumbContext;
use crate::link::{LinkComponentType, LinkRenderProps, use_link_component};
use crate::utils::disabled_interaction::{cancel_disabled_click, cancel_disabled_key};
use crate::utils::is_renderable::is_renderable;
use crate::utils::safe_url::is_safe_url;

/// One item in a breadcrumb ordered list. The current page is not interactive.
#[component]
pub fn BreadcrumbItem(
    /// Label content, optionally including rich text.
    children: Element,
    /// Navigation destination for a non-current item.
    #[props(default)]
    href: Option<String>,
    /// Activation handler for a non-current item, with or without a destination.
    #[props(default)]
    on_click: Option<EventHandler<MouseEvent>>,
    /// Explicitly select or exclude this item as current; omitted enables last-item detection.
    #[props(default)]
    is_current: Option<bool>,
    /// Artwork before the label.
    #[props(default)]
    start_icon: Option<Element>,
    /// Override the link renderer supplied by LinkProvider.
    #[props(default)]
    render_link: Option<LinkComponentType>,
    /// Caller classes on the list item.
    #[props(default)]
    class: String,
    /// Caller inline declarations on the list item.
    #[props(default)]
    style: String,
    /// Mounted callback for the native list item.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native attributes and events on the list item.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    const MARKER: &str = "data-astryx-breadcrumb-item";
    let id = format!("breadcrumb-item-{}", current_scope_id().0);
    let renderer = use_link_component(render_link);
    let settings = try_consume_context::<BreadcrumbContext>()
        .map(|context| (context.0)())
        .unwrap_or_default();
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(false);
        }
    });
    let content = rsx! {
        if let Some(icon) = start_icon.filter(is_renderable) {
            span { class: "astryx-breadcrumb-item__icon", {icon} }
        }
        {children}
    };
    let state = match is_current {
        Some(true) => "current",
        Some(false) => "excluded",
        None => "auto",
    };
    let label = if is_current == Some(true) || (href.is_none() && on_click.is_none()) {
        rsx! { span { class: "astryx-breadcrumb-item__content", "aria-current": (is_current == Some(true)).then_some("page"), {content} } }
    } else if let Some(href) = href {
        let is_safe = is_safe_url(&href);
        if let Some(renderer) = renderer.filter(|_| is_safe) {
            renderer.call(LinkRenderProps {
                href: Some(href),
                target: None,
                rel: None,
                attributes: vec![Attribute::new(
                    "class",
                    "astryx-breadcrumb-item__content astryx-breadcrumb-item__link",
                    None,
                    false,
                )],
                onclick: on_click.unwrap_or_default(),
                onkeydown: EventHandler::default(),
                onmounted: EventHandler::default(),
                children: content,
            })
        } else {
            rsx! { a { class: "astryx-breadcrumb-item__content astryx-breadcrumb-item__link", href: is_safe.then_some(href),
            role: (!is_safe).then_some("link"), tabindex: (!is_safe).then_some("0"), "aria-disabled": (!is_safe).then_some("true"),
            onclick: move |event| { if !is_safe { cancel_disabled_click(&event); } else if let Some(callback) = on_click { callback.call(event); } },
            onkeydown: move |event| { if !is_safe { cancel_disabled_key(&event); } }, {content} } }
        }
    } else {
        rsx! { button { r#type: "button", class: "astryx-breadcrumb-item__content astryx-breadcrumb-item__link", onclick: on_click.unwrap_or_default(), {content} } }
    };
    let mount_id = id.clone();
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class" | "style" | MARKER | "data-current" | "data-variant" | "onmounted"
            )
        })
        .collect();
    rsx! {
        li { class: "astryx-breadcrumb-item {class}", style: (!style.is_empty()).then_some(style),
            "data-astryx-breadcrumb-item": id, "data-current": state, "data-variant": settings.variant.as_str(),
            onmounted: move |event| {
                if let Some(previous) = *controller.peek() { let _ = previous.send(false); }
                let eval = document::eval(include_str!("breadcrumb_current.js"));
                let _ = eval.send(mount_id.clone());
                controller.set(Some(eval));
                if let Some(callback) = onmounted { callback.call(event); }
            },
            ..attributes,
            span { class: "astryx-breadcrumb-item__separator", "aria-hidden": "true",
                if let Some(separator) = settings.separator { {separator} }
                else { span { class: "astryx-breadcrumb-item__slash", "/" } }
            }
            {label}
        }
    }
}
