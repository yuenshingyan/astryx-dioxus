use dioxus::core::AttributeValue;
use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;
use crate::{ElementSize, SizeProvider};

/// Expanded navigation rail with independently scrolling items.
#[component]
pub fn SideNav(
    children: Element,
    /// Landmark label used when no native `aria-label` is supplied.
    #[props(default = "Side navigation".to_owned())]
    label: String,
    #[props(default)] header: Option<Element>,
    #[props(default)] top_content: Option<Element>,
    #[props(default)] footer: Option<Element>,
    #[props(default)] footer_icons: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let header = header.filter(is_renderable);
    let top_content = top_content.filter(is_renderable);
    let footer = footer.filter(is_renderable);
    let footer_icons = footer_icons.filter(is_renderable);
    let has_top = header.is_some() || top_content.is_some();
    let has_bottom = footer.is_some() || footer_icons.is_some();
    let mut aria_label = AttributeValue::Text(label);
    let attributes = attributes
        .into_iter()
        .filter_map(|attr| match attr.name {
            "class" | "style" | "role" => None,
            "aria-label" => {
                // Upstream spreads native naming attributes after the default.
                // Emit one label so SSR and the client use the same precedence.
                if !matches!(attr.value, AttributeValue::None) {
                    aria_label = attr.value;
                }
                None
            }
            _ => Some(attr),
        })
        .collect::<Vec<_>>();
    rsx! {
        nav { class: format!("astryx-side-nav {class}"), style, "aria-label": aria_label, ..attributes,
            if has_top {
                div { class: "astryx-side-nav__top",
                    {header}
                    if let Some(top_content) = top_content { div { {top_content} } }
                }
            }
            div { class: "astryx-side-nav__scrollable", "data-has-top": has_top.then_some("true"), "data-has-bottom": has_bottom.then_some("true"), {children} }
            if has_bottom {
                div { class: "astryx-side-nav__bottom",
                    {footer}
                    if let Some(icons) = footer_icons {
                        div { class: "astryx-side-nav__footer-row", SizeProvider { size: ElementSize::Small, {icons} } }
                    }
                }
            }
        }
    }
}
