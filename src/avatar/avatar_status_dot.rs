//! Status plate and glyphs adapted from Astryx Avatar/AvatarStatusDot.tsx.

use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};

use dioxus::prelude::*;

use super::avatar_size_context::AvatarSizeContext;

/// Open theme identifier for the status plate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvatarStatusDotVariant(Cow<'static, str>);

impl AvatarStatusDotVariant {
    /// Filled success plate.
    pub const SUCCESS: Self = Self(Cow::Borrowed("success"));
    /// Neutral ring.
    pub const NEUTRAL: Self = Self(Cow::Borrowed("neutral"));
    /// Error plate with a minus mark.
    pub const ERROR: Self = Self(Cow::Borrowed("error"));

    /// Validate a custom theme identifier without changing it.
    pub fn new(name: impl Into<String>) -> Result<Self, InvalidAvatarStatusDotVariant> {
        let name = name.into();
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(InvalidAvatarStatusDotVariant);
        }
        Ok(Self(Cow::Owned(name)))
    }

    /// Reflected theme identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for AvatarStatusDotVariant {
    fn default() -> Self {
        Self::SUCCESS
    }
}

/// The variant was empty or contained unsupported characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidAvatarStatusDotVariant;

impl Display for InvalidAvatarStatusDotVariant {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Avatar status variant must be a nonempty identifier containing only ASCII letters, digits, hyphens, or underscores")
    }
}

impl Error for InvalidAvatarStatusDotVariant {}

#[derive(Clone, Copy)]
struct Geometry {
    diameter: u32,
    border: u32,
    icon: u32,
    stroke: f64,
}

impl Geometry {
    fn resolve(size: u32) -> Self {
        match size {
            0..=36 => Self {
                diameter: 10,
                border: 1,
                icon: 0,
                stroke: 1.0,
            },
            37..=72 => Self {
                diameter: 20,
                border: 2,
                icon: 12,
                stroke: 1.5,
            },
            _ => Self {
                diameter: 32,
                border: 4,
                icon: 18,
                stroke: 2.0,
            },
        }
    }
}

fn glyph(variant: &AvatarStatusDotVariant, geometry: Geometry) -> Element {
    let shape = match variant.as_str() {
        "neutral" => "ring",
        "error" => "minus",
        _ => return rsx! {},
    };
    let field = geometry.diameter - 2 * geometry.border;
    let center = f64::from(field) / 2.0;
    let radius = center - geometry.stroke / 2.0;
    let start = center * 0.25 + geometry.stroke / 2.0;
    let end = f64::from(field) - start;
    rsx! {
        svg { class: "astryx-avatar-status-dot-glyph", "data-shape": shape, "aria-hidden": "true",
            view_box: "0 0 {field} {field}", width: "{field}", height: "{field}", fill: "none",
            if shape == "ring" {
                circle { cx: "{center}", cy: "{center}", r: "{radius}", stroke: "currentColor", stroke_width: "{geometry.stroke}" }
            } else {
                line { x1: "{start}", y1: "{center}", x2: "{end}", y2: "{center}", stroke: "currentColor", stroke_width: "{geometry.stroke}", stroke_linecap: "round" }
            }
        }
    }
}

/// Compact status plate. A standalone dot uses the smallest avatar tier.
#[component]
pub fn AvatarStatusDot(
    /// Status paint and built-in glyph.
    #[props(default)]
    variant: AvatarStatusDotVariant,
    /// Accessible name. Unnamed dots add no role or name.
    #[props(default)]
    label: Option<String>,
    /// Artwork replacing the built-in glyph only when space permits.
    #[props(default)]
    icon: Option<Element>,
    /// Caller classes.
    #[props(default)]
    class: String,
    /// Caller inline CSS.
    #[props(default)]
    style: String,
    /// Native root attributes and callbacks, including `onmounted`.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let geometry = Geometry::resolve(
        try_consume_context::<AvatarSizeContext>()
            .unwrap_or_default()
            .0,
    );
    let label = label.filter(|name| !name.is_empty());
    let is_named = label.is_some();
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(attribute.name, "style" | "class" | "data-variant")
                && !(is_named && matches!(attribute.name, "role" | "aria-label"))
        })
        .collect();
    let artwork = match icon.filter(|_| geometry.icon > 0) {
        Some(icon) => {
            rsx! { span { class: "astryx-avatar-status-dot-icon", "aria-hidden": "true", style: "width:{geometry.icon}px;height:{geometry.icon}px", {icon} } }
        }
        None => glyph(&variant, geometry),
    };
    rsx! {
        div { class: "astryx-avatar-status-dot {class}", "data-variant": variant.as_str(),
            style: "--_avatar-dot-size:{geometry.diameter}px;--_avatar-dot-border:{geometry.border}px;{style}",
            role: is_named.then_some("img"), "aria-label": label,
            ..attributes,
            {artwork}
        }
    }
}

#[cfg(test)]
mod tests {
    use dioxus::prelude::*;
    use dioxus_ssr::render_element;

    use super::{AvatarSizeContext, AvatarStatusDot, AvatarStatusDotVariant};
    use crate::layer::LayerContentBoundary;

    #[test]
    fn avatar_size_selects_glyph_or_icon_until_a_layer_boundary() {
        #[component]
        fn Parent(size: u32, children: Element) -> Element {
            use_context_provider(|| AvatarSizeContext(size));
            children
        }
        let html = render_element(rsx! {
            Parent { size: 72,
                AvatarStatusDot { variant: AvatarStatusDotVariant::NEUTRAL, icon: rsx! { svg { id: "replacement" } } }
                LayerContentBoundary { AvatarStatusDot { variant: AvatarStatusDotVariant::NEUTRAL, icon: rsx! { svg { id: "ignored" } } } }
            }
            Parent { size: 96, AvatarStatusDot { variant: AvatarStatusDotVariant::ERROR } }
        });
        for expected in [
            "--_avatar-dot-size:20px;--_avatar-dot-border:2px",
            "width:12px;height:12px",
            "id=\"replacement\"",
            "--_avatar-dot-size:10px",
            "data-shape=\"ring\"",
            "--_avatar-dot-size:32px;--_avatar-dot-border:4px",
            "data-shape=\"minus\"",
        ] {
            assert!(html.contains(expected), "{html}");
        }
        assert!(!html.contains("ignored"));
    }
}
