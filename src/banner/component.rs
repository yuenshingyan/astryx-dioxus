use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;
use crate::{Button, ButtonSize, ButtonVariant, Elevation, Icon, IconSize};

#[cfg(target_arch = "wasm32")]
use super::focus_origin::FocusOrigin;

macro_rules! banner_identifier {
    ($(#[$doc:meta])* $name:ident { $($constant:ident => $value:literal),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name(Cow<'static, str>);
        impl $name {
            $(pub const $constant: Self = Self(Cow::Borrowed($value));)+
            /// Validates an open theme identifier.
            pub fn new(value: impl Into<String>) -> Result<Self, InvalidBannerIdentifier> {
                let value = value.into();
                if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_')) {
                    return Err(InvalidBannerIdentifier);
                }
                Ok(Self(Cow::Owned(value)))
            }
            /// Name reflected on the banner's theme target.
            pub fn as_str(&self) -> &str { &self.0 }
        }
    };
}

banner_identifier! {
    /// Theme-extensible message status.
    BannerStatus { INFO => "info", SUCCESS => "success", WARNING => "warning", ERROR => "error" }
}
banner_identifier! {
    /// Theme-extensible container treatment.
    BannerContainer { CARD => "card", SECTION => "section" }
}
impl Default for BannerContainer {
    fn default() -> Self {
        Self::CARD
    }
}

/// The supplied theme identifier was empty or contained unsupported characters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidBannerIdentifier;
impl Display for InvalidBannerIdentifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Expected a nonempty banner identifier of ASCII letters, digits, hyphens, or underscores")
    }
}
impl std::error::Error for InvalidBannerIdentifier {}

/// A contextual status message with optional actions and dismissal.
#[component]
pub fn Banner(
    status: BannerStatus,
    title: String,
    #[props(default)] description: Option<Element>,
    #[props(default)] icon: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] is_dismissable: bool,
    #[props(default)] on_dismiss: Option<EventHandler<()>>,
    #[props(default)] dismiss_label: Option<String>,
    #[props(default)] container: BannerContainer,
    #[props(default)] elevation: Elevation,
    /// Announces only the title and description as an urgent alert.
    #[props(default)]
    is_urgent: bool,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut is_dismissed = use_signal(|| false);
    #[cfg(target_arch = "wasm32")]
    let mut focus_origin = use_signal(|| None::<FocusOrigin>);
    #[cfg(target_arch = "wasm32")]
    use_drop(move || {
        focus_origin.write().take();
    });
    if is_dismissed() {
        return VNode::empty();
    }
    let description = description.filter(is_renderable);
    let end_content = end_content.filter(is_renderable);
    let icon = icon.filter(is_renderable).or_else(|| status_icon(&status));
    let should_center = description.is_none() && (end_content.is_some() || is_dismissable);
    let has_action = end_content.is_some();
    let dismiss_label = dismiss_label.unwrap_or_else(|| format!("Dismiss {title}"));
    let mount = move |event: MountedEvent| {
        #[cfg(target_arch = "wasm32")]
        focus_origin.set(event.data().downcast().and_then(FocusOrigin::attach));
        #[cfg(not(target_arch = "wasm32"))]
        let _ = event;
    };
    let dismiss = move |_| {
        #[cfg(target_arch = "wasm32")]
        let origin = focus_origin
            .write()
            .take()
            .and_then(FocusOrigin::into_origin);
        is_dismissed.set(true);
        if let Some(handler) = on_dismiss {
            handler.call(());
        }
        // Return focus after notifying the caller, but before the renderer removes the Banner.
        #[cfg(target_arch = "wasm32")]
        if let Some(origin) = origin.filter(|origin| origin.is_connected()) {
            let _ = origin.focus();
        }
    };
    rsx! {
        div { class: format!("astryx-banner-root {class}"), style, "data-elevation": elevation.as_str(), "data-container": container.as_str(), onmounted: mount, ..attributes,
            div { class: "astryx-banner", "data-status": status.as_str(), "data-centered": should_center.then_some("true"), "data-has-action": has_action.then_some("true"),
                if let Some(icon) = icon {
                    span { class: "astryx-banner-icon", "aria-hidden": "true", {icon} }
                }
                div { class: "astryx-banner-message", role: is_urgent.then_some("alert"),
                    div { class: "astryx-banner-title", "{title}" }
                    if let Some(description) = description {
                        div { class: "astryx-banner-description", {description} }
                    }
                }
                if has_action || is_dismissable {
                    div { class: "astryx-banner-actions",
                        {end_content}
                        if is_dismissable {
                            Button { class: "astryx-banner-dismiss", label: dismiss_label, tooltip: "Dismiss", size: ButtonSize::Small, variant: ButtonVariant::Ghost, is_icon_only: true,
                                icon: glyph("M6 6l12 12M18 6L6 18", false, IconSize::Small),
                                onclick: dismiss,
                            }
                        }
                    }
                }
            }
        }
    }
}

fn status_icon(status: &BannerStatus) -> Option<Element> {
    let path = match status.as_str() {
        "info" => "M12 11v5m0-8v.1",
        "warning" | "error" => "M12 8v5m0 3v.1",
        "success" => "m8 12 3 3 5-6",
        _ => return None,
    };
    Some(glyph(path, true, IconSize::Medium))
}

fn glyph(path: &'static str, has_circle: bool, size: IconSize) -> Element {
    rsx! { Icon { size, icon: rsx! {
        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            if has_circle { circle { cx: "12", cy: "12", r: "9" } }
            path { d: path }
        }
    } } }
}
