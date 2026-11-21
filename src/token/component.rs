use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::token_link::use_token_surface;
use crate::hooks::use_interactive_role::{InteractiveRole, use_interactive_role};
use crate::hooks::use_press_feedback::{PressFeedbackConfig, use_feedback_controller};
use crate::link::{LinkRenderProps, use_link_component};
use crate::utils::disabled_interaction::{cancel_disabled_click, cancel_disabled_key};
use crate::utils::safe_url::is_safe_url;
use crate::utils::text_direction::text_direction;

/// Open theme identifier for a token tone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenColor(Cow<'static, str>);

macro_rules! tones {
    ($($name:ident => $value:literal),+ $(,)?) => { $(
        #[doc = concat!("Built-in `", $value, "` tone.")]
        pub const $name: Self = Self(Cow::Borrowed($value));
    )+ };
}

impl TokenColor {
    tones!(DEFAULT => "default", RED => "red", ORANGE => "orange", YELLOW => "yellow",
        GREEN => "green", TEAL => "teal", CYAN => "cyan", BLUE => "blue",
        PURPLE => "purple", PINK => "pink", GRAY => "gray");

    /// Validate a custom identifier without changing its spelling.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidTokenColor> {
        let value = value.into();
        if value.is_empty()
            || !value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_'))
        {
            return Err(InvalidTokenColor);
        }
        Ok(Self(Cow::Owned(value)))
    }

    /// Identifier reflected on the token.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for TokenColor {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// A token color was not a valid theme identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidTokenColor;
impl Error for InvalidTokenColor {}
impl Display for InvalidTokenColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Token color must be a nonempty ASCII identifier containing only letters, digits, hyphens, or underscores")
    }
}

/// Token height on the element size scale.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TokenSize {
    /// Small.
    Small,
    /// Medium.
    #[default]
    Medium,
    /// Large.
    Large,
}

/// An inline tag with optional navigation, activation, or a separate Remove button.
#[component]
pub fn Token(
    /// Visible and accessible text.
    label: String,
    /// Token height.
    #[props(default)]
    size: TokenSize,
    /// Theme tone.
    #[props(default)]
    color: TokenColor,
    /// Leading artwork.
    #[props(default)]
    icon: Option<Element>,
    /// Dim and disable actions without removing their tab stops.
    #[props(default)]
    is_disabled: bool,
    /// Separate Remove action.
    #[props(default)]
    on_remove: Option<EventHandler<MouseEvent>>,
    /// Primary action, or click callback for an enabled link.
    #[props(default)]
    on_click: Option<EventHandler<MouseEvent>>,
    /// Navigation destination.
    #[props(default)]
    href: Option<String>,
    /// Accessible description.
    #[props(default)]
    description: Option<String>,
    /// Content between the label and Remove.
    #[props(default)]
    end_content: Option<Element>,
    /// Hide the label visually, but retain its accessible name.
    #[props(default)]
    is_label_hidden: bool,
    /// Override the full localized Remove name.
    #[props(default)]
    remove_label: Option<String>,
    /// Link target.
    #[props(default)]
    target: Option<String>,
    /// Link relationship.
    #[props(default)]
    rel: Option<String>,
    /// Root classes.
    #[props(default)]
    class: String,
    /// Root inline CSS.
    #[props(default)]
    style: String,
    /// Root mounted callback.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native attributes; naming and popup relationships are routed to a primary button.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let id = format!("token-{}", current_scope_id().0);
    let is_rejected = href.as_deref().is_some_and(|url| !is_safe_url(url));
    let role = use_interactive_role(href.is_some(), on_click.is_some(), is_disabled);
    let is_link = href.is_some() && (role == InteractiveRole::Link || on_click.is_none());
    let is_button = !is_link && role == InteractiveRole::Button;
    let is_blocked = is_disabled || is_rejected;
    let has_surface = is_button || (is_link && on_remove.is_some());
    let is_interactive = (is_link || is_button) && !is_blocked;
    let renderer = use_link_component(None);
    let destination = href.filter(|_| is_interactive && is_link);
    let surface = use_token_surface(
        id.clone(),
        has_surface && is_interactive,
        destination.clone(),
    );
    let press = use_feedback_controller(
        PressFeedbackConfig {
            id: id.clone(),
            label: label.clone(),
            tooltip: None,
            is_icon_only: false,
            is_loading: false,
            is_disabled: !is_interactive,
        },
        false,
        is_interactive,
    );
    let mounted = EventHandler::new(move |event: MountedEvent| {
        surface.call(event.clone());
        press.call(event.clone());
        if let Some(callback) = onmounted {
            callback.call(event);
        }
    });
    let mut primary_attributes = Vec::new();
    let mut root_attributes: Vec<_> = attributes
        .into_iter()
        .filter_map(|attr| {
            if matches!(
                attr.name,
                "class"
                    | "style"
                    | "data-color"
                    | "data-size"
                    | "data-disabled"
                    | "data-astryx-token-id"
                    | "data-astryx-feedback-id"
                    | "onmounted"
            ) {
                return None;
            }
            if is_button
                && matches!(
                    attr.name,
                    "aria-labelledby" | "aria-describedby" | "aria-haspopup" | "aria-expanded"
                )
            {
                primary_attributes.push(attr);
                return None;
            }
            Some(attr)
        })
        .collect();
    let root_class = format!("astryx-token {class}");
    let size = match size {
        TokenSize::Small => "sm",
        TokenSize::Medium => "md",
        TokenSize::Large => "lg",
    };
    root_attributes.extend([
        Attribute::new("class", root_class, None, false),
        Attribute::new("data-color", color.as_str(), None, false),
        Attribute::new("data-size", size, None, false),
        Attribute::new("data-astryx-token-id", id.clone(), None, false),
        Attribute::new("data-astryx-feedback-id", id, None, false),
    ]);
    for (key, value) in [
        ("style", (!style.is_empty()).then_some(style)),
        ("data-disabled", is_disabled.then(|| "disabled".into())),
        ("data-interactive", is_interactive.then(|| "true".into())),
        (
            "data-astryx-container-press",
            is_interactive.then(|| "true".into()),
        ),
        ("aria-description", description),
    ] {
        if let Some(value) = value {
            root_attributes.push(Attribute::new(key, value, None, false));
        }
    }
    let remove_name = remove_label.unwrap_or_else(|| format!("Remove {label}"));
    let label_node = rsx! { span { class: if is_label_hidden { "astryx-token__label astryx-token__label--hidden" } else { "astryx-token__label" }, dir: text_direction(&label), "{label}" } };
    let click = EventHandler::new(move |event: MouseEvent| {
        if is_blocked {
            cancel_disabled_click(&event);
        } else if let Some(callback) = on_click {
            callback.call(event);
        }
    });
    let keydown = EventHandler::new(move |event: KeyboardEvent| {
        if is_blocked {
            cancel_disabled_key(&event);
        }
    });
    let remove = rsx! {
        if let Some(callback) = on_remove {
            button { r#type: "button", class: "astryx-token__remove", "aria-label": remove_name, "aria-disabled": is_disabled.then_some("true"),
                onkeydown: move |event| { if is_disabled { cancel_disabled_key(&event); } },
                onclick: move |event: MouseEvent| { event.stop_propagation(); if is_disabled { cancel_disabled_click(&event); } else { callback.call(event); } },
                svg { width: "12", height: "12", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", "aria-hidden": "true", path { d: "m6 6 12 12M18 6 6 18" } }
            }
        }
    };
    if is_link && !has_surface {
        let children = rsx! { {icon} {label_node} {end_content} };
        if let Some(renderer) = renderer.filter(|_| destination.is_some()) {
            return renderer.call(LinkRenderProps {
                href: destination,
                target,
                rel,
                attributes: root_attributes,
                onclick: click,
                onkeydown: keydown,
                onmounted: mounted,
                children,
            });
        }
        return rsx! { a { href: destination, target, rel, role: is_blocked.then_some("link"), tabindex: is_blocked.then_some("0"), "aria-disabled": is_blocked.then_some("true"), onclick: click, onkeydown: keydown, onmounted: mounted, ..root_attributes, {children} } };
    }
    let action = if is_link {
        if let Some(renderer) = renderer.filter(|_| destination.is_some()) {
            renderer.call(LinkRenderProps {
                href: destination,
                target,
                rel,
                attributes: vec![
                    Attribute::new("class", "astryx-token__action", None, false),
                    Attribute::new("data-astryx-container-action", "true", None, false),
                ],
                onclick: click,
                onkeydown: keydown,
                onmounted: EventHandler::default(),
                children: label_node,
            })
        } else {
            rsx! { a { class: "astryx-token__action", "data-astryx-container-action": "true", href: destination, target, rel, role: is_blocked.then_some("link"), tabindex: is_blocked.then_some("0"), "aria-disabled": is_blocked.then_some("true"), onclick: click, onkeydown: keydown, {label_node} } }
        }
    } else if is_button {
        rsx! { button { r#type: "button", class: "astryx-token__action", "data-astryx-container-action": "true", "aria-disabled": is_blocked.then_some("true"), onclick: click, onkeydown: keydown, ..primary_attributes, {label_node} } }
    } else {
        label_node
    };
    rsx! { span { onmounted: mounted, ..root_attributes, {icon} {action} {end_content} {remove} } }
}
