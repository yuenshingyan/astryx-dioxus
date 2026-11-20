// Adapted from Meta Astryx 476a3153417dbf75c565f7df05b0784e94ddd59e.
// Card FR3 corrects the recorded published-padding overshoot; see docs/card-port.md.
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};

use dioxus::core::AttributeValue;
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;

use crate::layout::padding::spacing_step_css;
use crate::utils::types::{Elevation, SizeValue, SpacingStep};

/// Theme-extensible Card surface. Unpainted custom names retain a transparent,
/// borderless base rather than falling through to another named variant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardVariant(Cow<'static, str>);
impl CardVariant {
    /// Standard card background with a border inset into its padding.
    pub const DEFAULT: Self = Self(Cow::Borrowed("default"));
    /// No background or border.
    pub const TRANSPARENT: Self = Self(Cow::Borrowed("transparent"));
    /// Muted supporting surface.
    pub const MUTED: Self = Self(Cow::Borrowed("muted"));
    /// Blue categorization tint, not semantic status.
    pub const BLUE: Self = Self(Cow::Borrowed("blue"));
    /// Cyan categorization tint.
    pub const CYAN: Self = Self(Cow::Borrowed("cyan"));
    /// Gray categorization tint.
    pub const GRAY: Self = Self(Cow::Borrowed("gray"));
    /// Green categorization tint.
    pub const GREEN: Self = Self(Cow::Borrowed("green"));
    /// Orange categorization tint.
    pub const ORANGE: Self = Self(Cow::Borrowed("orange"));
    /// Pink categorization tint.
    pub const PINK: Self = Self(Cow::Borrowed("pink"));
    /// Purple categorization tint.
    pub const PURPLE: Self = Self(Cow::Borrowed("purple"));
    /// Red categorization tint.
    pub const RED: Self = Self(Cow::Borrowed("red"));
    /// Teal categorization tint.
    pub const TEAL: Self = Self(Cow::Borrowed("teal"));
    /// Yellow categorization tint.
    pub const YELLOW: Self = Self(Cow::Borrowed("yellow"));
    /// Validate an open theme identifier without rewriting its spelling.
    pub fn new(name: impl Into<String>) -> Result<Self, InvalidCardVariant> {
        let name = name.into();
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(InvalidCardVariant);
        }
        Ok(Self(Cow::Owned(name)))
    }
    /// Name reflected on the Card theme target.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl Default for CardVariant {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// A Card theme identifier was empty or contained unsupported characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidCardVariant;
impl Display for InvalidCardVariant {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Card variant must be a nonempty identifier containing only ASCII letters, digits, hyphens, or underscores")
    }
}
impl Error for InvalidCardVariant {}

/// A static, themeable surface for one independently grouped item.
///
/// Card adds no role, name, focus target, interaction, state, effect or timer.
/// Sizing is preferred rather than a minimum; a supplied non-auto height enables
/// scrolling. Free-form children keep their semantics and behavior. Padding is
/// published at the four logical edges for opt-in bleed consumers; the published
/// inset equals the applied padding, correcting upstream's recorded border gap.
/// Caller CSS classes override base styles; inline `style` is applied last.
///
/// ```
/// use astryx_dioxus::{Card, CardVariant, Elevation, SizeValue};
/// use dioxus::prelude::*;
/// let _ = rsx! {
///     Card {
///         width: SizeValue::from(320), max_width: SizeValue::from("100%"),
///         variant: CardVariant::MUTED, elevation: Elevation::Low,
///         "Caller-owned content"
///     }
/// };
/// ```
#[component]
pub fn Card(
    /// Preferred width, not a minimum. Omission leaves natural width.
    #[props(default)]
    width: Option<SizeValue>,
    /// Non-auto explicit height selects a scrolling container.
    #[props(default)]
    height: Option<SizeValue>,
    /// Optional maximum width.
    #[props(default)]
    max_width: Option<SizeValue>,
    /// Optional minimum height.
    #[props(default)]
    min_height: Option<SizeValue>,
    /// Omission reads theme card padding, falling back to spacing step four.
    #[props(default)]
    padding: Option<SpacingStep>,
    /// Open categorization surface. Only default draws a built-in border.
    #[props(default)]
    variant: CardVariant,
    /// Resting shadow; composes with a caller/composing surface's inset ring.
    #[props(default)]
    elevation: Elevation,
    /// Additional root class names.
    #[props(default)]
    class: String,
    /// Caller root style, after component-generated values.
    #[props(default)]
    style: String,
    /// Native root ref callback; composes with attribute-builder mounted events.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Free-form caller content; Card supplies no structure inside it.
    #[props(default = rsx!{})]
    children: Element,
    /// Native props and events for the one surface element.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let has_fixed_height = height.as_ref().is_some_and(|height| !height.is_auto());
    let mut class = format!(
        "astryx-card astryx-card--elevation-{} {class}",
        elevation.as_str()
    );
    let mut generated_style = String::new();
    for (axis, size) in [
        ("width", width),
        ("height", height),
        ("max-width", max_width),
        ("min-height", min_height),
    ] {
        if let Some(size) = size {
            let value = size.as_css()?;
            if let Some(keyword) = size.css_wide_keyword() {
                class.push_str(&format!(" astryx-card--{axis}-{}", keyword.as_str()));
            }
            generated_style.push_str(&format!("--_card-{axis}:{value};"));
        }
    }
    if let Some(padding) = padding {
        generated_style.push_str(&format!("--_card-padding:{};", spacing_step_css(padding)));
    }
    generated_style.push_str(&style);
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "class" | "style" | "data-variant" | "data-elevation" | "data-astryx-card-scroll"
            )
        })
        .collect();
    if let Some(handler) = onmounted {
        let mounted_callers: Vec<_> = attributes
            .iter()
            .filter(|attribute| attribute.name == "onmounted")
            .cloned()
            .collect();
        attributes.retain(|attribute| attribute.name != "onmounted");
        attributes.push(Attribute::new(
            "onmounted",
            AttributeValue::listener(move |event: Event<PlatformEventData>| {
                for attribute in &mounted_callers {
                    if let AttributeValue::Listener(callback) = &attribute.value {
                        callback.call(event.clone().into_any());
                    }
                }
                handler.call(event.map(|data| MountedData::from(data)));
            }),
            None,
            false,
        ));
    }
    rsx! {div{
        class,style:generated_style,
        "data-variant":variant.as_str(),"data-elevation":elevation.as_str(),
        "data-astryx-card-scroll":has_fixed_height.then_some("true"),
        ..attributes,{children}
    }}
}
