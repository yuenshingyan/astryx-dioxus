use dioxus::prelude::*;

use crate::link::{render_navigation_anchor, use_link_component};

/// Standard featured content for a top navigation mega menu. The parent menu
/// owns dismissal and focus return.
#[component]
pub fn TopNavMegaMenuFeaturedCard(
    /// Card title.
    title: String,
    /// Supporting text beneath the title.
    #[props(default)]
    description: Option<String>,
    /// Image URL above the body.
    #[props(default)]
    image: Option<String>,
    /// Alternative text for an informative image. Omit for decorative images.
    #[props(default)]
    image_alt: Option<String>,
    /// Link text. Both label and destination are required to show the link.
    #[props(default)]
    link_label: Option<String>,
    /// Link destination.
    #[props(default)]
    link_href: Option<String>,
    /// Additional content at the end of the body.
    #[props(default)]
    children: Element,
    /// Additional classes on the root.
    #[props(default)]
    class: String,
    /// Inline declarations on the root.
    #[props(default)]
    style: Option<String>,
    /// Native attributes on the root.
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let renderer = use_link_component(None);
    let image_alt = image_alt.filter(|alt| !alt.trim().is_empty());
    let is_decorative = image_alt.is_none();
    let cta = link_label.zip(link_href).filter(|(label, href)| !label.trim().is_empty() && !href.trim().is_empty()).map(|(label, href)| {
        render_navigation_anchor(href, renderer, vec![Attribute::new("class", "astryx-top-nav-mega-menu-featured-card__link", None, false)], rsx! {
            span { "{label}" }
            " "
            span { class: "astryx-top-nav-mega-menu-featured-card__arrow", "aria-hidden": "true", "→" }
        }, None)
    });
    rsx! {
        div { class: "astryx-top-nav-mega-menu-featured-card {class}", style, ..attributes,
            if let Some(image) = image.filter(|url| !url.trim().is_empty()) {
                img { class: "astryx-top-nav-mega-menu-featured-card__image", src: image, alt: image_alt.unwrap_or_default(),
                    role: is_decorative.then_some("presentation"), "aria-hidden": is_decorative.then_some("true"),
                }
            }
            div { class: "astryx-top-nav-mega-menu-featured-card__body",
                span { class: "astryx-top-nav-mega-menu-featured-card__title", "{title}" }
                if let Some(description) = description.filter(|text| !text.is_empty()) {
                    span { class: "astryx-top-nav-mega-menu-featured-card__description", "{description}" }
                }
                {cta}
                {children}
            }
        }
    }
}
