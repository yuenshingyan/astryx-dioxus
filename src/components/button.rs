// Adapted from Meta Astryx Button. See THIRD_PARTY_NOTICES.md.

use std::future::Future;
use std::pin::Pin;

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::html::PlatformEventData;
use dioxus::prelude::*;

use super::button_link::{ButtonLinkContext, ButtonLinkProps, ButtonLinkRenderer};
use super::button_support::{
    ButtonSizeContext, IconDefaultSizeProvider, IconSize, Spinner, SpinnerShade, SpinnerSize,
    use_button_environment,
};

/// Button height: the shared 28/32/36px control-size scale.
pub use super::button_support::ControlSize as ButtonSize;

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
/// name. Override `loading_label` for localized loading announcements.
/// See `docs/button-port.md` for upstream references and verification.
#[component]
pub fn Button(
    label: String,
    #[props(default)] children: Option<Element>,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: Option<ButtonSize>,
    #[props(default)] elevation: ButtonElevation,
    #[props(default)] button_type: ButtonType,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_loading: bool,
    #[props(default)] is_interruptible: bool,
    #[props(default)] is_icon_only: bool,
    #[props(default)] icon: Option<Element>,
    #[props(default)] end_content: Option<Element>,
    #[props(default = "Loading".to_string())] loading_label: String,
    #[props(default)] tooltip: Option<String>,
    #[props(default)] width: Option<String>,
    #[props(default)] href: Option<String>,
    #[props(default)] target: Option<String>,
    #[props(default)] rel: Option<String>,
    #[props(default)] render_link: Option<ButtonLinkRenderer>,
    #[props(default)] name: Option<String>,
    #[props(default)] value: Option<String>,
    #[props(default)] form: Option<String>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    #[props(default)] onkeydown: EventHandler<KeyboardEvent>,
    #[props(default)] onmounted: EventHandler<MountedEvent>,
    #[props(default)] onfocus: Option<EventHandler<FocusEvent>>,
    #[props(default)] onblur: Option<EventHandler<FocusEvent>>,
    #[props(default)] onkeyup: Option<EventHandler<KeyboardEvent>>,
    #[props(default)] onpointerdown: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointerup: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointermove: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointercancel: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointerenter: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointerleave: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointerover: Option<EventHandler<PointerEvent>>,
    #[props(default)] onpointerout: Option<EventHandler<PointerEvent>>,
    #[props(default)] onmousedown: Option<EventHandler<MouseEvent>>,
    #[props(default)] onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(default)] onmouseenter: Option<EventHandler<MouseEvent>>,
    #[props(default)] onmouseleave: Option<EventHandler<MouseEvent>>,
    #[props(default)] oncontextmenu: Option<EventHandler<MouseEvent>>,
    #[props(default)] ondblclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] ontouchstart: Option<EventHandler<TouchEvent>>,
    #[props(default)] ontouchmove: Option<EventHandler<TouchEvent>>,
    #[props(default)] ontouchend: Option<EventHandler<TouchEvent>>,
    #[props(default)] ontouchcancel: Option<EventHandler<TouchEvent>>,
    #[props(default)] click_action: Option<ButtonAction>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let tooltip = tooltip.filter(|text| !text.trim().is_empty());
    let environment = use_button_environment();
    let inherited_size = try_consume_context::<ButtonSizeContext>().map(|context| (context.0)());
    let size = size
        .or(inherited_size)
        .or(environment.size)
        .unwrap_or_default();
    let elevation = if environment.is_grouped {
        ButtonElevation::None
    } else {
        elevation
    };
    let is_disabled = is_disabled
        || environment.is_disabled
        || href
            .as_deref()
            .is_some_and(|href| !is_safe_destination(href));
    let controller_id = use_hook(|| format!("astryx-button-{}", current_scope_id().0));
    let mut browser_controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(controller) = *browser_controller.peek() {
            let _ = controller.send("dispose");
        }
    });
    let mut pending_actions = use_signal(|| 0usize);
    let mut is_mounted = use_signal(|| false);
    let mut is_live_region_ready = use_signal(|| !is_loading);
    let is_pending = pending_actions() > 0;
    let is_loading_state = is_loading || is_pending;
    let controller_config = (
        controller_id.clone(),
        label.clone(),
        tooltip.clone(),
        is_icon_only,
        is_loading_state,
    );
    use_effect(use_reactive(&controller_config, move |config| {
        if let Some(controller) = *browser_controller.read() {
            let _ = controller.send(config);
        }
    }));
    let availability = Availability::resolve(is_disabled, is_loading_state, is_interruptible);
    let is_blocked = availability.blocks_activation();
    let is_delayed = is_pending || is_interruptible;
    let has_custom_children = children.is_some();
    let has_derived_label = is_icon_only || is_loading_state || has_custom_children;
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
    let aria_label = has_derived_label.then_some(label.clone());
    let is_caller_busy = aria_is_true(&attributes, "aria-busy");
    let is_caller_disabled = aria_is_true(&attributes, "aria-disabled");
    let tooltip_id = format!("{controller_id}-tooltip");
    let caller_label = text_attribute(&attributes, "aria-label");
    let accessible_name = if attributes
        .iter()
        .any(|attribute| attribute.name == "aria-labelledby")
    {
        None
    } else if has_derived_label {
        Some(label.as_str())
    } else {
        caller_label
            .as_deref()
            .or_else(|| (icon.is_none() && end_content.is_none()).then_some(label.as_str()))
    };
    let is_tooltip_name = tooltip
        .as_deref()
        .zip(accessible_name)
        .is_some_and(|(tooltip, name)| tooltip.trim() == name.trim());
    let caller_description = text_attribute(&attributes, "aria-describedby");
    let described_by = if tooltip.is_some() && !is_tooltip_name {
        Some(
            [caller_description.as_deref(), Some(tooltip_id.as_str())]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .join(" "),
        )
    } else {
        caller_description
    };
    let tooltip_element = tooltip.as_ref().map(|tooltip| {
        rsx! {
            div {
                id: tooltip_id.clone(),
                class: "astryx-button-tooltip",
                role: "tooltip",
                popover: "manual",
                "{tooltip}"
            }
        }
    });
    let aria_busy = (is_loading_state || is_caller_busy).then_some("true");
    let aria_disabled = (is_blocked || is_caller_disabled).then_some("true");
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "class"
                    | "style"
                    | "type"
                    | "href"
                    | "target"
                    | "rel"
                    | "name"
                    | "value"
                    | "form"
                    | "onclick"
                    | "onkeydown"
                    | "onmounted"
                    | "aria-busy"
                    | "aria-disabled"
                    | "aria-describedby"
                    | "data-variant"
                    | "data-size"
                    | "data-elevation"
                    | "data-icon-only"
                    | "data-loading"
                    | "data-loading-delayed"
                    | "data-disabled"
                    | "data-astryx-edge-comp"
                    | "data-astryx-button-id"
                    | "data-astryx-pressable"
                    | "data-astryx-grouped"
            ) && !(has_derived_label && attribute.name == "aria-label")
        })
        .collect();
    [
        ("onfocus", forwarded_event(onfocus)),
        ("onblur", forwarded_event(onblur)),
        ("onkeyup", forwarded_event(onkeyup)),
        ("onpointerdown", forwarded_event(onpointerdown)),
        ("onpointerup", forwarded_event(onpointerup)),
        ("onpointermove", forwarded_event(onpointermove)),
        ("onpointercancel", forwarded_event(onpointercancel)),
        ("onpointerenter", forwarded_event(onpointerenter)),
        ("onpointerleave", forwarded_event(onpointerleave)),
        ("onpointerover", forwarded_event(onpointerover)),
        ("onpointerout", forwarded_event(onpointerout)),
        ("onmousedown", forwarded_event(onmousedown)),
        ("onmouseup", forwarded_event(onmouseup)),
        ("onmouseenter", forwarded_event(onmouseenter)),
        ("onmouseleave", forwarded_event(onmouseleave)),
        ("oncontextmenu", forwarded_event(oncontextmenu)),
        ("ondblclick", forwarded_event(ondblclick)),
        ("ontouchstart", forwarded_event(ontouchstart)),
        ("ontouchmove", forwarded_event(ontouchmove)),
        ("ontouchend", forwarded_event(ontouchend)),
        ("ontouchcancel", forwarded_event(ontouchcancel)),
    ]
    .into_iter()
    .for_each(|(name, value)| {
        if let Some(value) = value {
            attributes.push(Attribute::new(name, value, None, false));
        }
    });

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
    let mounted_config = controller_config.clone();
    let handle_mounted = move |event: MountedEvent| {
        if let Some(previous) = *browser_controller.peek() {
            let _ = previous.send("dispose");
        }
        let controller = document::eval(include_str!("../../assets/button-controller.js"));
        let _ = controller.send(mounted_config.clone());
        browser_controller.set(Some(controller));
        is_mounted.set(true);
        onmounted.call(event);
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
                Spinner { size: SpinnerSize::Small, shade: SpinnerShade::Inherit, is_decorative: true }
            }
        }
        span {
            class: "astryx-button__content",
            "aria-hidden": is_loading_state.then_some("true"),
            if icon.is_some() {
                span { class: "astryx-button__icon",
                    IconDefaultSizeProvider {
                        size: if size == ButtonSize::Large { IconSize::Medium } else { IconSize::Small },
                        {icon}
                    }
                }
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

    let provider_link = try_consume_context::<ButtonLinkContext>().map(|context| (context.0)());
    if let Some(href) = href.filter(|_| !is_disabled) {
        if let Some(renderer) = render_link.or(provider_link) {
            let mut link_attributes = attributes;
            [
                ("class", Some(classes)),
                ("style", Some(styles)),
                ("data-variant", Some(variant_name.to_string())),
                ("data-size", Some(size.as_str().to_string())),
                ("data-elevation", Some(elevation.as_str().to_string())),
                ("data-icon-only", is_icon_only.then(|| "true".to_string())),
                ("data-loading", is_loading_state.then(|| "true".to_string())),
                (
                    "data-loading-delayed",
                    (is_loading_state && is_delayed).then(|| "true".to_string()),
                ),
                ("data-disabled", is_disabled.then(|| "true".to_string())),
                (
                    "data-astryx-edge-comp",
                    (variant == ButtonVariant::Ghost).then(String::new),
                ),
                ("data-astryx-button-id", Some(controller_id)),
                ("data-astryx-pressable", Some(String::new())),
                (
                    "data-astryx-grouped",
                    environment.is_grouped.then(|| "true".to_string()),
                ),
                ("aria-label", aria_label),
                ("aria-busy", aria_busy.map(str::to_string)),
                ("aria-disabled", aria_disabled.map(str::to_string)),
                ("aria-describedby", described_by),
            ]
            .into_iter()
            .for_each(|(name, value)| {
                if let Some(value) = value {
                    link_attributes.push(Attribute::new(name, value, None, false));
                }
            });
            let link = renderer.call(ButtonLinkProps {
                href,
                target,
                rel,
                attributes: link_attributes,
                onclick: EventHandler::new(handle_click),
                onkeydown: EventHandler::new(handle_keydown),
                onmounted: EventHandler::new(handle_mounted),
                children: content,
            });
            return rsx! { {link} {tooltip_element} };
        }
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
                "data-astryx-button-id": controller_id.clone(),
                "data-astryx-pressable": "",
                "data-astryx-grouped": environment.is_grouped.then_some("true"),
                "aria-label": aria_label,
                "aria-describedby": described_by,
                "aria-busy": aria_busy,
                "aria-disabled": aria_disabled,
                onclick: handle_click,
                onkeydown: handle_keydown,
                onmounted: handle_mounted,
                ..attributes,
                {content}
            }
            {tooltip_element}
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
            "data-astryx-button-id": controller_id,
            "data-astryx-pressable": "",
            "data-astryx-grouped": environment.is_grouped.then_some("true"),
            "aria-label": aria_label,
            "aria-describedby": described_by,
            "aria-busy": aria_busy,
            "aria-disabled": aria_disabled,
            onclick: handle_click,
            onkeydown: handle_keydown,
            onmounted: handle_mounted,
            ..attributes,
            {content}
        }
        {tooltip_element}
    }
}

fn forwarded_event<T>(handler: Option<EventHandler<Event<T>>>) -> Option<AttributeValue>
where
    T: 'static + for<'event> From<&'event PlatformEventData>,
{
    handler.map(|handler| {
        AttributeValue::listener(move |event: Event<PlatformEventData>| {
            handler.call(event.map(|data| T::from(data)));
        })
    })
}

fn text_attribute(attributes: &[Attribute], name: &str) -> Option<String> {
    attributes.iter().find_map(|attribute| {
        if attribute.name == name
            && let AttributeValue::Text(value) = &attribute.value
        {
            return Some(value.clone());
        }
        None
    })
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
