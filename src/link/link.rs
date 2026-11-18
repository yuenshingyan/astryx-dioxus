// Adapted from Astryx Link/Link.tsx. See THIRD_PARTY_NOTICES.md.

use dioxus::core::{AttributeValue, current_scope_id};
use dioxus::prelude::*;

use super::compute_target_and_rel::compute_target_and_rel;
use super::types::{LinkComponentType, LinkRenderProps};
use super::use_link_component::use_link_component;
use crate::hooks::use_interactive_role::{InteractiveRole, use_interactive_role};
use crate::hooks::use_press_feedback::{PressFeedbackConfig, use_press_feedback};
use crate::icon::global_icon_registry::resolve_icon;
use crate::icon::{Icon, IconRequest, IconSize};
use crate::text::Text;
use crate::text::text_clamp_context::use_text_clamp_context;
use crate::theme::{TextColor, TextDisplay, TextSize, TextType, TextWeight};
use crate::tooltip::use_tooltip::tooltip_element;
use crate::utils::safe_url::is_safe_url;

/// Native download behavior, independent of Link's visual state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum LinkDownload {
    /// No download attribute.
    #[default]
    Disabled,
    /// Let the browser choose the filename.
    Enabled,
    /// Supply a suggested filename.
    FileName(String),
}

impl LinkDownload {
    fn attribute(self) -> Option<String> {
        match self {
            Self::Disabled => None,
            Self::Enabled => Some(String::new()),
            Self::FileName(name) => Some(name),
        }
    }
}

fn text_attribute(attributes: &[Attribute], name: &str) -> Option<String> {
    attributes.iter().rev().find_map(|attribute| {
        if attribute.name == name
            && let AttributeValue::Text(value) = &attribute.value
        {
            return Some(value.clone());
        }
        None
    })
}

fn external_glyph() -> Element {
    resolve_icon(
        "externalLink",
        IconRequest {
            size: IconSize::ExtraSmall,
            variant: None,
        },
        rsx! {
            svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round", "aria-hidden": "true",
                path { d: "M18 13v6a2 2 0 01-2 2H5a2 2 0 01-2-2V8a2 2 0 012-2h6" }
                path { d: "M15 3h6v6" } path { d: "M10 14L21 3" }
            }
        },
    )
}

/// Styled navigation or a native link-styled action. Typography remains Text-owned.
///
/// Enabled destinations select explicit `render_link`, nearest LinkProvider, then
/// a native anchor. Disabled navigation skips routers, stays focusable, and never
/// activates. A click handler without an enabled destination selects a button.
/// `onmounted` is the root-ref adapter. Pixel lengths and native values use strings.
#[component]
pub fn Link(
    children: Element,
    #[props(default)] href: Option<String>,
    #[props(default)] render_link: Option<LinkComponentType>,
    #[props(default)] label: Option<String>,
    #[props(default)] has_underline: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_external_link: bool,
    #[props(default = "(opens in new tab)".to_string())] new_tab_label: String,
    #[props(default)] target: Option<String>,
    #[props(default)] rel: Option<String>,
    #[props(default)] download: LinkDownload,
    #[props(default)] referrer_policy: Option<String>,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] onkeydown: EventHandler<KeyboardEvent>,
    #[props(default)] onmounted: EventHandler<MountedEvent>,
    #[props(default)] tooltip: Option<String>,
    #[props(default)] is_standalone: bool,
    #[props(default)] text_type: TextType,
    #[props(default)] size: Option<TextSize>,
    #[props(default)] weight: Option<TextWeight>,
    #[props(default = TextColor::ACCENT)] color: TextColor,
    #[props(default)] display: TextDisplay,
    #[props(default)] max_lines: u32,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let renderer = use_link_component(render_link);
    let role = use_interactive_role(href.is_some(), onclick.is_some(), is_disabled);
    let is_in_parent_clamp = use_text_clamp_context();
    let is_rejected = href.as_deref().is_some_and(|value| !is_safe_url(value));
    let is_blocked = is_disabled || is_rejected;
    let is_button = !is_rejected
        && (role == InteractiveRole::Button || (role == InteractiveRole::Inert && href.is_none()));
    let is_clamped =
        is_in_parent_clamp && display == TextDisplay::Inline && max_lines == 0 && !is_button;
    let is_external = is_external_link && !is_button;
    let color_name = color.as_str().to_string();
    let label_color = if [
        "primary",
        "secondary",
        "disabled",
        "placeholder",
        "accent",
        "inherit",
    ]
    .contains(&color.as_str())
    {
        TextColor::INHERIT
    } else {
        color.clone()
    };
    let tooltip = tooltip.filter(|value| !value.trim().is_empty());
    let id = use_hook(|| format!("astryx-link-{}", current_scope_id().0));
    let aria_label = text_attribute(&attributes, "aria-label")
        .or_else(|| label.filter(|value| !value.is_empty()));
    let is_same_tooltip = !attributes
        .iter()
        .any(|attribute| attribute.name == "aria-labelledby")
        && tooltip
            .as_deref()
            .zip(aria_label.as_deref())
            .is_some_and(|(tip, name)| tip.trim() == name.trim());
    let mut descriptions: Vec<String> = text_attribute(&attributes, "aria-describedby")
        .into_iter()
        .collect();
    if tooltip.is_some() && !is_same_tooltip {
        descriptions.push(format!("{id}-tooltip"));
    }
    let described_by = (!descriptions.is_empty()).then(|| descriptions.join(" "));
    let mounted = use_press_feedback(PressFeedbackConfig {
        id: id.clone(),
        label: aria_label.clone().unwrap_or_default(),
        tooltip: tooltip.clone(),
        is_icon_only: false,
        is_loading: false,
        is_disabled: is_blocked,
    });
    let handle_mounted = move |event: MountedEvent| {
        mounted.call(event.clone());
        onmounted.call(event);
    };
    let handle_click = move |event: MouseEvent| {
        if is_blocked {
            event.prevent_default();
            event.stop_propagation();
            return;
        }
        if let Some(handler) = onclick {
            handler.call(event);
        }
    };
    let handle_keydown = move |event: KeyboardEvent| {
        if is_blocked
            && (event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()))
        {
            event.prevent_default();
            event.stop_propagation();
            return;
        }
        onkeydown.call(event);
    };
    let has_tabindex = attributes
        .iter()
        .any(|attribute| attribute.name == "tabindex");
    let mut attributes: Vec<_> = attributes
        .into_iter()
        .filter(|attribute| {
            !matches!(
                attribute.name,
                "href"
                    | "target"
                    | "rel"
                    | "download"
                    | "referrerpolicy"
                    | "type"
                    | "class"
                    | "style"
                    | "onclick"
                    | "onkeydown"
                    | "onmounted"
                    | "aria-label"
                    | "aria-describedby"
                    | "data-color"
                    | "data-astryx-feedback-id"
                    | "data-astryx-pressable"
            ) && !(is_blocked && matches!(attribute.name, "role" | "aria-disabled"))
        })
        .collect();
    attributes.push(Attribute::new(
        "data-color",
        color_name.clone(),
        None,
        false,
    ));
    if let Some(value) = download.attribute() {
        attributes.push(Attribute::new("download", value, None, false));
    }
    if let Some(value) = referrer_policy {
        attributes.push(Attribute::new("referrerpolicy", value, None, false));
    }
    let classes = format!(
        "astryx-link {}{}{}{}{} {class}",
        color_name,
        if has_underline {
            " astryx-link--underline"
        } else {
            ""
        },
        if is_standalone {
            " astryx-link--standalone"
        } else {
            ""
        },
        if is_clamped {
            " astryx-link--in-clamp"
        } else {
            ""
        },
        if is_blocked {
            " astryx-link--disabled"
        } else {
            ""
        }
    );
    let styles = format!(
        "{}{}",
        if max_lines > 0 { "min-width:0;" } else { "" },
        style
    );
    let label_style = if label_color == TextColor::INHERIT {
        "color:inherit;min-width:0"
    } else {
        "min-width:0"
    };
    let content = rsx! {
        Text { class: "astryx-link__label", style: label_style, text_type, size, weight, color: label_color, display, max_lines,
            {children}
        }
        if is_external {
            Icon { size: IconSize::ExtraSmall, class: "astryx-link__external", icon: external_glyph() }
            span { class: "astryx-link__new-tab", "{new_tab_label}" }
        }
    };
    let tip = tooltip_element(&id, tooltip.as_deref());
    if is_button {
        return rsx! {
            button { class: classes, style: styles, r#type: "button", "aria-label": aria_label,
                "aria-describedby": described_by, "aria-disabled": is_blocked.then_some("true"),
                "data-astryx-feedback-id": id, "data-astryx-pressable": (!is_blocked).then_some(""),
                onclick: handle_click, onkeydown: handle_keydown, onmounted: handle_mounted,
                ..attributes, {content}
            }
            {tip}
        };
    }
    let (target, rel) = compute_target_and_rel(
        if is_external_link {
            Some("_blank".to_string())
        } else {
            target
        },
        rel,
    );
    if !is_blocked && let Some(renderer) = renderer {
        for (name, value) in [
            ("class", AttributeValue::Text(classes)),
            ("style", AttributeValue::Text(styles)),
            (
                "aria-label",
                aria_label
                    .map(AttributeValue::Text)
                    .unwrap_or(AttributeValue::None),
            ),
            (
                "aria-describedby",
                described_by
                    .map(AttributeValue::Text)
                    .unwrap_or(AttributeValue::None),
            ),
            ("data-astryx-feedback-id", AttributeValue::Text(id)),
            ("data-astryx-pressable", AttributeValue::Text(String::new())),
        ] {
            attributes.push(Attribute::new(name, value, None, false));
        }
        let link = renderer.call(LinkRenderProps {
            href,
            target,
            rel,
            attributes,
            onclick: EventHandler::new(handle_click),
            onkeydown: EventHandler::new(handle_keydown),
            onmounted: EventHandler::new(handle_mounted),
            children: content,
        });
        return rsx! { {link} {tip} };
    }
    rsx! {
        a { class: classes, style: styles, href: href.filter(|_| !is_blocked), target: target.filter(|_| !is_blocked), rel: rel.filter(|_| !is_blocked),
            role: is_blocked.then_some("link"), tabindex: (is_blocked && !has_tabindex).then_some(0),
            "aria-label": aria_label, "aria-describedby": described_by, "aria-disabled": is_blocked.then_some("true"),
            "data-astryx-feedback-id": id, "data-astryx-pressable": (!is_blocked).then_some(""),
            onclick: handle_click, onkeydown: handle_keydown, onmounted: handle_mounted,
            ..attributes, {content}
        }
        {tip}
    }
}
