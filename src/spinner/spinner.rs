// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use std::error::Error;
use std::f64::consts::PI;
use std::fmt::{Display, Formatter};

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use super::spinner_rotor::use_rotation_phase;
use crate::hooks::use_announce_rendered_text::use_announce_rendered_text;
use crate::text::Text;
use crate::theme::types::TextWeight;

/// Drawn ring size. The box adds one stroke width on each side.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpinnerSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl SpinnerSize {
    fn geometry(self) -> (u32, u32) {
        match self {
            Self::Small => (10, 2),
            Self::Medium => (14, 3),
            Self::Large => (18, 3),
            Self::ExtraLarge => (28, 4),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
            Self::ExtraLarge => "xl",
        }
    }
}

/// Upstream closed set of ring and track shades.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpinnerShade {
    #[default]
    Default,
    OnMedia,
    Subtle,
    Inherit,
}

impl SpinnerShade {
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::OnMedia => "onMedia",
            Self::Subtle => "subtle",
            Self::Inherit => "inherit",
        }
    }

    fn track_opacity(self) -> f64 {
        match self {
            Self::Default | Self::Subtle => 1.0,
            Self::OnMedia => 77.0 / 255.0,
            Self::Inherit => 0.3,
        }
    }
}

/// Invalid combinations at the Spinner's public boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpinnerError {
    /// The compatible string label and rich label cannot both be supplied.
    ConflictingLabels,
    /// A localized fallback must contain text when it supplies the name.
    EmptyLoadingLabel,
}

impl Display for SpinnerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::ConflictingLabels => {
                "Spinner cannot render both label and label_content; supply one visible label"
            }
            Self::EmptyLoadingLabel => {
                "Spinner cannot supply an accessible name because loading_label is empty"
            }
        })
    }
}

impl Error for SpinnerError {}

enum VisibleLabel {
    Text(String),
    Rich(Element),
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

/// SVG indeterminate progress indicator with a 135-degree rounded arc.
///
/// `label` remains a compatible string prop. `label_content` supplies rich nodes
/// instead; supplying both returns a recoverable render error. A string label
/// renders as bold body Text and names the indicator through aria-labelledby.
/// Native aria-label (or `accessible_label`) overrides it. Rich/unlabelled forms
/// use `loading_label` as their localized fallback (default "Loading").
///
/// The unlabelled span is the root; a labelled div carries root styling and
/// native attributes. `onmounted` adapts the root ref. Standalone appearances
/// announce through the shared channel, not a live role on the indicator.
/// `is_decorative` preserves the existing owner-silencing API used by Button;
/// native aria-hidden and caller non-progressbar roles also silence announcements.
///
/// CSS owns colors and geometry; all rotors share one batched timeline origin.
#[component]
pub fn Spinner(
    #[props(default)] size: SpinnerSize,
    #[props(default)] shade: SpinnerShade,
    #[props(default)] label: Option<String>,
    #[props(default)] label_content: Option<Element>,
    #[props(default)] accessible_label: Option<String>,
    #[props(default = "Loading".to_string())] loading_label: String,
    #[props(default)] is_decorative: bool,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onmounted: EventHandler<MountedEvent>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let id = use_hook(|| format!("astryx-spinner-{}", current_scope_id().0));
    let rotor_mounted = use_rotation_phase(id.clone());
    let has_conflicting_labels = label.is_some() && label_content.is_some();
    let visible_label = match (label, label_content) {
        (Some(text), _) if !text.trim().is_empty() => Some(VisibleLabel::Text(text)),
        (_, Some(content)) => Some(VisibleLabel::Rich(content)),
        _ => None,
    };
    let has_label = visible_label.is_some();
    let label_id = format!("{id}-label");
    let caller_label = text_attribute(&attributes, "aria-label")
        .filter(|value| !value.trim().is_empty())
        .or_else(|| accessible_label.filter(|value| !value.trim().is_empty()));
    let labelled_by =
        text_attribute(&attributes, "aria-labelledby").filter(|value| !value.trim().is_empty());
    let caller_role = text_attribute(&attributes, "role");
    let is_hidden =
        is_decorative || text_attribute(&attributes, "aria-hidden").as_deref() == Some("true");
    let fallback_name = caller_label
        .clone()
        .or_else(|| match &visible_label {
            Some(VisibleLabel::Text(text)) => Some(text.clone()),
            _ => None,
        })
        .unwrap_or(loading_label);
    let is_named_by_label = matches!(&visible_label, Some(VisibleLabel::Text(_)))
        && caller_label.is_none()
        && labelled_by.is_none();
    let indicator_labelled_by =
        labelled_by.or_else(|| is_named_by_label.then_some(label_id.clone()));
    let indicator_label =
        (indicator_labelled_by.is_none() && !is_decorative).then_some(fallback_name.clone());
    let should_announce = !is_hidden
        && caller_role
            .as_deref()
            .is_none_or(|role| role == "progressbar");
    let announce_mounted = use_announce_rendered_text(
        format!(r#"[data-astryx-spinner-id="{id}"]"#),
        should_announce,
        fallback_name.clone(),
    );
    let handle_mounted = move |event: MountedEvent| {
        announce_mounted.call(event.clone());
        onmounted.call(event);
    };
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "class"
                    | "style"
                    | "onmounted"
                    | "role"
                    | "aria-label"
                    | "aria-labelledby"
                    | "aria-hidden"
                    | "data-size"
                    | "data-shade"
                    | "data-has-label"
                    | "data-astryx-spinner-id"
            )
        })
        .collect();
    if has_conflicting_labels {
        return Err(SpinnerError::ConflictingLabels.into());
    }
    if fallback_name.trim().is_empty() && indicator_labelled_by.is_none() && !is_decorative {
        return Err(SpinnerError::EmptyLoadingLabel.into());
    }
    let (diameter, stroke) = size.geometry();
    let frame = diameter + stroke * 2;
    let radius = diameter / 2;
    let circumference = PI * f64::from(diameter);
    let dash = format!("{} {}", circumference * 0.375, circumference * 0.625);
    let ring_style = format!(
        "{};width:var(--_spinner-box-size, {frame}px);height:var(--_spinner-box-size, {frame}px)",
        if has_label { "" } else { &style }
    );
    let ring_class = if has_label {
        "astryx-spinner__ring".to_string()
    } else {
        format!("astryx-spinner astryx-spinner__ring {class}")
    };
    let ring_attributes = if has_label {
        Vec::new()
    } else {
        attributes.clone()
    };
    let ring = rsx! {
        span {
            class: ring_class, style: ring_style,
            role: (!is_decorative).then(|| if has_label { "progressbar".to_string() } else { caller_role.clone().unwrap_or_else(|| "progressbar".to_string()) }),
            "aria-label": indicator_label,
            "aria-labelledby": (!is_decorative).then_some(indicator_labelled_by).flatten(),
            "aria-hidden": (!has_label && is_hidden).then_some("true"),
            "data-size": (!has_label).then_some(size.as_str()),
            "data-shade": (!has_label).then_some(shade.as_str()),
            "data-astryx-spinner-id": (!has_label).then_some(id.clone()),
            onmounted: move |event| if !has_label { handle_mounted(event); },
            ..ring_attributes,
            svg {
                class: "astryx-spinner__svg astryx-spinner__track",
                width: "0", height: "0", "aria-hidden": "true",
                circle {
                    cx: "50%", cy: "50%", r: radius, fill: "none",
                    stroke_width: stroke, stroke_linecap: "round",
                    stroke: "var(--spinner-track-color, currentColor)",
                    stroke_opacity: shade.track_opacity(),
                }
            }
            span {
                class: "astryx-spinner__rotor",
                "data-astryx-spinner-rotor": "",
                "data-astryx-spinner-rotor-id": id.clone(),
                "aria-hidden": "true", onmounted: rotor_mounted,
                svg {
                    class: "astryx-spinner__svg", width: frame, height: frame,
                    circle {
                        class: "astryx-spinner__arc",
                        cx: "50%", cy: "50%", r: radius, fill: "none",
                        stroke_width: stroke, stroke_linecap: "round",
                        stroke: "var(--spinner-color, currentColor)", stroke_dasharray: dash,
                        style: "transform:rotate(-90deg);transform-box:fill-box;transform-origin:center",
                    }
                }
            }
        }
    };
    match visible_label {
        None => ring,
        Some(label) => rsx! {
            div {
                class: "astryx-spinner astryx-spinner--labelled {class}", style,
                "data-size": size.as_str(), "data-shade": shade.as_str(), "data-has-label": "true",
                "data-astryx-spinner-id": id, "aria-hidden": is_hidden.then_some("true"),
                role: caller_role, onmounted: handle_mounted, ..attributes,
                {ring}
                {match label {
                    VisibleLabel::Text(text) => rsx! { Text { class: "astryx-spinner__label", id: label_id, weight: TextWeight::Bold, "{text}" } },
                    VisibleLabel::Rich(content) => content,
                }}
            }
        },
    }
}
