// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::events::onmounted as mounted_attribute;
use dioxus::prelude::*;

/// Visual axis and default accessible orientation of a Divider.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerOrientation {
    /// Full-width row with inline rule segments.
    #[default]
    Horizontal,
    /// Full-height column with block-axis rule segments.
    Vertical,
}

impl DividerOrientation {
    fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// An open Divider theme axis, not a closed list of global theme choices.
///
/// Unmatched custom names retain the original unpainted-rule behavior. The
/// upstream custom-paint/fallback API remains an open design decision; this
/// type does not invent a public part or silently select a different variant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DividerVariant(Cow<'static, str>);

impl DividerVariant {
    /// Default rule color, from the border token.
    pub const SUBTLE: Self = Self(Cow::Borrowed("subtle"));
    /// Emphasized border color, at the same rule thickness.
    pub const STRONG: Self = Self(Cow::Borrowed("strong"));

    /// Validate an extensible ASCII theme identifier without rewriting it.
    pub fn new(name: impl Into<String>) -> Result<Self, InvalidDividerVariant> {
        let name = name.into();
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(InvalidDividerVariant);
        }
        Ok(Self(Cow::Owned(name)))
    }

    /// The original name reflected on the aggregate theme target.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for DividerVariant {
    fn default() -> Self {
        Self::SUBTLE
    }
}

/// Divider theme names cannot be empty or contain unsupported characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidDividerVariant;
impl Display for InvalidDividerVariant {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Divider variant must be a nonempty identifier containing only ASCII letters, digits, hyphens, or underscores")
    }
}
impl Error for InvalidDividerVariant {}

/// Shallow optional label alternatives, preserving zero and opaque rich nodes.
#[derive(Clone, Default, PartialEq)]
pub enum DividerLabel {
    /// No label, matching absent/boolean/empty scalar content.
    #[default]
    Empty,
    /// Empty text is skipped; whitespace remains renderable.
    Text(String),
    /// A numeric label, including zero.
    Number(f64),
    /// Caller content is rendered without flattening or descendant inspection.
    Rich(Element),
}
impl From<&str> for DividerLabel {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}
impl From<String> for DividerLabel {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
impl From<f64> for DividerLabel {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl From<f32> for DividerLabel {
    fn from(value: f32) -> Self {
        Self::Number(f64::from(value))
    }
}
impl From<Element> for DividerLabel {
    fn from(value: Element) -> Self {
        Self::Rich(value)
    }
}
impl From<bool> for DividerLabel {
    fn from(_: bool) -> Self {
        Self::Empty
    }
}
macro_rules! integer_labels {
    ($($number:ty),*) => { $(impl From<$number> for DividerLabel {
        fn from(value: $number) -> Self { Self::Text(value.to_string()) }
    })* };
}
integer_labels!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

// Numeric display belongs to this label boundary, not another component's
// public label type. LowerExp retains shortest digits with the source thresholds.
fn number_text(value: f64) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value == 0.0 {
        return "0".to_string();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            "-Infinity"
        } else {
            "Infinity"
        }
        .to_string();
    }
    if (1e-6..1e21).contains(&value.abs()) {
        return value.to_string();
    }
    let scientific = format!("{value:e}");
    if value.abs() >= 1e21 {
        scientific.replace('e', "e+")
    } else {
        scientific
    }
}

fn text_attribute(attributes: &[Attribute], name: &str) -> Option<String> {
    attributes.iter().rev().find_map(|attribute| {
        if attribute.name != name {
            return None;
        }
        match &attribute.value {
            AttributeValue::Text(value) => Some(value.clone()),
            AttributeValue::Bool(value) => Some(value.to_string()),
            _ => None,
        }
    })
}

/// A named or unnamed semantic separator with logical full-bleed geometry.
///
/// Renderable labels split the rule in two and name the separator through a
/// stable aria-labelledby relationship. Explicit native aria-labelledby wins,
/// then aria-label. Caller role/aria-orientation override semantic defaults per
/// the pinned contract; visual orientation and theme metadata remain owned.
///
/// Full bleed consumes inherited --container-padding-* variables on its axis;
/// it does not publish container padding or create a new container abstraction.
/// Label strings/numbers/rich nodes are accepted; booleans and empty text omit
/// the label. No effect, measurement, listener or live region is installed.
/// `onmounted` adapts the root ref; native event builders also pass via attributes.
#[component]
pub fn Divider(
    #[props(default)] orientation: DividerOrientation,
    #[props(default, into)] label: DividerLabel,
    #[props(default)] variant: DividerVariant,
    #[props(default)] is_full_bleed: bool,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let label_id = use_hook(|| format!("astryx-divider-label-{}", current_scope_id().0));
    let label = match label {
        DividerLabel::Empty => None,
        DividerLabel::Text(text) if text.is_empty() => None,
        DividerLabel::Text(text) => Some(rsx! { "{text}" }),
        DividerLabel::Number(number) => Some(rsx! { "{number_text(number)}" }),
        DividerLabel::Rich(content) => Some(content),
    };
    let labelled_by = text_attribute(&attributes, "aria-labelledby").or_else(|| {
        (label.is_some() && text_attribute(&attributes, "aria-label").is_none())
            .then(|| label_id.clone())
    });
    let has_role = attributes.iter().any(|attribute| attribute.name == "role");
    let has_aria_orientation = attributes
        .iter()
        .any(|attribute| attribute.name == "aria-orientation");
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "class" | "style" | "data-variant" | "data-orientation" | "aria-labelledby"
            ) && (onmounted.is_none() || attribute.name != "onmounted")
        })
        .collect();
    if let Some(handler) = onmounted {
        attributes.push(mounted_attribute(move |event| handler.call(event)));
    }
    rsx! {
        div {
            class: if is_full_bleed { format!("astryx-divider astryx-divider--full-bleed {class}") } else { format!("astryx-divider {class}") },
            style,
            role: (!has_role).then_some("separator"),
            "aria-orientation": (!has_aria_orientation).then_some(orientation.as_str()),
            "aria-labelledby": labelled_by,
            "data-orientation": orientation.as_str(), "data-variant": variant.as_str(), ..attributes,
            div { class: "astryx-divider__line" }
            if let Some(label) = label {
                div { class: "astryx-divider__label", id: label_id, {label} }
                div { class: "astryx-divider__line" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::number_text;

    #[test]
    fn numeric_labels_preserve_scalar_presentation_and_zero() {
        for (number, expected) in [
            (0.0, "0"),
            (-0.0, "0"),
            (12.5, "12.5"),
            (1e-6, "0.000001"),
            (1e-7, "1e-7"),
            (1e20, "100000000000000000000"),
            (1e21, "1e+21"),
            (-1e21, "-1e+21"),
            (f64::INFINITY, "Infinity"),
            (f64::NEG_INFINITY, "-Infinity"),
            (f64::NAN, "NaN"),
        ] {
            assert_eq!(number_text(number), expected);
        }
    }
}
