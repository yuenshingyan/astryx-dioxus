// Adapted from Meta Astryx Button. See THIRD_PARTY_NOTICES.md.

use std::future::Future;
use std::pin::Pin;

use dioxus::core::AttributeValue;
use dioxus::prelude::*;

/// Visual emphasis of a button. Secondary is the upstream default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    #[default]
    Secondary,
    Ghost,
    Destructive,
}

impl ButtonVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Ghost => "ghost",
            Self::Destructive => "destructive",
        }
    }
}

/// Button height: 28, 32, or 36 CSS pixels at the default text size.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    fn as_str(self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
        }
    }
}

/// Resting shadow of a standalone button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonElevation {
    #[default]
    None,
    Low,
    Medium,
    High,
}

impl ButtonElevation {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Low => "low",
            Self::Medium => "med",
            Self::High => "high",
        }
    }
}

/// Native form behavior of a button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

impl ButtonType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
        }
    }
}

/// Caller-owned async work. Box the future with `Box::pin(async move { ... })`.
///
/// Button tracks pending work and cancels its awaiting task on unmount. Callers
/// own error reporting and cancellation of operations detached from that future.
pub type ButtonAction = Callback<MouseEvent, Pin<Box<dyn Future<Output = ()>>>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Availability {
    Operable,
    Busy,
    Disabled,
}

impl Availability {
    fn resolve(is_disabled: bool, is_loading: bool, is_interruptible: bool) -> Self {
        if is_disabled {
            Self::Disabled
        } else if is_loading && !is_interruptible {
            Self::Busy
        } else {
            Self::Operable
        }
    }

    fn blocks_activation(self) -> bool {
        self != Self::Operable
    }
}

/// An Astryx action button or navigation link.
///
/// `label` provides visible text and a stable accessible name during loading.
/// Custom `children` replace the visible label. Disabled and busy buttons remain
/// focusable but cancel activation. `is_interruptible` permits repeated actions;
/// it does not cancel a previous action automatically.
///
/// Link destinations may be relative or use HTTP(S), mailto, or tel. Rejected
/// destinations render as inert buttons. `label` must be a non-empty accessible
/// name; callers supply localized `loading_label` text when announcements are needed.
/// See `docs/button-port.md` for upstream references and remaining parity gaps.
#[component]
pub fn Button(
    label: String,
    #[props(default)] children: Option<Element>,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] elevation: ButtonElevation,
    #[props(default)] button_type: ButtonType,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_loading: bool,
    #[props(default)] is_interruptible: bool,
    #[props(default)] is_icon_only: bool,
    #[props(default)] icon: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default)] loading_label: String,
    #[props(default)] width: Option<String>,
    #[props(default)] href: Option<String>,
    #[props(default)] target: Option<String>,
    #[props(default)] rel: Option<String>,
    #[props(default)] name: Option<String>,
    #[props(default)] value: Option<String>,
    #[props(default)] form: Option<String>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    #[props(default)] onkeydown: EventHandler<KeyboardEvent>,
    #[props(default)] click_action: Option<ButtonAction>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let is_disabled = is_disabled
        || href
            .as_deref()
            .is_some_and(|href| !is_safe_destination(href));
    let mut pending_actions = use_signal(|| 0usize);
    let mut is_mounted = use_signal(|| false);
    let mut is_live_region_ready = use_signal(|| !is_loading);
    let is_pending = pending_actions() > 0;
    let is_loading_state = is_loading || is_pending;
    let availability = Availability::resolve(is_disabled, is_loading_state, is_interruptible);
    let is_blocked = availability.blocks_activation();
    let is_delayed = is_pending || is_interruptible;
    let has_custom_children = children.is_some();
    let needs_label = is_icon_only || is_loading_state || has_custom_children;
    let variant_name = variant.as_str();
    let classes = format!("astryx-button astryx-button--{variant_name} {class}");
    let styles = width
        .as_ref()
        .map_or_else(|| style.clone(), |width| format!("width: {width}; {style}"));
    // A blocked submit/reset must not act before the renderer installs guards.
    let rendered_type = if is_blocked && !is_mounted() {
        ButtonType::Button
    } else {
        button_type
    };
    let aria_label = needs_label.then_some(label.clone());
    let caller_busy = aria_is_true(&attributes, "aria-busy");
    let caller_disabled = aria_is_true(&attributes, "aria-disabled");
    let aria_busy = (is_loading_state || caller_busy).then_some("true");
    let aria_disabled = (is_blocked || caller_disabled).then_some("true");
    let attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "aria-busy"
                    | "aria-disabled"
                    | "data-variant"
                    | "data-size"
                    | "data-elevation"
                    | "data-icon-only"
                    | "data-loading"
                    | "data-loading-delayed"
                    | "data-disabled"
                    | "data-astryx-edge-comp"
            ) && !(needs_label && attribute.name == "aria-label")
        })
        .collect();

    let handle_click = move |event: MouseEvent| {
        // Read the signal, not the render's snapshot: two same-tick clicks can
        // arrive before a pending-state rerender installs the next handler.
        let availability = Availability::resolve(
            is_disabled,
            is_loading || *pending_actions.peek() > 0,
            is_interruptible,
        );
        if availability.blocks_activation() {
            event.prevent_default();
            event.stop_propagation();
            return;
        }
        onclick.call(event.clone());
        if !event.default_action_enabled() {
            return;
        }
        if let Some(action) = click_action {
            pending_actions += 1;
            spawn(async move {
                action.call(event).await;
                pending_actions -= 1;
            });
        }
    };
    let handle_keydown = move |event: KeyboardEvent| {
        let availability = Availability::resolve(
            is_disabled,
            is_loading || *pending_actions.peek() > 0,
            is_interruptible,
        );
        if availability.blocks_activation()
            && (event.key() == Key::Enter || event.key() == Key::Character(" ".into()))
        {
            event.prevent_default();
            return;
        }
        onkeydown.call(event);
    };
    let handle_mounted = move |_: MountedEvent| {
        is_mounted.set(true);
        if !*is_live_region_ready.peek() {
            // First-render loading needs an empty live region before its text
            // changes. No frame is requested for an initially idle button.
            spawn(async move {
                let _ =
                    document::eval("await new Promise(requestAnimationFrame); return true;").await;
                is_live_region_ready.set(true);
            });
        }
    };
    let content = rsx! {
        if is_loading_state {
            span {
                class: "astryx-button__spinner-overlay",
                "aria-hidden": "true",
                span { class: "astryx-button__spinner" }
            }
        }
        span {
            class: "astryx-button__content",
            "aria-hidden": is_loading_state.then_some("true"),
            if icon.is_some() {
                span { class: "astryx-button__icon", {icon} }
            }
            if !is_icon_only {
                span { class: "astryx-button__label",
                    if has_custom_children { {children} } else { "{label}" }
                }
                if end_content.is_some() {
                    span { class: "astryx-button__end", {end_content} }
                }
            }
        }
        span {
            class: "astryx-button__status",
            role: "status",
            "aria-live": "polite",
            if is_loading_state && is_live_region_ready() { "{loading_label}" }
        }
    };

    if let Some(href) = href.filter(|_| !is_disabled) {
        return rsx! {
            a {
                class: classes,
                style: styles,
                href: href,
                target: target,
                rel: rel,
                "data-variant": variant_name,
                "data-size": size.as_str(),
                "data-elevation": elevation.as_str(),
                "data-icon-only": is_icon_only.then_some("true"),
                "data-loading": is_loading_state.then_some("true"),
                "data-loading-delayed": (is_loading_state && is_delayed).then_some("true"),
                "data-disabled": is_disabled.then_some("true"),
                "data-astryx-edge-comp": (variant == ButtonVariant::Ghost).then_some(""),
                "aria-label": aria_label,
                "aria-busy": aria_busy,
                "aria-disabled": aria_disabled,
                onclick: handle_click,
                onkeydown: handle_keydown,
                onmounted: handle_mounted,
                ..attributes,
                {content}
            }
        };
    }

    rsx! {
        button {
            class: classes,
            style: styles,
            r#type: rendered_type.as_str(),
            name: name.filter(|_| !is_disabled),
            value: value,
            form: form,
            "data-variant": variant_name,
            "data-size": size.as_str(),
            "data-elevation": elevation.as_str(),
            "data-icon-only": is_icon_only.then_some("true"),
            "data-loading": is_loading_state.then_some("true"),
            "data-loading-delayed": (is_loading_state && is_delayed).then_some("true"),
            "data-disabled": is_disabled.then_some("true"),
            "data-astryx-edge-comp": (variant == ButtonVariant::Ghost).then_some(""),
            "aria-label": aria_label,
            "aria-busy": aria_busy,
            "aria-disabled": aria_disabled,
            onclick: handle_click,
            onkeydown: handle_keydown,
            onmounted: handle_mounted,
            ..attributes,
            {content}
        }
    }
}

fn aria_is_true(attributes: &[Attribute], name: &str) -> bool {
    attributes.iter().any(|attribute| {
        attribute.name == name
            && match &attribute.value {
                AttributeValue::Text(value) => value == "true",
                AttributeValue::Bool(value) => *value,
                _ => false,
            }
    })
}

fn is_safe_destination(href: &str) -> bool {
    if href.chars().any(char::is_control) {
        return false;
    }
    let href = href.trim();
    let prefix = href.split(['/', '?', '#']).next().unwrap_or_default();
    let Some((scheme, _)) = prefix.split_once(':') else {
        return true;
    };
    ["http", "https", "mailto", "tel"]
        .iter()
        .any(|allowed| scheme.eq_ignore_ascii_case(allowed))
}

#[cfg(test)]
mod tests {
    use super::{Availability, is_safe_destination};

    #[test]
    fn destinations_reject_executable_schemes_and_control_characters() {
        [
            "javascript:alert(1)",
            "data:text/html,test",
            "java\nscript:alert(1)",
            "custom:action",
        ]
        .iter()
        .for_each(|href| assert!(!is_safe_destination(href), "{href}"));
        [
            "/docs",
            "#heading",
            "?query=1",
            "https://example.com",
            "HTTP://example.com",
            "mailto:a@example.com",
            "tel:+123",
            "./relative:name",
        ]
        .iter()
        .for_each(|href| assert!(is_safe_destination(href), "{href}"));
    }

    #[test]
    fn disabled_wins_over_interruptible_loading() {
        assert_eq!(
            Availability::resolve(true, true, true),
            Availability::Disabled
        );
        assert_eq!(
            Availability::resolve(false, true, false),
            Availability::Busy
        );
        assert_eq!(
            Availability::resolve(false, true, true),
            Availability::Operable
        );
        assert_eq!(
            Availability::resolve(false, false, false),
            Availability::Operable
        );
    }
}
