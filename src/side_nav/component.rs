use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;
use crate::{ElementSize, SizeProvider};

/// Expanded navigation rail with independently scrolling items.
#[component]
pub fn SideNav(
    children: Element,
    #[props(default = "Side navigation".to_owned())] label: String,
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
    let attributes = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style" | "role" | "aria-label"))
        .collect::<Vec<_>>();
    rsx! {
        nav { class: format!("astryx-side-nav {class}"), style, "aria-label": label, ..attributes,
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
