// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md. Custom fallback and universal forced-color outlines
// are approved corrections to the pinned source's documented design gaps.

use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;

use super::use_status_dot_tooltip::use_status_dot_tooltip;

/// Theme-extensible status tone. A custom name retains its target value and has
/// neutral plate/ink until a theme paints it. There is no implicit variant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusDotVariant(Cow<'static, str>);

impl StatusDotVariant {
    /// Success icon-role plate with surface ink.
    pub const SUCCESS: Self = Self(Cow::Borrowed("success"));
    /// Warning icon-role plate with surface ink.
    pub const WARNING: Self = Self(Cow::Borrowed("warning"));
    /// Error icon-role plate with surface ink.
    pub const ERROR: Self = Self(Cow::Borrowed("error"));
    /// Accent plate and on-accent ink.
    pub const ACCENT: Self = Self(Cow::Borrowed("accent"));
    /// Secondary icon-role plate with surface ink.
    pub const NEUTRAL: Self = Self(Cow::Borrowed("neutral"));

    /// Validate without trimming, rewriting, or silently replacing a name.
    pub fn new(name: impl Into<String>) -> Result<Self, InvalidStatusDotVariant> {
        let name = name.into();
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(InvalidStatusDotVariant);
        }
        Ok(Self(Cow::Owned(name)))
    }

    /// Original built-in or validated custom target value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A status theme identifier was empty or contained unsupported characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidStatusDotVariant;

impl Display for InvalidStatusDotVariant {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("StatusDot variant must be a nonempty identifier containing only ASCII letters, digits, hyphens, or underscores")
    }
}
impl Error for InvalidStatusDotVariant {}

/// Caller-owned icon content. Booleans, absent options and empty scalar strings
/// create no wrapper. Opaque elements/fragments retain their own renderability,
/// matching a React element whose children happen to be empty.
#[derive(Clone, Default, PartialEq)]
pub enum StatusDotIcon {
    /// No inner mark.
    #[default]
    Empty,
    /// Scalar text; nonempty values render inside the hidden icon field.
    Text(String),
    /// Caller SVG/component/fragment. Artwork, size and currentColor are caller-owned.
    Rich(Element),
}

impl From<&str> for StatusDotIcon {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}
impl From<String> for StatusDotIcon {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
impl From<Element> for StatusDotIcon {
    fn from(value: Element) -> Self {
        Self::Rich(value)
    }
}
impl From<bool> for StatusDotIcon {
    fn from(_: bool) -> Self {
        Self::Empty
    }
}
impl<T: Into<StatusDotIcon>> From<Option<T>> for StatusDotIcon {
    fn from(value: Option<T>) -> Self {
        value.map(Into::into).unwrap_or_default()
    }
}
macro_rules! integer_icons {
    ($($ty:ty),* $(,)?) => {$(impl From<$ty> for StatusDotIcon {
        fn from(value: $ty) -> Self { Self::Text(value.to_string()) }
    })*};
}
integer_icons!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

/// The required accessible status label was empty or whitespace-only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusDotError {
    /// Supply a meaningful label; tooltip text is not a substitute for a name.
    EmptyLabel,
}
impl Display for StatusDotError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyLabel => formatter.write_str("StatusDot label must contain non-whitespace text so the status has an accessible name without hover"),
        }
    }
}
impl Error for StatusDotError {}

/// Fixed 8px, non-shrinking status signal with required tone and accessible name.
/// Caller artwork is centered in a hidden 8px field. Pair color with visible
/// text or a distinct shape; a screen-reader name is not a visual alternative.
///
/// A nonempty tooltip upgrades a standalone dot to a keyboard/touch toggle after
/// native placement is known. Inside a control it remains an image, previews on
/// hover only, and delegates the explanation to the host's content-derived name.
/// Explicit host aria-label/aria-labelledby remain the application's responsibility.
/// SSR starts as an image without an automatic tab stop. Native root props override
/// role/name; event builders run before tooltip handling and can prevent default.
///
/// ```compile_fail
/// use astryx_dioxus::StatusDot;
/// use dioxus::prelude::*;
/// let _ = rsx! { StatusDot { label: "Online" } };
/// ```
/// ```compile_fail
/// use astryx_dioxus::{StatusDot, StatusDotVariant};
/// use dioxus::prelude::*;
/// let _ = rsx! { StatusDot { variant: StatusDotVariant::SUCCESS } };
/// ```
#[component]
pub fn StatusDot(
    /// Required, theme-extensible semantic tone.
    variant: StatusDotVariant,
    /// Required status name, available without a tooltip.
    label: String,
    /// Two-second opacity pulse; reduced motion stops it.
    #[props(default)]
    is_pulsing: bool,
    /// Optional explanation; absent/whitespace-only adds no tooltip machinery.
    #[props(default)]
    tooltip: Option<String>,
    /// Optional caller-owned artwork or scalar content, hidden from AT.
    #[props(default, into)]
    icon: StatusDotIcon,
    /// Additional root class names.
    #[props(default)]
    class: String,
    /// Caller styles override component styles.
    #[props(default)]
    style: String,
    /// Native root ref callback, composed after internal mount handling.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native ARIA/data/DOM attributes and additional event builders.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    if label.trim().is_empty() {
        return Err(StatusDotError::EmptyLabel.into());
    }
    let tooltip = tooltip.filter(|value| !value.trim().is_empty());
    let id = use_hook(|| format!("astryx-status-dot-{}", current_scope_id().0));
    let caller_attribute_names = attributes
        .iter()
        .filter(|attr| !is_owned_metadata(attr.name))
        .map(|attr| attr.name.to_owned())
        .collect();
    let support = use_status_dot_tooltip(
        id.clone(),
        label.clone(),
        tooltip.clone(),
        caller_attribute_names,
    );
    let role = (!attributes
        .iter()
        .any(|attr| attr.name.eq_ignore_ascii_case("role")))
    .then_some("img");
    let name = (!attributes
        .iter()
        .any(|attr| attr.name.eq_ignore_ascii_case("aria-label")))
    .then_some(label);
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| !is_owned_metadata(attr.name))
        .collect();
    if tooltip.is_some() {
        for (name, handler) in [
            ("onkeydown", support.onkeydown),
            ("onkeyup", support.onkeyup),
        ] {
            let callers = take_events(&mut attributes, name);
            attributes.push(composed_event(name, callers, handler));
        }
        let callers = take_events(&mut attributes, "onclick");
        attributes.push(composed_event("onclick", callers, support.onclick));
    }
    let mount_callers = take_events(&mut attributes, "onmounted");
    attributes.push(composed_event(
        "onmounted",
        mount_callers,
        EventHandler::new(move |event: MountedEvent| {
            support.onmounted.call(event.clone());
            if let Some(handler) = onmounted {
                handler.call(event);
            }
        }),
    ));
    let icon = match icon {
        StatusDotIcon::Empty => None,
        StatusDotIcon::Text(text) if text.is_empty() => None,
        StatusDotIcon::Text(text) => Some(rsx! {"{text}"}),
        StatusDotIcon::Rich(content) => Some(content),
    };
    rsx! {
        span {
            class: format!("astryx-status-dot {class}"), style,
            role, "aria-label": name, "data-variant": variant.as_str(),
            "data-astryx-pulsing": is_pulsing.then_some("true"),
            "data-astryx-status-tooltip-id": tooltip.as_ref().map(|_| id.clone()),
            ..attributes,
            if let Some(icon) = icon { span {class:"astryx-status-dot__icon","aria-hidden":"true",{icon}} }
        }
    }
}

fn is_owned_metadata(name: &str) -> bool {
    matches!(
        name,
        "class"
            | "style"
            | "data-variant"
            | "data-astryx-status-tooltip-id"
            | "data-astryx-pulsing"
            | "data-astryx-tooltip-placement"
            | "data-astryx-tooltip-pinned"
            | "data-astryx-hit-area"
    )
}

fn take_events(attributes: &mut Vec<Attribute>, name: &str) -> Vec<Attribute> {
    let callers = attributes
        .iter()
        .filter(|attr| attr.name == name)
        .cloned()
        .collect();
    attributes.retain(|attr| attr.name != name);
    callers
}

// Keep the platform payload for raw event builders; converting it to MouseData
// first would make their erased native-listener downcast invalid. Mapping later
// preserves the shared prevent-default/propagation state for internal handling.
fn composed_event<T>(
    name: &'static str,
    callers: Vec<Attribute>,
    handler: EventHandler<Event<T>>,
) -> Attribute
where
    T: 'static + for<'event> From<&'event PlatformEventData>,
{
    Attribute::new(
        name,
        AttributeValue::listener(move |event: Event<PlatformEventData>| {
            for caller in &callers {
                if let AttributeValue::Listener(callback) = &caller.value {
                    callback.call(event.clone().into_any());
                }
            }
            handler.call(event.map(|data| T::from(data)));
        }),
        None,
        false,
    )
}
