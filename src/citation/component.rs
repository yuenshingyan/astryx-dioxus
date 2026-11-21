use dioxus::prelude::*;

use crate::icon::{IconDefaultSizeProvider, IconSize};
use crate::utils::safe_url::is_safe_url;

/// Source metadata for an inline citation.
#[derive(Clone, Default, PartialEq)]
pub struct CitationSource {
    /// Visible title, falling back to the reference number.
    pub title: Option<String>,
    /// Destination, if the citation navigates.
    pub url: Option<String>,
    /// Decorative source image.
    pub src: Option<String>,
    /// Decorative artwork; takes precedence over `src`.
    pub icon: Option<Element>,
}

/// Inline rendering style.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CitationVariant {
    /// Source title with optional artwork.
    #[default]
    Label,
    /// Superscript reference number.
    Number,
}

/// Inline source reference that links only when its destination is valid.
#[component]
pub fn Citation(
    source: CitationSource,
    number: u32,
    #[props(default)] variant: CitationVariant,
    /// Overrides the generated English accessible name.
    #[props(default)]
    accessible_label: Option<String>,
    #[props(default)] class: String,
    #[props(default)] style: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let title = source.title.unwrap_or_else(|| number.to_string());
    let href = source.url.filter(|url| !url.is_empty() && is_safe_url(url));
    let name = accessible_label.unwrap_or_else(|| format!("Citation {number}: {title}"));
    let has_icon =
        variant == CitationVariant::Label && (source.icon.is_some() || source.src.is_some());
    let variant_name = match variant {
        CitationVariant::Label => "label",
        CitationVariant::Number => "number",
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "role"
                    | "aria-label"
                    | "aria-labelledby"
                    | "title"
                    | "href"
                    | "target"
                    | "rel"
                    | "data-variant"
                    | "data-has-icon"
            )
        })
        .collect();
    let content = rsx! {
        if variant == CitationVariant::Number { "{number}" } else {
            if has_icon {
                span { class: "astryx-citation__icon", "aria-hidden": "true",
                    if let Some(icon) = source.icon { IconDefaultSizeProvider { size: IconSize::Small, {icon} } }
                    else if let Some(src) = source.src { img { src, alt: "" } }
                }
            }
            span { class: "astryx-citation__text", "{title}" }
        }
    };
    let class = if class.is_empty() {
        "astryx-citation".to_owned()
    } else {
        format!("astryx-citation {class}")
    };
    let has_icon = has_icon.then_some("true");
    if let Some(href) = href {
        rsx! { a { class, style, title, href, target: "_blank", rel: "noopener noreferrer", role: "doc-noteref",
            "aria-label": name, "data-variant": variant_name, "data-has-icon": has_icon, ..attributes, {content}
        } }
    } else {
        rsx! { span { class, style, title, role: "group",
            "aria-label": name, "data-variant": variant_name, "data-has-icon": has_icon, ..attributes, {content}
        } }
    }
}
