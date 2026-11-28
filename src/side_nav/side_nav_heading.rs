//! Non-menu SideNavHeading forms adapted from Astryx SideNavHeading.tsx.

use dioxus::prelude::*;

use crate::link::{Link, LinkComponentType, render_navigation_anchor, use_link_component};
use crate::theme::{TextColor, TextSize, TextWeight};
use crate::utils::is_renderable::is_renderable;

/// Identifies a product, suite, or account in the side navigation header.
///
/// Like the upstream identity block, the title is not a document-outline heading.
#[component]
pub fn SideNavHeading(
    /// Name of the product or application.
    heading: String,
    /// Leading noninteractive product artwork. Use `header_end_content` for controls.
    #[props(default)]
    icon: Option<Element>,
    /// Destination for the product heading.
    #[props(default)]
    heading_href: Option<String>,
    /// Text above the product heading.
    #[props(default)]
    superheading: Option<String>,
    /// Independent destination for the text above the heading.
    #[props(default)]
    superheading_href: Option<String>,
    /// Text below the product heading.
    #[props(default)]
    subheading: Option<String>,
    /// Independent destination for the text below the heading.
    #[props(default)]
    subheading_href: Option<String>,
    /// Trailing content, kept outside any link.
    #[props(default)]
    header_end_content: Option<Element>,
    /// Overrides the provider renderer for whole-heading links and leading icon links.
    #[props(default)]
    render_link: Option<LinkComponentType>,
    /// Classes on the root.
    #[props(default)]
    class: String,
    /// Inline declarations on the root.
    #[props(default)]
    style: String,
    /// Mounted callback on the root.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native attributes on the root.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let renderer = use_link_component(render_link);
    // Upstream treats empty hrefs as absent. Leave nonempty values untouched for
    // the shared destination policy and the caller's router.
    let heading_href = heading_href.filter(|href| !href.is_empty());
    let superheading_href = superheading_href.filter(|href| !href.is_empty());
    let subheading_href = subheading_href.filter(|href| !href.is_empty());
    let is_whole_link =
        heading_href.is_some() && superheading_href.is_none() && subheading_href.is_none();
    let end = header_end_content.filter(is_renderable);
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style" | "href" | "onmounted"))
        .collect();
    attributes.push(Attribute::new(
        "class",
        format!("astryx-side-nav-heading {class}"),
        None,
        false,
    ));
    if !style.is_empty() {
        attributes.push(Attribute::new("style", style, None, false));
    }
    let icon = icon.filter(is_renderable).map(|icon| {
        if let Some(href) = heading_href.clone().filter(|_| !is_whole_link) {
            render_navigation_anchor(
                href,
                renderer,
                vec![
                    Attribute::new("class", "astryx-side-nav-heading__icon", None, false),
                    Attribute::new("aria-label", heading.clone(), None, false),
                ],
                icon,
                None,
            )
        } else {
            rsx! { span { class: "astryx-side-nav-heading__icon", {icon} } }
        }
    });
    let title_href = heading_href.clone().filter(|_| !is_whole_link);
    let content = rsx! {
        {icon}
        span { class: "astryx-side-nav-heading__text",
            if let Some(text) = superheading.filter(|text| !text.is_empty()) {
                if let Some(href) = superheading_href {
                    Link { href, color: TextColor::SECONDARY, size: TextSize::ExtraSmall, "{text}" }
                } else { span { class: "astryx-side-nav-heading__secondary", "{text}" } }
            }
            if let Some(href) = title_href {
                Link { href, color: TextColor::PRIMARY, weight: TextWeight::Semibold, "{heading}" }
            } else { span { class: "astryx-side-nav-heading__title", "{heading}" } }
            if let Some(text) = subheading.filter(|text| !text.is_empty()) {
                if let Some(href) = subheading_href {
                    Link { href, color: TextColor::SECONDARY, size: TextSize::ExtraSmall, "{text}" }
                } else { span { class: "astryx-side-nav-heading__secondary", "{text}" } }
            }
        }
    };
    let content = if let Some(href) = heading_href.filter(|_| is_whole_link) {
        if end.is_none() {
            return render_navigation_anchor(href, renderer, attributes, content, onmounted);
        }
        render_navigation_anchor(
            href,
            renderer,
            vec![Attribute::new(
                "class",
                "astryx-side-nav-heading__link",
                None,
                false,
            )],
            content,
            None,
        )
    } else {
        content
    };
    rsx! {
        div { onmounted: move |event| { if let Some(handler) = onmounted { handler.call(event); } }, ..attributes,
            {content}
            if let Some(end) = end { span { class: "astryx-side-nav-heading__end", {end} } }
        }
    }
}
