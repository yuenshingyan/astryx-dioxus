//! Navigation trail adapted from Astryx Breadcrumbs/Breadcrumbs.tsx.

use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};

use dioxus::core::AttributeValue;
use dioxus::prelude::*;

/// Open theme identifier for the appearance of a breadcrumb trail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreadcrumbsVariant(Cow<'static, str>);

impl BreadcrumbsVariant {
    /// Standard body typography.
    pub const DEFAULT: Self = Self(Cow::Borrowed("default"));
    /// Smaller supporting typography.
    pub const SUPPORTING: Self = Self(Cow::Borrowed("supporting"));

    /// Validate a custom theme identifier without changing its spelling.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidBreadcrumbsVariant> {
        let value = value.into();
        if value.is_empty()
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(InvalidBreadcrumbsVariant);
        }
        Ok(Self(Cow::Owned(value)))
    }

    /// Identifier reflected on the trail and its items.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for BreadcrumbsVariant {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Invalid breadcrumb theme identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidBreadcrumbsVariant;

impl Display for InvalidBreadcrumbsVariant {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Breadcrumbs variant must be a nonempty ASCII identifier containing only letters, digits, hyphens, or underscores")
    }
}

impl Error for InvalidBreadcrumbsVariant {}

#[derive(Clone, Default, PartialEq)]
pub(crate) struct BreadcrumbSettings {
    pub(crate) variant: BreadcrumbsVariant,
    pub(crate) separator: Option<Element>,
}

#[derive(Clone, Copy)]
pub(crate) struct BreadcrumbContext(pub(crate) Signal<BreadcrumbSettings>);

/// A named navigation landmark with a wrapping breadcrumb trail.
#[component]
pub fn Breadcrumbs(
    /// BreadcrumbItem children, in route order.
    children: Element,
    /// Decorative separator; the default slash mirrors in RTL.
    #[props(default)]
    separator: Option<Element>,
    /// Trail appearance, inherited by its items within the same layer.
    #[props(default)]
    variant: BreadcrumbsVariant,
    /// Localizable landmark label, overridden by native ARIA naming attributes.
    #[props(default = "Breadcrumb".into())]
    label: String,
    /// Caller classes on the navigation landmark.
    #[props(default)]
    class: String,
    /// Caller inline declarations on the navigation landmark.
    #[props(default)]
    style: String,
    /// Native attributes, events and mounted callback on the landmark.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let settings = BreadcrumbSettings { variant, separator };
    let mut context = use_context_provider(|| BreadcrumbContext(Signal::new(settings.clone())));
    if *context.0.peek() != settings {
        context.0.set(settings.clone());
    }
    let has_name = attributes.iter().any(|attr| {
        matches!(attr.name, "aria-label" | "aria-labelledby")
            && !matches!(attr.value, AttributeValue::None)
    });
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(attr.name, "class" | "style" | "data-variant")
                && !(matches!(attr.name, "aria-label" | "aria-labelledby")
                    && matches!(attr.value, AttributeValue::None))
        })
        .collect();
    rsx! {
        nav { class: "astryx-breadcrumbs {class}", style: (!style.is_empty()).then_some(style),
            "data-variant": settings.variant.as_str(), "aria-label": (!has_name).then_some(label), ..attributes,
            ol { class: "astryx-breadcrumbs__list", {children} }
        }
    }
}
