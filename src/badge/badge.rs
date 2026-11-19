// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};

use dioxus::events::onmounted as mounted_attribute;
use dioxus::prelude::*;

use crate::utils::text_direction::text_direction;

/// Theme-extensible badge tone. Built-in constants preserve the original names.
///
/// Custom variants inherit text color over a transparent background until an
/// application theme paints their `data-variant` selector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BadgeVariant(Cow<'static, str>);

impl BadgeVariant {
    /// Default neutral fill and primary text.
    pub const NEUTRAL: Self = Self(Cow::Borrowed("neutral"));
    /// Solid information tone.
    pub const INFO: Self = Self(Cow::Borrowed("info"));
    /// Solid success tone.
    pub const SUCCESS: Self = Self(Cow::Borrowed("success"));
    /// Solid warning tone.
    pub const WARNING: Self = Self(Cow::Borrowed("warning"));
    /// Solid error tone.
    pub const ERROR: Self = Self(Cow::Borrowed("error"));
    /// Blue category tint.
    pub const BLUE: Self = Self(Cow::Borrowed("blue"));
    /// Cyan category tint.
    pub const CYAN: Self = Self(Cow::Borrowed("cyan"));
    /// Green category tint.
    pub const GREEN: Self = Self(Cow::Borrowed("green"));
    /// Orange category tint.
    pub const ORANGE: Self = Self(Cow::Borrowed("orange"));
    /// Pink category tint.
    pub const PINK: Self = Self(Cow::Borrowed("pink"));
    /// Purple category tint.
    pub const PURPLE: Self = Self(Cow::Borrowed("purple"));
    /// Red category tint.
    pub const RED: Self = Self(Cow::Borrowed("red"));
    /// Teal category tint.
    pub const TEAL: Self = Self(Cow::Borrowed("teal"));
    /// Yellow category tint.
    pub const YELLOW: Self = Self(Cow::Borrowed("yellow"));

    /// Validate a custom identifier without silently trimming or changing it.
    pub fn new(name: impl Into<String>) -> Result<Self, InvalidBadgeVariant> {
        let name = name.into();
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(InvalidBadgeVariant);
        }
        Ok(Self(Cow::Owned(name)))
    }

    /// Original built-in or validated custom target value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for BadgeVariant {
    fn default() -> Self {
        Self::NEUTRAL
    }
}

/// The badge theme identifier was empty or contained unsupported characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidBadgeVariant;

impl Display for InvalidBadgeVariant {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Badge variant must be a nonempty identifier containing only ASCII letters, digits, hyphens, or underscores")
    }
}
impl Error for InvalidBadgeVariant {}

/// Visible label alternatives. Rich nodes keep their own direction/semantics.
///
/// String and numeric conversions are accepted by the `label` builder. Integers
/// become exact decimal text without lossy conversion through a floating point.
/// `Empty` represents absent/boolean React content without a label-height floor.
#[derive(Clone, PartialEq)]
pub enum BadgeLabel {
    /// Plain text supplies a native title and inferred direction.
    Text(String),
    /// JavaScript-style floating-point number presentation, including zero.
    Number(f64),
    /// Opaque caller content; no inferred title or direction.
    Rich(Element),
    /// No visible scalar content, matching absent or boolean React nodes.
    Empty,
}

impl From<&str> for BadgeLabel {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}
impl From<String> for BadgeLabel {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
impl From<f64> for BadgeLabel {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl From<f32> for BadgeLabel {
    fn from(value: f32) -> Self {
        Self::Number(f64::from(value))
    }
}
impl From<Element> for BadgeLabel {
    fn from(value: Element) -> Self {
        Self::Rich(value)
    }
}
impl From<bool> for BadgeLabel {
    fn from(_: bool) -> Self {
        Self::Empty
    }
}

macro_rules! integer_labels {
    ($($number:ty),*) => { $(impl From<$number> for BadgeLabel {
        fn from(value: $number) -> Self { Self::Text(value.to_string()) }
    })* };
}
integer_labels!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

// Match String(number)'s zero/non-finite and scientific-notation boundaries.
// Rust's shortest round-trip digits stay intact; only their presentation changes.
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
    let raw = value.abs().to_string();
    let point = raw.find('.').unwrap_or(raw.len());
    let mut digits: String = raw.chars().filter(|character| *character != '.').collect();
    let leading = digits.bytes().take_while(|byte| *byte == b'0').count();
    let exponent = point as i32 - leading as i32 - 1;
    digits.drain(..leading);
    while digits.ends_with('0') {
        digits.pop();
    }
    let sign = if value.is_sign_negative() { "-" } else { "" };
    if !(-6..21).contains(&exponent) {
        let tail = if digits.len() > 1 {
            format!(".{}", &digits[1..])
        } else {
            String::new()
        };
        return format!(
            "{sign}{}{tail}e{}{exponent}",
            &digits[..1],
            if exponent >= 0 { "+" } else { "" }
        );
    }
    if exponent < 0 {
        return format!("{sign}0.{}{digits}", "0".repeat((-exponent - 1) as usize));
    }
    let point = exponent as usize + 1;
    if digits.len() <= point {
        return format!("{sign}{digits}{}", "0".repeat(point - digits.len()));
    }
    format!("{sign}{}.{}", &digits[..point], &digits[point..])
}

/// Read-only, one-line pill with native full-text titles for scalar labels.
///
/// Badge adds no role, tab stop, live region, tooltip controller, measurement,
/// or effect. The optional icon renders directly before the isolated label;
/// its size, artwork, and accessibility remain caller-owned. Native attributes
/// and event builders pass through `attributes`; `onmounted` adapts the root ref.
/// Required `label` accepts plain text, numbers, rich nodes, or explicit Empty.
///
/// ```compile_fail
/// use astryx_dioxus::Badge;
/// use dioxus::prelude::*;
/// let _ = rsx! { Badge {} };
/// ```
#[component]
pub fn Badge(
    #[props(into)] label: BadgeLabel,
    #[props(default = BadgeVariant::NEUTRAL)] variant: BadgeVariant,
    #[props(default)] icon: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onmounted: Option<EventHandler<MountedEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let (content, scalar) = match label {
        BadgeLabel::Text(text) => (None, Some(text)),
        BadgeLabel::Number(number) => (None, Some(number_text(number))),
        BadgeLabel::Rich(content) => (Some(content), None),
        BadgeLabel::Empty => (None, None),
    };
    let has_label = content.is_some() || scalar.as_ref().is_some_and(|text| !text.is_empty());
    let title = if attributes.iter().any(|attribute| attribute.name == "title") {
        None
    } else {
        scalar.as_ref().filter(|text| !text.is_empty()).cloned()
    };
    let direction = scalar.as_deref().and_then(text_direction);
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(attribute.name, "class" | "style" | "data-variant")
                && (onmounted.is_none() || attribute.name != "onmounted")
        })
        .collect();
    if let Some(handler) = onmounted {
        attributes.push(mounted_attribute(move |event| handler.call(event)));
    }
    // Default is supplied at the prop boundary; custom values get no built-in
    // paint. In particular, never collapse a custom variant into neutral.
    rsx! {
        span {
            class: if has_label { format!("astryx-badge astryx-badge--labelled {class}") } else { format!("astryx-badge {class}") },
            style, "data-variant": variant.as_str(), title, ..attributes,
            {icon}
            span { class: "astryx-badge__label", dir: direction,
                if let Some(text) = scalar { "{text}" }
                {content}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::number_text;

    #[test]
    fn numeric_labels_keep_javascript_scalar_presentation_boundaries() {
        for (number, expected) in [
            (0.0, "0"),
            (-0.0, "0"),
            (12.5, "12.5"),
            (-12.5, "-12.5"),
            (1e-6, "0.000001"),
            (1e-7, "1e-7"),
            (1e20, "100000000000000000000"),
            (1e21, "1e+21"),
            (f64::INFINITY, "Infinity"),
            (f64::NEG_INFINITY, "-Infinity"),
            (f64::NAN, "NaN"),
            (f64::MIN_POSITIVE, "2.2250738585072014e-308"),
        ] {
            assert_eq!(number_text(number), expected);
        }
    }
}
