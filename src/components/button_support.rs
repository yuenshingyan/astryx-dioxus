// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use std::f64::consts::PI;

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

/// Shared control size, independent of any particular component's API.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ControlSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ControlSize {
    /// Upstream size selector.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
        }
    }
}

/// Reactive inherited control size. Read the signal to subscribe to changes.
#[derive(Clone, Copy, PartialEq)]
pub struct ButtonSizeContext(pub Signal<ControlSize>);

/// Supplies a default size without adding a DOM element. The nearest provider wins.
#[component]
pub fn SizeProvider(size: ControlSize, children: Element) -> Element {
    let mut context = use_context_provider(|| ButtonSizeContext(Signal::new(size)));
    if *context.0.peek() != size {
        context.0.set(size);
    }
    children
}

/// Direction of a connected group.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ButtonGroupOrientation {
    /// Upstream orientation selector.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Context shared by ButtonGroup and input-owned button slots.
///
/// `size: None` defers to SizeProvider, then the control's own default. A nested
/// environment replaces the entire outer environment, including disabled state;
/// this is an override boundary. A connected ButtonGroup's constraints remain
/// authoritative even inside a nested input-slot environment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ButtonEnvironment {
    pub size: Option<ControlSize>,
    pub is_disabled: bool,
    pub is_grouped: bool,
    pub orientation: ButtonGroupOrientation,
}

/// Reactive nearest environment, distinct from the general size context.
#[derive(Clone, Copy, PartialEq)]
pub struct ButtonEnvironmentContext(pub Signal<ButtonEnvironment>);

#[derive(Clone, Copy)]
struct ConnectedButtonGroupContext(Signal<ButtonEnvironment>);

/// Reads the nearest environment, or defaults when no provider exists.
///
/// Buttons resolve explicit size > nearest ButtonSizeContext > environment
/// fallback > Medium. Their own `is_disabled` is ORed with this environment's flag.
pub fn use_button_environment() -> ButtonEnvironment {
    let environment = try_consume_context::<ButtonEnvironmentContext>()
        .map(|context| (context.0)())
        .unwrap_or_default();
    if let Some(group) = try_consume_context::<ConnectedButtonGroupContext>() {
        let group = (group.0)();
        return ButtonEnvironment {
            is_disabled: environment.is_disabled || group.is_disabled,
            is_grouped: true,
            orientation: group.orientation,
            size: environment.size.or(group.size),
        };
    }
    environment
}

/// Headless integration seam for input groups and other button-slot owners.
///
/// Supply the complete slot environment; nested slot providers replace one
/// another, but do not clear an enclosing ButtonGroup's disabled/group state.
/// No InputGroup UI or application-owned operation is introduced here.
#[component]
pub fn ButtonEnvironmentProvider(environment: ButtonEnvironment, children: Element) -> Element {
    let inherited_size = try_consume_context::<ButtonSizeContext>().map(|context| (context.0)());
    let size = environment.size.or(inherited_size).unwrap_or_default();
    let mut context = use_context_provider(|| ButtonEnvironmentContext(Signal::new(environment)));
    if *context.0.peek() != environment {
        context.0.set(environment);
    }
    rsx! { SizeProvider { size, {children} } }
}

/// Resting shadow of the connected group as a single surface.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonGroupElevation {
    #[default]
    None,
    Low,
    Medium,
    High,
}

impl ButtonGroupElevation {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Low => "low",
            Self::Medium => "med",
            Self::High => "high",
        }
    }
}

/// Connected button surface and environment provider.
///
/// `label` must name the group (whitespace is trimmed). Browser integration
/// owns roving focus and touch tracking through `data-astryx-button-group`;
/// this component does not install listeners or alter children's tab indices.
#[component]
pub fn ButtonGroup(
    label: String,
    children: Element,
    #[props(default)] size: Option<ControlSize>,
    #[props(default)] orientation: ButtonGroupOrientation,
    #[props(default)] is_disabled: bool,
    #[props(default)] elevation: ButtonGroupElevation,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let inherited_size = try_consume_context::<ButtonSizeContext>().map(|context| (context.0)());
    let resolved_size = size.or(inherited_size).unwrap_or_default();
    let environment = ButtonEnvironment {
        size: Some(resolved_size),
        is_disabled,
        is_grouped: true,
        orientation,
    };
    let mut group_context =
        use_context_provider(|| ConnectedButtonGroupContext(Signal::new(environment)));
    if *group_context.0.peek() != environment {
        group_context.0.set(environment);
    }
    let label = label.trim();
    rsx! {
        ButtonEnvironmentProvider { environment,
            div {
                class: "astryx-button-group astryx-button-group--{orientation.as_str()} {class}",
                style,
                role: "group",
                "aria-label": (!label.is_empty()).then_some(label),
                "aria-disabled": is_disabled.then_some("true"),
                "data-astryx-button-group": "",
                "data-size": resolved_size.as_str(),
                "data-orientation": orientation.as_str(),
                "data-elevation": elevation.as_str(),
                "data-disabled": is_disabled.then_some("true"),
                {children}
            }
        }
    }
}

/// Icon dimensions at a 16px root: 12, 16, 20 and 24px, expressed in rem.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconSize {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
}

impl IconSize {
    /// Upstream size selector.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExtraSmall => "xsm",
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
        }
    }

    /// Default glyph dimension, before theme overrides.
    pub fn dimension(self) -> &'static str {
        match self {
            Self::ExtraSmall => "0.75rem",
            Self::Small => "1rem",
            Self::Medium => "1.25rem",
            Self::Large => "1.5rem",
        }
    }
}

/// Reactive default for unsized Icon components in an owning component's slot.
#[derive(Clone, Copy, PartialEq)]
pub struct IconDefaultSizeContext(pub Signal<IconSize>);

/// Sets the nearest icon-slot default without a wrapper element.
#[component]
pub fn IconDefaultSizeProvider(size: IconSize, children: Element) -> Element {
    let mut context = use_context_provider(|| IconDefaultSizeContext(Signal::new(size)));
    if *context.0.peek() != size {
        context.0.set(size);
    }
    children
}

/// Caller-supplied glyph with contextual sizing and accessible wrapper semantics.
///
/// The glyph should be noninteractive, inherit currentColor, and use a viewBox
/// or 1em dimensions. CSS must size direct SVG children to the wrapper. `color`
/// is an open CSS color value (including var()); it is not a semantic registry.
/// A missing, empty, or whitespace-only label makes the wrapper decorative.
#[component]
pub fn Icon(
    icon: Element,
    #[props(default)] size: Option<IconSize>,
    #[props(default)] color: Option<String>,
    #[props(default)] label: Option<String>,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let inherited_size =
        try_consume_context::<IconDefaultSizeContext>().map(|context| (context.0)());
    let size = size.or(inherited_size).unwrap_or_default();
    let dimension = size.dimension();
    let color = color
        .as_deref()
        .filter(|color| !color.trim().is_empty())
        .unwrap_or("inherit");
    let label = label
        .as_deref()
        .map(str::trim)
        .filter(|label| !label.is_empty());
    let styles = format!(
        "display:inline-flex;flex-shrink:0;align-items:center;justify-content:center;width:var(--_icon-width, {dimension});height:var(--_icon-height, {dimension});font-size:{dimension};color:{color};{style}"
    );
    rsx! {
        span {
            class: "astryx-icon {class}",
            style: styles,
            "data-size": size.as_str(),
            role: label.map(|_| "img"),
            "aria-label": label,
            "aria-hidden": label.is_none().then_some("true"),
            span { class: "astryx-icon__glyph", "aria-hidden": "true", {icon} }
        }
    }
}

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
