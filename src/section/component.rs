//! Two-box Section container, adapted from Astryx Section.tsx.

use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter, Write};

use dioxus::prelude::*;

use crate::layout::padding::spacing_step_css;
use crate::utils::types::{SizeValue, SpacingStep};

/// Built-in or theme-defined Section surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SectionVariant(Cow<'static, str>);

impl SectionVariant {
    /// The theme's surface background.
    pub const SECTION: Self = Self(Cow::Borrowed("section"));
    /// No background.
    pub const TRANSPARENT: Self = Self(Cow::Borrowed("transparent"));
    /// The theme's muted background.
    pub const MUTED: Self = Self(Cow::Borrowed("muted"));

    /// Validate and preserve a custom theme identifier.
    pub fn new(name: impl Into<String>) -> Result<Self, InvalidSectionVariant> {
        let name = name.into();
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(InvalidSectionVariant);
        }
        Ok(Self(Cow::Owned(name)))
    }

    /// Identifier reflected on the inner surface.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SectionVariant {
    fn default() -> Self {
        Self::SECTION
    }
}

/// The Section theme identifier was empty or contained unsupported characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidSectionVariant;

impl Display for InvalidSectionVariant {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Section variant must be a nonempty identifier containing only ASCII letters, digits, hyphens, or underscores")
    }
}

impl Error for InvalidSectionVariant {}

/// Edge on which Section paints a one-pixel divider.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionDivider {
    /// Physical top.
    Top,
    /// Physical bottom.
    Bottom,
    /// Logical inline start.
    Start,
    /// Logical inline end.
    End,
}

/// Themeable layout region that can bleed into its parent's published padding.
#[component]
pub fn Section(
    /// Background of the inner surface.
    #[props(default)]
    variant: SectionVariant,
    /// Outer width.
    #[props(default, into)]
    width: Option<SizeValue>,
    /// Outer height.
    #[props(default, into)]
    height: Option<SizeValue>,
    /// Outer maximum width.
    #[props(default, into)]
    max_width: Option<SizeValue>,
    /// Outer minimum height.
    #[props(default, into)]
    min_height: Option<SizeValue>,
    /// Selected divider edges; duplicates have no effect.
    #[props(default)]
    dividers: Vec<SectionDivider>,
    /// Uniform padding that also supplies nested Section defaults.
    #[props(default)]
    padding: Option<SpacingStep>,
    /// Override padding on the inline axis.
    #[props(default)]
    padding_inline: Option<SpacingStep>,
    /// Override inline-start padding.
    #[props(default)]
    padding_inline_start: Option<SpacingStep>,
    /// Override inline-end padding.
    #[props(default)]
    padding_inline_end: Option<SpacingStep>,
    /// Override padding on the block axis.
    #[props(default)]
    padding_block: Option<SpacingStep>,
    /// Override block-start padding.
    #[props(default)]
    padding_block_start: Option<SpacingStep>,
    /// Override block-end padding.
    #[props(default)]
    padding_block_end: Option<SpacingStep>,
    /// Caller class on the outer wrapper.
    #[props(default)]
    class: String,
    /// Caller inline style on the outer wrapper.
    #[props(default)]
    style: String,
    /// Content inside the painted region.
    #[props(default = rsx! {})]
    children: Element,
    /// Native outer-wrapper attributes, events and mounted callback.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let mut outer_style = String::new();
    for (property, value) in [
        ("width", width),
        ("height", height),
        ("max-width", max_width),
        ("min-height", min_height),
    ] {
        if let Some(value) = value {
            let _ = write!(outer_style, "--_section-{property}:{};", value.as_css()?);
        }
    }
    outer_style.push_str(&style);
    let mut inner_style = String::new();
    if let Some(padding) = padding {
        let _ = write!(
            inner_style,
            "--_section-padding-propagated:{};",
            spacing_step_css(padding)
        );
    }
    for (edge, value) in [
        (
            "inline-start",
            padding_inline_start.or(padding_inline).or(padding),
        ),
        (
            "inline-end",
            padding_inline_end.or(padding_inline).or(padding),
        ),
        (
            "block-start",
            padding_block_start.or(padding_block).or(padding),
        ),
        ("block-end", padding_block_end.or(padding_block).or(padding)),
    ] {
        if let Some(value) = value {
            let _ = write!(
                inner_style,
                "--container-padding-{edge}:{};",
                spacing_step_css(value)
            );
        }
    }
    let mut classes = String::from("astryx-section");
    for (side, class) in [
        (SectionDivider::Top, "top"),
        (SectionDivider::Bottom, "bottom"),
        (SectionDivider::Start, "start"),
        (SectionDivider::End, "end"),
    ] {
        if dividers.contains(&side) {
            let _ = write!(classes, " astryx-section--divider-{class}");
        }
    }
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| !matches!(attribute.name, "class" | "style"))
        .collect();
    rsx! {
        div { class: "astryx-section-wrapper {class}", style: (!outer_style.is_empty()).then_some(outer_style), ..attributes,
            div { class: classes, "data-variant": variant.as_str(), style: (!inner_style.is_empty()).then_some(inner_style), {children} }
        }
    }
}
