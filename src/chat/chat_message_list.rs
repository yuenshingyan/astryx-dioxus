use dioxus::prelude::*;

use super::chat_message::ChatDensity;
use crate::SpacingStep;
use crate::layout::padding::spacing_step_css;
use crate::utils::is_renderable::is_renderable;

/// Resting block alignment when a transcript does not fill its container.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ChatMessageListAlign {
    Top,
    #[default]
    Bottom,
}

/// A focusable transcript log. The enclosing surface owns scrolling.
#[component]
pub fn ChatMessageList(
    /// Omit to show the optional empty state.
    #[props(default)]
    children: Option<Element>,
    #[props(default)] empty_state: Option<Element>,
    #[props(default)] density: ChatDensity,
    /// Overrides row spacing without affecting insets.
    #[props(default)]
    gap: Option<SpacingStep>,
    #[props(default)] align: ChatMessageListAlign,
    /// Exposes the log as busy while an existing message is streaming.
    #[props(default)]
    is_streaming: bool,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let density = match density {
        ChatDensity::Compact => "compact",
        ChatDensity::Balanced => "balanced",
        ChatDensity::Spacious => "spacious",
    };
    let gap = gap.map(|gap| format!("gap:{};", spacing_step_css(gap)));
    let attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class"
                    | "style"
                    | "data-density"
                    | "role"
                    | "aria-live"
                    | "aria-busy"
                    | "tabindex"
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        div { class: "astryx-chat-message-list {class}", style, "data-density": density,
            role: "log", "aria-live": "polite", "aria-busy": is_streaming.then_some("true"), tabindex: "0", ..attributes,
            div { class: "astryx-chat-message-list__inner", style: gap,
                if align == ChatMessageListAlign::Bottom {
                    div { class: "astryx-chat-message-list__spacer", "aria-hidden": "true" }
                }
                if let Some(children) = children {
                    {children}
                } else if let Some(empty_state) = empty_state.filter(is_renderable) {
                    div { class: "astryx-chat-message-list__empty", {empty_state} }
                }
            }
        }
    }
}
