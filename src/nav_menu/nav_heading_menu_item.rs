use dioxus::html::onclick;
use dioxus::prelude::*;

use crate::link::{render_navigation_anchor_with_click, use_link_component};
use crate::utils::safe_url::is_safe_url;
use crate::{IconDefaultSizeProvider, IconSize};

use super::nav_heading_menu::NavHeadingMenuContext;

/// A navigation menu row. The surrounding menu owns keyboard navigation and dismissal.
#[component]
pub fn NavHeadingMenuItem(
    /// Primary content. Use noninteractive text or artwork.
    label: Element,
    /// Supporting content below the label.
    #[props(default)]
    description: Option<Element>,
    /// Leading artwork.
    #[props(default)]
    icon: Option<Element>,
    /// Destination for link rows.
    #[props(default)]
    href: Option<String>,
    /// Suppresses activation and removes any link destination.
    #[props(default)]
    is_disabled: bool,
    /// Activation callback.
    #[props(default)]
    on_select: Option<EventHandler<MouseEvent>>,
    /// Additional root classes.
    #[props(default)]
    class: String,
    /// Inline root styles.
    #[props(default)]
    style: Option<String>,
    /// Mounted callback for the row.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native root attributes. The menu item owns its role and tab index.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let renderer = use_link_component(None);
    let size = try_consume_context::<NavHeadingMenuContext>()
        .map(|context| (context.0)())
        .unwrap_or_default();
    let is_disabled = is_disabled || href.as_deref().is_some_and(|url| !is_safe_url(url));
    let activate = on_select.filter(|_| !is_disabled);
    let is_link = href.is_some();
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class"
                    | "style"
                    | "href"
                    | "role"
                    | "tabindex"
                    | "aria-disabled"
                    | "draggable"
                    | "data-size"
                    | "onclick"
                    | "onmounted"
            )
        })
        .collect();
    attributes.extend([
        Attribute::new(
            "class",
            format!("astryx-nav-heading-menu-item {class}"),
            None,
            false,
        ),
        Attribute::new("role", "menuitem", None, false),
        Attribute::new("tabindex", "-1", None, false),
        Attribute::new("data-size", size.as_str(), None, false),
    ]);
    if is_disabled {
        attributes.push(Attribute::new("aria-disabled", "true", None, false));
    }
    if is_link {
        attributes.push(Attribute::new("draggable", "false", None, false));
    }
    if let Some(style) = style {
        attributes.push(Attribute::new("style", style, None, false));
    }
    let content = rsx! {
        if let Some(icon) = icon {
            span { class: "astryx-nav-heading-menu-item__icon", IconDefaultSizeProvider { size: IconSize::Small, {icon} } }
        }
        span { class: "astryx-nav-heading-menu-item__content",
            span { class: "astryx-nav-heading-menu-item__label", {label} }
            if let Some(description) = description {
                span { class: "astryx-nav-heading-menu-item__description", {description} }
            }
        }
    };
    if let Some(href) = href.filter(|_| !is_disabled) {
        return render_navigation_anchor_with_click(
            href, renderer, attributes, content, onmounted, activate,
        );
    }
    if let Some(handler) = activate {
        attributes.push(onclick(move |event| handler.call(event)));
    }
    let mounted = move |event| {
        if let Some(handler) = onmounted {
            handler.call(event);
        }
    };
    if is_link {
        rsx! { a { onmounted: mounted, ..attributes, {content} } }
    } else {
        rsx! { div { onmounted: mounted, ..attributes, {content} } }
    }
}
