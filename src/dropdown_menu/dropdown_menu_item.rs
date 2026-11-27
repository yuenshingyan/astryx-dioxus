use dioxus::core::AttributeValue;
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;

use crate::utils::safe_url::is_safe_url;
use crate::{Item, ItemLabel, LinkRenderProps, use_link_component};

/// Browser target for a destination row.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DropdownMenuItemTarget {
    #[default]
    SameTab,
    NewTab,
}

/// Foreground treatment of a menu action.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum DropdownMenuItemVariant {
    #[default]
    Default,
    Destructive,
}

/// An action row whose focus and keyboard activation are owned by its menu.
#[component]
pub fn DropdownMenuItem(
    #[props(into)] label: ItemLabel,
    #[props(default)] description: Option<ItemLabel>,
    #[props(default)] icon: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] is_disabled: bool,
    /// A destination turns the row into a link.
    #[props(default)]
    href: Option<String>,
    #[props(default)] target: DropdownMenuItemTarget,
    #[props(default)] rel: Option<String>,
    /// Set to false to keep the menu open for in-place feedback after activation.
    #[props(default = true)]
    has_close_on_select: bool,
    #[props(default)] variant: DropdownMenuItemVariant,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let renderer = use_link_component(None);
    let is_link = href.is_some();
    let href = href.filter(|url| !is_disabled && is_safe_url(url));
    let is_disabled = is_disabled || (is_link && href.is_none());
    let mut attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "role"
                    | "tabindex"
                    | "aria-disabled"
                    | "onclick"
                    | "data-variant"
                    | "data-astryx-menu-keep-open"
                    | "href"
                    | "target"
                    | "rel"
                    | "draggable"
            ) && !(is_disabled && attr.name == "onmousedown")
        })
        .collect::<Vec<_>>();
    attributes.extend([
        Attribute::new("role", "menuitem", None, false),
        Attribute::new("tabindex", "-1", None, false),
        Attribute::new(
            "data-variant",
            match variant {
                DropdownMenuItemVariant::Default => "default",
                DropdownMenuItemVariant::Destructive => "destructive",
            },
            None,
            false,
        ),
    ]);
    if !has_close_on_select {
        attributes.push(Attribute::new(
            "data-astryx-menu-keep-open",
            "true",
            None,
            false,
        ));
    }
    if is_disabled {
        attributes.push(Attribute::new("aria-disabled", "true", None, false));
        attributes.push(Attribute::new(
            "onmousedown",
            AttributeValue::listener(|event: Event<PlatformEventData>| event.prevent_default()),
            None,
            false,
        ));
    }
    let class = format!("astryx-dropdown-menu-item astryx-dropdown-menu-item--action {class}");
    if is_link {
        let content = rsx! { Item { label, description, start_content: icon, end_content } };
        let class = format!("astryx-dropdown-menu-item--link {class}");
        let target = (target == DropdownMenuItemTarget::NewTab).then(|| "_blank".to_owned());
        let rel = if target.is_some() {
            let mut tokens: Vec<_> = rel
                .as_deref()
                .unwrap_or_default()
                .split_ascii_whitespace()
                .filter(|token| {
                    !["opener", "noopener", "noreferrer"]
                        .iter()
                        .any(|reserved| token.eq_ignore_ascii_case(reserved))
                })
                .collect();
            tokens.extend(["noopener", "noreferrer"]);
            Some(tokens.join(" "))
        } else {
            rel
        };
        let activate = move |event: MouseEvent| {
            if is_disabled {
                event.prevent_default();
            } else if event.modifiers().is_empty()
                && let Some(callback) = onclick
            {
                callback.call(event);
            }
        };
        if let Some(renderer) = renderer.filter(|_| href.is_some()) {
            attributes.extend([
                Attribute::new("class", class, None, false),
                Attribute::new("style", style, None, false),
                Attribute::new("draggable", "false", None, false),
            ]);
            return renderer.call(LinkRenderProps {
                href,
                target,
                rel,
                attributes,
                children: content,
                onclick: EventHandler::new(activate),
                onkeydown: EventHandler::default(),
                onmounted: EventHandler::default(),
            });
        }
        return rsx! { a { class, style, href, target, rel, draggable: "false", onclick: activate, ..attributes, {content} } };
    }
    attributes.push(Attribute::new(
        "onclick",
        AttributeValue::listener(move |event: Event<PlatformEventData>| {
            if is_disabled {
                event.prevent_default();
            } else if let Some(callback) = onclick {
                callback.call(event.map(|data| MouseData::from(data)));
            }
        }),
        None,
        false,
    ));
    rsx! {
        Item { label, description, start_content: icon, end_content, style, attributes,
            class,
        }
    }
}
