use dioxus::prelude::*;

/// Clipping shape; both alternatives respect the supplied ratio.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AspectRatioShape {
    /// Standard rectangular box.
    #[default]
    Rectangle,
    /// Circle at ratio one, oval otherwise.
    Ellipse,
}

impl AspectRatioShape {
    fn as_str(self) -> &'static str {
        match self {
            Self::Rectangle => "rectangle",
            Self::Ellipse => "ellipse",
        }
    }
}

/// Optional direct-child sizing, without modifying child props.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AspectRatioFit {
    /// Fill the box and crop direct image/video media.
    Cover,
    /// Fill the box and letterbox direct image/video media.
    Contain,
    /// Keep natural child size, centered by the content slot.
    Center,
}

impl AspectRatioFit {
    fn as_str(self) -> &'static str {
        match self {
            Self::Cover => "cover",
            Self::Contain => "contain",
            Self::Center => "center",
        }
    }
}

/// CSS-only width/height ratio box. Requires a definite ancestor width.
///
/// Pass one child: unset `fit` leaves it unstyled; cover/contain size every direct
/// child, so additional children can be clipped. Compose overlays in one wrapper.
/// Children retain their own semantics, alternative text, events and styles.
/// The ratio declaration lives in `astryx-base`, allowing unlayered responsive
/// CSS and caller inline styles to override it. Height-only constraints do not
/// release full width: use `width:auto` to derive width from height. Shrink-to-fit
/// parents without a definite width can collapse to zero.
#[component]
pub fn AspectRatio(
    /// Required numeric width divided by height. Invalid values are left to CSS.
    ratio: f64,
    /// Rectangle by default; ellipse clips at 50% on both axes.
    #[props(default)]
    shape: AspectRatioShape,
    /// Unset by default, preserving caller-owned child layout.
    #[props(default)]
    fit: Option<AspectRatioFit>,
    /// Single caller-owned content element, not cloned or edited.
    children: Element,
    /// Additional root classes (CSS replaces upstream StyleX `xstyle`).
    #[props(default)]
    class: String,
    /// Inline caller styles, after the ratio custom property.
    #[props(default)]
    style: String,
    /// Native root ref adapter.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native DOM, ARIA, data attributes and events forwarded to the root div.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    // Non-finite numbers must remain invalid CSS, not silently become a valid
    // fallback ratio. Finite zero/negative inputs likewise retain CSS behavior.
    let ratio = if ratio.is_finite() {
        ratio.to_string()
    } else {
        "invalid".to_owned()
    };
    let style = format!("--astryx-aspect-ratio:{ratio};{style}");
    rsx! {
        div {
            class: "astryx-aspect-ratio {class}",
            "data-shape": shape.as_str(),
            style,
            onmounted: move |event| { if let Some(handler) = onmounted { handler.call(event); } },
            ..attributes,
            div {
                class: "astryx-aspect-ratio__content",
                "data-astryx-aspect-ratio-override": fit.map(AspectRatioFit::as_str),
                {children}
            }
        }
    }
}
