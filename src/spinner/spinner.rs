// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use std::f64::consts::PI;

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

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

/// SVG indeterminate progress indicator with a 135-degree rounded arc.
///
/// `label` renders below the ring and names it via aria-labelledby unless
/// `accessible_label` overrides it. Without either, the upstream fallback is
/// "Loading"; applications should supply localized text. `is_decorative` hides
/// the whole component when an owning Button already announces progress.
///
/// CSS owns the track/arc colors, themeable geometry and compositor rotation.
/// The rotor hook `data-astryx-spinner-rotor` lets the browser controller pin
/// animations to timeline origin. No timers or listeners are created here.
#[component]
pub fn Spinner(
    #[props(default)] size: SpinnerSize,
    #[props(default)] shade: SpinnerShade,
    #[props(default)] label: Option<String>,
    #[props(default)] accessible_label: Option<String>,
    #[props(default)] is_decorative: bool,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let (diameter, stroke) = size.geometry();
    let frame = diameter + stroke * 2;
    let radius = diameter / 2;
    let circumference = PI * f64::from(diameter);
    let dash = format!("{} {}", circumference * 0.375, circumference * 0.625);
    let label = label.filter(|label| !label.trim().is_empty());
    let accessible_label = accessible_label.filter(|label| !label.trim().is_empty());
    let has_label = label.is_some();
    let is_named_by_label = has_label && accessible_label.is_none();
    let label_id = format!("astryx-spinner-label-{}", current_scope_id().0);
    let aria_label = if is_decorative || is_named_by_label {
        None
    } else {
        Some(accessible_label.unwrap_or_else(|| "Loading".to_string()))
    };
    let ring = rsx! {
        span {
            class: "astryx-spinner__ring",
            style: "width:var(--_spinner-box-size, {frame}px);height:var(--_spinner-box-size, {frame}px)",
            role: (!is_decorative).then_some("progressbar"),
            "aria-label": aria_label,
            "aria-labelledby": (!is_decorative && is_named_by_label).then_some(label_id.clone()),
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
                "aria-hidden": "true",
                svg {
                    class: "astryx-spinner__svg",
                    width: frame, height: frame,
                    circle {
                        class: "astryx-spinner__arc",
                        cx: "50%", cy: "50%", r: radius, fill: "none",
                        stroke_width: stroke, stroke_linecap: "round",
                        stroke: "var(--spinner-color, currentColor)",
                        stroke_dasharray: dash,
                        style: "transform:rotate(-90deg);transform-box:fill-box;transform-origin:center",
                    }
                }
            }
        }
    };
    rsx! {
        span {
            class: "astryx-spinner {class}",
            style,
            "data-size": size.as_str(),
            "data-shade": shade.as_str(),
            "data-has-label": has_label.then_some("true"),
            "aria-hidden": is_decorative.then_some("true"),
            {ring}
            if let Some(label) = label {
                span { class: "astryx-spinner__label", id: label_id, "{label}" }
            }
        }
    }
}
