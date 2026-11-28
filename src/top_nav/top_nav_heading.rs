//! Expanded, non-menu forms from Astryx TopNavHeading.tsx.

use dioxus::prelude::*;

use crate::link::{Link, LinkComponentType, render_navigation_anchor, use_link_component};
use crate::theme::{TextColor, TextSize, TextWeight};
use crate::utils::is_renderable::is_renderable;

/// Identifies a product or suite in the top navigation and optionally links to it.
/// The product name is navigation text, not a heading in the page outline.
#[component]
pub fn TopNavHeading(
    /// Product name. Without it, only the logo is shown.
    #[props(default)]
    heading: Option<String>,
    /// Product logo or other leading artwork. In linked configurations, use passive
    /// artwork here and place independent controls in `header_end_content`.
    #[props(default)]
    logo: Option<Element>,
    /// Name for a linked logo without adjacent text. Supply it for a logo-only link
    /// if the artwork does not already provide an accessible name.
    #[props(default)]
    logo_label: Option<String>,
    /// Destination for the product heading or logo.
    #[props(default)]
    heading_href: Option<String>,
    /// Text above the product name.
    #[props(default)]
    superheading: Option<String>,
    /// Independent destination for the text above the product name.
    #[props(default)]
    superheading_href: Option<String>,
    /// Text below the product name.
    #[props(default)]
    subheading: Option<String>,
    /// Independent destination for the text below the product name.
    #[props(default)]
    subheading_href: Option<String>,
    /// Trailing content when `heading` is present, kept outside any heading link.
    #[props(default)]
    header_end_content: Option<Element>,
    /// Overrides the provider renderer for whole-heading and logo links.
    #[props(default)]
    render_link: Option<LinkComponentType>,
    /// Additional root classes.
    #[props(default)]
    class: String,
    /// Inline declarations on the root.
    #[props(default)]
    style: String,
    /// Mounted callback for the root.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native root attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let renderer = use_link_component(render_link);
    let heading = heading.filter(|text| !text.is_empty());
    let logo_label = logo_label.filter(|label| !label.trim().is_empty());
    let heading_href = heading_href.filter(|href| !href.is_empty());
    let superheading = superheading.filter(|text| !text.is_empty());
    let subheading = subheading.filter(|text| !text.is_empty());
    // A destination on an omitted line must not split the visible product link.
    let superheading_href =
        superheading_href.filter(|href| superheading.is_some() && !href.is_empty());
    let subheading_href = subheading_href.filter(|href| subheading.is_some() && !href.is_empty());
    let has_independent_links = superheading_href.is_some() || subheading_href.is_some();
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style" | "href" | "onmounted"))
        .collect();
    attributes.push(Attribute::new(
        "class",
        format!("astryx-top-nav-heading {class}"),
        None,
        false,
    ));
    if !style.is_empty() {
        attributes.push(Attribute::new("style", style, None, false));
    }
    let logo = logo.filter(is_renderable);
    let Some(heading) = heading else {
        let content = rsx! { if let Some(logo) = logo { span { class: "astryx-top-nav-heading__logo", {logo} } } };
        if let Some(href) = heading_href {
            if let Some(label) =
                logo_label.filter(|_| !attributes.iter().any(|attr| attr.name == "aria-label"))
            {
                attributes.push(Attribute::new("aria-label", label, None, false));
            }
            return render_navigation_anchor(href, renderer, attributes, content, onmounted);
        }
        return rsx! { div { onmounted: move |event| { if let Some(handler) = onmounted { handler.call(event); } }, ..attributes, {content} } };
    };
    let logo = logo.map(|logo| {
        if let Some(href) = heading_href.clone().filter(|_| has_independent_links) {
            render_navigation_anchor(
                href,
                renderer,
                vec![
                    Attribute::new("class", "astryx-top-nav-heading__logo", None, false),
                    Attribute::new(
                        "aria-label",
                        logo_label.unwrap_or_else(|| heading.clone()),
                        None,
                        false,
                    ),
                ],
                logo,
                None,
            )
        } else {
            rsx! { span { class: "astryx-top-nav-heading__logo", {logo} } }
        }
    });
    let title_href = heading_href.clone().filter(|_| has_independent_links);
    let text_class = if has_independent_links {
        "astryx-top-nav-heading__text astryx-top-nav-heading__text--linked"
    } else {
        "astryx-top-nav-heading__text"
    };
    let content = rsx! {
        {logo}
        span { class: text_class,
            if let Some(text) = superheading {
                if let Some(href) = superheading_href {
                    Link { href, color: TextColor::SECONDARY, size: TextSize::ExtraSmall, "{text}" }
                } else { span { class: "astryx-top-nav-heading__secondary", "{text}" } }
            }
            if let Some(href) = title_href {
                Link { href, color: TextColor::PRIMARY, weight: TextWeight::Semibold, "{heading}" }
            } else { span { class: "astryx-top-nav-heading__title", "{heading}" } }
            if let Some(text) = subheading {
                if let Some(href) = subheading_href {
                    Link { href, color: TextColor::SECONDARY, size: TextSize::ExtraSmall, "{text}" }
                } else { span { class: "astryx-top-nav-heading__secondary", "{text}" } }
            }
        }
    };
    let end = header_end_content.filter(is_renderable);
    let content = if let Some(href) = heading_href.filter(|_| !has_independent_links) {
        if end.is_none() {
            return render_navigation_anchor(href, renderer, attributes, content, onmounted);
        }
        render_navigation_anchor(
            href,
            renderer,
            vec![Attribute::new(
                "class",
                "astryx-top-nav-heading__link",
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
            if let Some(end) = end { span { class: "astryx-top-nav-heading__end", {end} } }
        }
    }
}
