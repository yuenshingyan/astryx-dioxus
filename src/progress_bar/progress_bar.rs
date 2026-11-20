// Adapted from Meta Astryx 476a3153417dbf75c565f7df05b0784e94ddd59e.
// Approved corrections: inset distinction boundary and accent custom baseline.

use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;

use crate::visually_hidden::VisuallyHidden;

use super::progress_bar_mark_tooltip::use_progress_bar_mark_tooltip;
use super::progress_bar_touch::{ProgressBarMotionMode, use_progress_bar_touch};

/// Theme-extensible progress tone. Custom names retain their identity and use
/// accent/on-accent until a theme paints them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgressBarVariant(Cow<'static, str>);

impl ProgressBarVariant {
    /// Accent fill (default).
    pub const ACCENT: Self = Self(Cow::Borrowed("accent"));
    /// Success fill.
    pub const SUCCESS: Self = Self(Cow::Borrowed("success"));
    /// Warning fill.
    pub const WARNING: Self = Self(Cow::Borrowed("warning"));
    /// Error fill.
    pub const ERROR: Self = Self(Cow::Borrowed("error"));
    /// Neutral, text-disabled fill without disabled label styling.
    pub const NEUTRAL: Self = Self(Cow::Borrowed("neutral"));

    /// Validate an open theme identifier without rewriting it.
    pub fn new(name: impl Into<String>) -> Result<Self, InvalidProgressBarVariant> {
        let name = name.into();
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(InvalidProgressBarVariant);
        }
        Ok(Self(Cow::Owned(name)))
    }

    /// Original or validated theme name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl Default for ProgressBarVariant {
    fn default() -> Self {
        Self::ACCENT
    }
}

/// A progress theme name was empty or contained unsupported characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidProgressBarVariant;
impl Display for InvalidProgressBarVariant {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ProgressBar variant must be a nonempty identifier containing only ASCII letters, digits, hyphens, or underscores")
    }
}
impl Error for InvalidProgressBarVariant {}

/// A named target on the same value/max scale as determinate progress.
/// Nonfinite values are omitted; finite values clamp to the track edges.
#[derive(Clone, Debug, PartialEq)]
pub struct ProgressBarMark {
    /// Position before normalization/clamping.
    pub value: f64,
    /// Accessible name and tooltip text. Finite marks require nonempty text.
    pub label: String,
}
impl ProgressBarMark {
    /// Construct a target. Public input validation occurs when the bar renders.
    pub fn new(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
        }
    }
}

/// Mutually exclusive progress modes. Values/ranges/marks only exist when known.
#[derive(Clone, Debug, PartialEq)]
pub enum ProgressBarState {
    /// Caller-owned amount. Nonfinite values/max and nonpositive max normalize
    /// to zero. Finite values clamp into the normalized range.
    Determinate {
        /// Completed amount.
        value: f64,
        /// Maximum amount.
        max: f64,
        /// Named targets, independent of the semantic progressbar subtree.
        marks: Vec<ProgressBarMark>,
    },
    /// Unknown amount. No value attributes, formatter calls or marks render.
    Indeterminate,
}
impl ProgressBarState {
    /// Known progress on the default 0..100 scale.
    pub fn value(value: f64) -> Self {
        Self::range(value, 100.0)
    }
    /// Known progress on an application-defined scale.
    pub fn range(value: f64, max: f64) -> Self {
        Self::Determinate {
            value,
            max,
            marks: Vec::new(),
        }
    }
    /// Known progress with named targets.
    pub fn marked(value: f64, max: f64, marks: Vec<ProgressBarMark>) -> Self {
        Self::Determinate { value, max, marks }
    }
}
impl Default for ProgressBarState {
    fn default() -> Self {
        Self::value(0.0)
    }
}

/// Recoverable invalid text at the public boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressBarError {
    /// A progressbar needs a name even when its label is visually hidden.
    EmptyLabel,
    /// Every finite target needs a name and explanatory tooltip text.
    EmptyMarkLabel,
    /// Default percentage messages contain `{percent}` exactly once.
    InvalidPercentageTemplate,
    /// Custom/default value text must remain meaningful to assistive technology.
    EmptyValueText,
}
impl Display for ProgressBarError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::EmptyLabel => "ProgressBar label must contain non-whitespace text so the progressbar has an accessible name",
            Self::EmptyMarkLabel => "ProgressBar finite marks need non-whitespace labels for their accessible name and tooltip",
            Self::InvalidPercentageTemplate => "ProgressBar percentage_template must contain {percent} exactly once and no other brace placeholders",
            Self::EmptyValueText => "ProgressBar value formatter must return non-whitespace text for aria-valuetext",
        })
    }
}
impl Error for ProgressBarError {}

struct NormalizedProgress {
    value: f64,
    max: f64,
    percentage: f64,
    marks: Vec<NormalizedMark>,
}
struct NormalizedMark {
    percentage: f64,
    label: String,
    key: String,
}
fn normalized(
    value: f64,
    max: f64,
    marks: Vec<ProgressBarMark>,
) -> Result<NormalizedProgress, ProgressBarError> {
    let max = if max.is_finite() && max > 0.0 {
        max
    } else {
        0.0
    };
    let clamp = |value: f64| {
        if value.is_finite() && max > 0.0 {
            value.max(0.0).min(max)
        } else {
            0.0
        }
    };
    let value = clamp(value);
    let percentage = if max > 0.0 { value / max * 100.0 } else { 0.0 };
    let mut occurrences = HashMap::new();
    let marks = marks
        .into_iter()
        .filter(|mark| mark.value.is_finite())
        .map(|mark| {
            if mark.label.trim().is_empty() {
                return Err(ProgressBarError::EmptyMarkLabel);
            }
            let percentage = if max > 0.0 {
                clamp(mark.value) / max * 100.0
            } else {
                0.0
            };
            // Preserve source value/name identity across range changes and reorder,
            // while disambiguating repeated targets instead of collapsing them.
            let identity = format!("{}:{}", mark.value, mark.label);
            let occurrence = occurrences.entry(identity.clone()).or_insert(0usize);
            let key = format!("{identity}:{occurrence}");
            *occurrence += 1;
            Ok(NormalizedMark {
                percentage,
                label: mark.label,
                key,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(NormalizedProgress {
        value,
        max,
        percentage,
        marks,
    })
}

fn percentage_text(template: &str, percentage: f64) -> Result<String, ProgressBarError> {
    if template.matches("{percent}").count() != 1
        || template.replace("{percent}", "").contains(['{', '}'])
    {
        return Err(ProgressBarError::InvalidPercentageTemplate);
    }
    Ok(template.replacen("{percent}", &format!("{:.0}", percentage.round()), 1))
}

/// Named, caller-controlled progress with a sufficient standalone graphic.
/// `state` keeps unknown progress separate from value/range/mark data.
/// Values normalize to a finite 0..max range. Native props/ref/events reach the
/// container; the track retains its own label and protected value semantics.
///
/// Supply `percentage_template` for localized percentage messages, or supply a
/// formatter for application-specific units. Neither runs while indeterminate.
/// The default English message reproduces the source without adding a partial
/// locale provider. The original fill colors remain, with an approved inset
/// boundary and accent fallback for theme-added variants.
///
/// ```compile_fail
/// use astryx_dioxus::ProgressBar;
/// use dioxus::prelude::*;
/// let _ = rsx! {ProgressBar {}};
/// ```
#[component]
pub fn ProgressBar(
    /// Required accessible operation name.
    label: String,
    /// Known amount/range/marks, or unknown progress.
    #[props(default)]
    state: ProgressBarState,
    /// Semantic fill tone. Theme extensions retain an accent baseline.
    #[props(default)]
    variant: ProgressBarVariant,
    /// Visually hide the label without removing the accessible name.
    #[props(default)]
    is_label_hidden: bool,
    /// Show the same determinate value text used by aria-valuetext.
    #[props(default)]
    has_value_label: bool,
    /// Mute fill/text/marks. This adds no ARIA disabled state.
    #[props(default)]
    is_disabled: bool,
    /// Override determinate value text with normalized (value,max) inputs.
    #[props(default)]
    format_value_label: Option<Callback<(f64, f64), String>>,
    /// Localized default message containing `{percent}` exactly once.
    #[props(default = "{percent}%".to_owned())]
    percentage_template: String,
    /// Additional container class names.
    #[props(default)]
    class: String,
    /// Caller container styles.
    #[props(default)]
    style: String,
    /// Caller ref callback for the actual container.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Additional native container attributes and event builders.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    if label.trim().is_empty() {
        return Err(ProgressBarError::EmptyLabel.into());
    }
    let progress = match state {
        ProgressBarState::Determinate { value, max, marks } => Some(normalized(value, max, marks)?),
        ProgressBarState::Indeterminate => None,
    };
    let is_indeterminate = progress.is_none();
    let percentage = progress
        .as_ref()
        .map_or(0.0, |progress| progress.percentage);
    let value_text = progress
        .as_ref()
        .map(|progress| {
            let text = if let Some(formatter) = format_value_label {
                formatter.call((progress.value, progress.max))
            } else {
                percentage_text(&percentage_template, progress.percentage)?
            };
            if text.trim().is_empty() {
                return Err(ProgressBarError::EmptyValueText);
            }
            Ok(text)
        })
        .transpose()?;
    let should_show_value = has_value_label && !is_indeterminate;
    let id = use_hook(|| format!("astryx-progress-bar-{}", current_scope_id().0));
    let label_id = format!("{id}-label");
    let fill_variant = if is_disabled {
        "disabled"
    } else {
        variant.as_str()
    };
    let has_marks = progress
        .as_ref()
        .is_some_and(|progress| !progress.marks.is_empty());
    let mounted = use_progress_bar_touch(
        id.clone(),
        percentage,
        if is_indeterminate {
            ProgressBarMotionMode::Indeterminate
        } else {
            ProgressBarMotionMode::Determinate
        },
        has_marks,
        None,
    );
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class"
                    | "style"
                    | "data-variant"
                    | "data-mode"
                    | "data-disabled"
                    | "data-astryx-progress-bar-id"
                    | "data-astryx-progress-bar-motion"
                    | "data-astryx-progress-bar-direction"
                    | "data-astryx-progress-bar-empty"
            )
        })
        .collect();
    let mounted_callers: Vec<_> = attributes
        .iter()
        .filter(|attr| attr.name == "onmounted")
        .cloned()
        .collect();
    attributes.retain(|attr| attr.name != "onmounted");
    attributes.push(Attribute::new(
        "onmounted",
        AttributeValue::listener(move |event: Event<PlatformEventData>| {
            for attr in &mounted_callers {
                if let AttributeValue::Listener(callback) = &attr.value {
                    callback.call(event.clone().into_any());
                }
            }
            let event = event.map(|data| MountedData::from(data));
            mounted.call(event.clone());
            if let Some(handler) = onmounted {
                handler.call(event);
            }
        }),
        None,
        false,
    ));
    let value_now = progress.as_ref().map(|p| p.value.to_string());
    let value_max = progress.as_ref().map(|p| p.max.to_string());
    let marks = progress.map(|progress| progress.marks).unwrap_or_default();
    rsx! {
        div {
            class:format!("astryx-progress-bar {class}"),style,
            "data-variant":variant.as_str(),"data-mode":if is_indeterminate{"indeterminate"}else{"determinate"},
            "data-disabled":is_disabled.then_some("true"),"data-astryx-progress-bar-id":id.clone(),
            ..attributes,
            if !is_label_hidden || should_show_value {
                div {class:"astryx-progress-bar__header",
                    span {id:label_id.clone(),class:if is_label_hidden{"astryx-progress-bar__label astryx-progress-bar__label--hidden"}else{"astryx-progress-bar__label"},"{label}"}
                    if should_show_value {span {class:"astryx-progress-bar__value",{value_text.clone()}}}
                }
            } else {VisuallyHidden{id:label_id.clone(),"{label}"}}
            div {class:"astryx-progress-bar__track-container",
                div {class:"astryx-progress-bar-track",role:"progressbar","aria-labelledby":label_id,
                    "aria-valuenow":value_now,
                    "aria-valuemin":(!is_indeterminate).then_some("0"),
                    "aria-valuemax":value_max,
                    "aria-valuetext":value_text,
                    div {class:"astryx-progress-bar-fill","data-variant":fill_variant,
                        "data-empty":(!is_indeterminate&&percentage==0.0).then_some("true"),
                        style:format!("--_progress-bar-fill:{percentage}%"),
                    }
                }
                for mark in marks {
                    ProgressBarTick{key:"{mark.key}",label:mark.label,percentage:mark.percentage,fill_variant:fill_variant.to_owned(),is_on_fill:percentage>0.0&&mark.percentage<=percentage}
                }
            }
        }
    }
}

#[component]
fn ProgressBarTick(
    label: String,
    percentage: f64,
    fill_variant: String,
    is_on_fill: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-progress-mark-{}", current_scope_id().0));
    let mounted = use_progress_bar_mark_tooltip(id.clone(), label.clone());
    rsx! {span{
        class:"astryx-progress-bar-mark",role:"img","aria-label":label,tabindex:"0",
        "data-variant":fill_variant,"data-placement":if is_on_fill{"fill"}else{"track"},
        "data-astryx-mark-percent":percentage.to_string(),"data-astryx-progress-mark-id":id,
        style:format!("inset-inline-start:{percentage}%"),onmounted:mounted,
    }}
}
