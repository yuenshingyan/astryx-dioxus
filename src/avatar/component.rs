//! Standalone Avatar adapted from Astryx Avatar/Avatar.tsx.

use std::collections::BTreeMap;

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use super::avatar_size_context::AvatarSizeContext;
use super::avatar_status_label_context::AvatarStatusLabelContext;
use crate::avatar_group::avatar_group_context::avatar_group_settings;
use crate::hooks::use_press_feedback::{PressFeedbackConfig, use_feedback_controller};
use crate::link::{LinkComponentType, LinkRenderProps, use_link_component};
use crate::utils::disabled_interaction::{cancel_disabled_click, cancel_disabled_key};
use crate::utils::is_renderable::is_renderable;
use crate::utils::safe_url::is_safe_url;

/// Avatar dimensions on the upstream scale.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    /// 20 pixels.
    ExtraSmall,
    /// 24 pixels.
    Small,
    /// 36 pixels.
    #[default]
    Medium,
    /// 48 pixels.
    Large,
    /// 128 pixels.
    ExtraLarge,
    /// Explicit size from the numeric scale.
    Pixels(AvatarPixelSize),
}

/// Explicit pixel sizes supported by Avatar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum AvatarPixelSize {
    Px16 = 16,
    Px20 = 20,
    Px24 = 24,
    Px32 = 32,
    Px36 = 36,
    Px40 = 40,
    Px48 = 48,
    Px60 = 60,
    Px64 = 64,
    Px72 = 72,
    Px96 = 96,
    Px128 = 128,
    Px144 = 144,
    Px180 = 180,
}

impl AvatarSize {
    /// Resolved side length in pixels.
    pub const fn pixels(self) -> u32 {
        match self {
            Self::ExtraSmall => 20,
            Self::Small => 24,
            Self::Medium => 36,
            Self::Large => 48,
            Self::ExtraLarge => 128,
            Self::Pixels(size) => size as u32,
        }
    }
    pub(crate) fn selector(self) -> String {
        match self {
            Self::ExtraSmall => "xsm",
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
            Self::ExtraLarge => "xl",
            Self::Pixels(size) => return (size as u32).to_string(),
        }
        .into()
    }
}

/// Avatar silhouette.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarShape {
    /// Circular avatar.
    #[default]
    Circle,
    /// Rounded square.
    Rounded,
    /// Square without corner rounding.
    Square,
}

impl AvatarShape {
    pub(crate) const fn selector(self) -> &'static str {
        match self {
            Self::Circle => "circle",
            Self::Rounded => "rounded",
            Self::Square => "square",
        }
    }
}

/// Tooltip content for an avatar.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum AvatarTooltip {
    /// Show the supplied name when it is nonblank.
    #[default]
    Name,
    /// Show a custom description.
    Text(String),
    /// Suppress the tooltip.
    None,
}

fn initials(name: &str) -> String {
    let mut letters = name.split_whitespace().filter_map(|word| {
        word.chars()
            .find(|c| c.is_alphanumeric() || matches!(*c as u32, 0x1f000..=0x1faff))
    });
    letters
        .next()
        .into_iter()
        .chain(letters.next_back())
        .flat_map(char::to_uppercase)
        .collect()
}

#[component]
fn Face(
    src: Option<String>,
    fallback_src: Option<String>,
    initials: String,
    size: String,
    pixels: u32,
) -> Element {
    let mut failed_primary = use_signal(|| None::<String>);
    let mut failed_fallback = use_signal(|| None::<String>);
    if let Some(src) = src.filter(|src| failed_primary.read().as_ref() != Some(src)) {
        let failed = src.clone();
        return rsx! { img { key: "primary:{src}", class: "astryx-avatar__image", src, alt: "", onerror: move |_| failed_primary.set(Some(failed.clone())) } };
    }
    if let Some(src) = fallback_src.filter(|src| failed_fallback.read().as_ref() != Some(src)) {
        let failed = src.clone();
        return rsx! { img { key: "fallback:{src}", class: "astryx-avatar__image", src, alt: "", onerror: move |_| failed_fallback.set(Some(failed.clone())) } };
    }
    let glyph_size = f64::from(pixels) * 0.6;
    let escaped = initials.replace('\\', "\\\\").replace('"', "\\\"");
    rsx! {
        div { class: "astryx-avatar-fallback", "data-size": size, style: "--_avatar-font-size:{pixels}px;--_avatar-initials:\"{escaped}\"", "aria-hidden": "true",
            if initials.is_empty() {
                svg { width: "{glyph_size}", height: "{glyph_size}", view_box: "0 0 24 24", fill: "currentColor",
                    path { d: "M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z" }
                }
            }
        }
    }
}

/// An image, initials, or silhouette that represents a person or entity.
#[component]
pub fn Avatar(
    /// Primary image source.
    #[props(default)]
    src: Option<String>,
    /// Secondary image source, used if the primary source is absent or fails.
    #[props(default)]
    fallback_src: Option<String>,
    /// Identity used for initials and the default tooltip.
    #[props(default)]
    name: Option<String>,
    /// Accessible identity, preferred over name.
    #[props(default)]
    alt: Option<String>,
    /// Avatar dimensions.
    #[props(default)]
    size: AvatarSize,
    /// Avatar silhouette.
    #[props(default)]
    shape: AvatarShape,
    /// Content displayed at the inline-end corner.
    #[props(default)]
    status: Option<Element>,
    /// Explicit status text for custom artwork or server-rendered names.
    #[props(default)]
    status_label: Option<String>,
    /// Tooltip behavior.
    #[props(default)]
    tooltip: AvatarTooltip,
    /// Navigation destination.
    #[props(default)]
    href: Option<String>,
    /// Override the provider's link renderer.
    #[props(default)]
    render_link: Option<LinkComponentType>,
    /// Link target.
    #[props(default)]
    target: Option<String>,
    /// Link relationship.
    #[props(default)]
    rel: Option<String>,
    /// Activation callback.
    #[props(default)]
    on_click: Option<EventHandler<MouseEvent>>,
    /// Classes on the avatar root.
    #[props(default)]
    class: String,
    /// Inline declarations on the avatar root.
    #[props(default)]
    style: String,
    /// Mounted callback on the avatar root.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native attributes on the avatar root.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let group = avatar_group_settings();
    let is_grouped = group.owner.is_some();
    let (size, shape) = if is_grouped {
        (group.size, group.shape)
    } else {
        (size, shape)
    };
    let pixels = size.pixels();
    let mut size_ctx = use_context_provider(|| AvatarSizeContext(Signal::new(pixels)));
    if *size_ctx.0.peek() != pixels {
        size_ctx.0.set(pixels);
    }
    let labels = use_signal(BTreeMap::new);
    use_context_provider(|| AvatarStatusLabelContext(Some(labels)));
    let status_label = status_label
        .filter(|label| !label.trim().is_empty())
        .or_else(|| labels.read().values().next().cloned());
    let name = name.filter(|name| !name.trim().is_empty());
    let alt = alt.filter(|name| !name.trim().is_empty());
    let identity = alt.as_ref().or(name.as_ref());
    let accessible_name = match (identity, status_label) {
        (Some(name), Some(status)) => Some(format!("{name}, {status}")),
        (Some(name), None) => Some(name.clone()),
        (None, status) => status,
    };
    let is_custom = matches!(tooltip, AvatarTooltip::Text(_));
    let tooltip = match tooltip {
        AvatarTooltip::Name => name.clone(),
        AvatarTooltip::Text(text) => Some(text),
        AvatarTooltip::None => None,
    }
    .filter(|text| !text.trim().is_empty());
    let id = format!("avatar-{}", current_scope_id().0);
    let renderer = use_link_component(render_link);
    let is_link = href.is_some();
    let is_blocked = href.as_ref().is_some_and(|href| !is_safe_url(href));
    let is_button = !is_link && on_click.is_some();
    let has_tooltip = tooltip.is_some();
    let feedback = use_feedback_controller(
        PressFeedbackConfig {
            id: id.clone(),
            label: if is_custom {
                accessible_name.clone().unwrap_or_default()
            } else {
                tooltip.clone().unwrap_or_default()
            },
            tooltip,
            is_icon_only: false,
            is_loading: false,
            is_disabled: is_blocked,
        },
        false,
        has_tooltip,
    );
    let mounted = EventHandler::new(move |event: MountedEvent| {
        feedback.call(event.clone());
        if let Some(callback) = onmounted {
            callback.call(event);
        }
    });
    let attr_present = |key| {
        attributes
            .iter()
            .any(|attr| attr.name == key && !matches!(attr.value, AttributeValue::None))
    };
    let has_name = attr_present("aria-label") || attr_present("aria-labelledby");
    let has_derived_name = has_name || accessible_name.is_some();
    let has_role = attr_present("role");
    let has_hidden = attr_present("aria-hidden");
    let has_tabindex = attr_present("tabindex");
    let mut attrs: Vec<_> = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class"
                    | "style"
                    | "onmounted"
                    | "data-size"
                    | "data-shape"
                    | "data-astryx-feedback-id"
                    | "data-avatar-item"
                    | "data-astryx-avatar-group-member"
            ) && !matches!(attr.value, AttributeValue::None)
        })
        .collect();
    let shape_name = shape.selector();
    let size_name = size.selector();
    attrs.extend([
        Attribute::new(
            "class",
            format!(
                "astryx-avatar astryx-avatar--public {}{class}",
                if is_grouped {
                    "astryx-avatar--grouped "
                } else {
                    ""
                }
            ),
            None,
            false,
        ),
        Attribute::new(
            "style",
            format!("--_avatar-size:{pixels}px;{style}"),
            None,
            false,
        ),
        Attribute::new("data-size", size_name.clone(), None, false),
        Attribute::new("data-shape", shape_name, None, false),
        Attribute::new("data-astryx-feedback-id", id, None, false),
    ]);
    if is_link || is_button {
        attrs.push(Attribute::new("data-avatar-item", "", None, false));
        if let Some(owner) = group.owner {
            attrs.push(Attribute::new(
                "data-astryx-avatar-group-member",
                owner.to_string(),
                None,
                false,
            ));
        }
    }
    if !has_name && let Some(name) = accessible_name {
        attrs.push(Attribute::new("aria-label", name, None, false));
    }
    if !is_link && !is_button {
        if !has_role {
            attrs.push(Attribute::new(
                "role",
                if has_derived_name {
                    "img"
                } else {
                    "presentation"
                },
                None,
                false,
            ));
        }
        if !has_hidden && !has_derived_name {
            attrs.push(Attribute::new("aria-hidden", "true", None, false));
        }
        if !has_tabindex && has_tooltip && !is_grouped {
            attrs.push(Attribute::new("tabindex", "0", None, false));
        }
    }
    let initials = name.as_deref().map(initials).unwrap_or_default();
    let visual = rsx! {
        div { class: "astryx-avatar__content", "aria-hidden": "true", Face { src, fallback_src, initials, size: size_name, pixels } }
        if let Some(status) = status.filter(is_renderable) { div { class: "astryx-avatar__status", "aria-hidden": "true", {status} } }
    };
    let click = EventHandler::new(move |event: MouseEvent| {
        if is_blocked {
            cancel_disabled_click(&event);
        } else if let Some(callback) = on_click {
            callback.call(event);
        }
    });
    if is_link {
        if let Some(renderer) = renderer.filter(|_| !is_blocked) {
            return renderer.call(LinkRenderProps {
                href,
                target,
                rel,
                attributes: attrs,
                onclick: click,
                onkeydown: EventHandler::default(),
                onmounted: mounted,
                children: visual,
            });
        }
        return rsx! { a { href: href.filter(|_| !is_blocked), target, rel, role: is_blocked.then_some("link"), "aria-disabled": is_blocked.then_some("true"), tabindex: is_blocked.then_some("0"), onclick: click,
        onkeydown: move |event| { if is_blocked { cancel_disabled_key(&event); } }, onmounted: mounted, ..attrs, {visual} } };
    }
    if is_button {
        return rsx! { button { r#type: "button", onclick: click, onmounted: mounted, ..attrs, {visual} } };
    }
    rsx! { div { onmounted: mounted, ..attrs, {visual} } }
}
