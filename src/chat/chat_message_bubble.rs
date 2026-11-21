use dioxus::prelude::*;

use super::chat_message::{ChatDensity, ChatMessageSender};
use super::chat_message_context::ChatMessageContext;
use crate::SizeValue;
use crate::utils::is_renderable::is_renderable;

/// Paint treatment of a message bubble.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ChatMessageBubbleVariant {
    /// Neutral background with block and inline padding.
    #[default]
    Filled,
    /// Transparent background with only inline padding.
    Ghost,
}

/// Position in a run of bubbles from the same sender.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChatMessageBubbleGroup {
    First,
    Middle,
    Last,
}

/// A message surface with optional sibling name and metadata aligned to its text.
#[component]
pub fn ChatMessageBubble(
    children: Element,
    #[props(default)] variant: ChatMessageBubbleVariant,
    #[props(default)] name: Option<Element>,
    #[props(default)] metadata: Option<Element>,
    #[props(default)] group: Option<ChatMessageBubbleGroup>,
    /// Replaces the default width cap.
    #[props(default)]
    width: Option<SizeValue>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let (sender, density) = try_consume_context::<ChatMessageContext>().map_or(
        (ChatMessageSender::Assistant, ChatDensity::Balanced),
        |ctx| (ctx.0)(),
    );
    let sender = match sender {
        ChatMessageSender::Assistant => "assistant",
        ChatMessageSender::User => "user",
        ChatMessageSender::System => "system",
    };
    let density = match density {
        ChatDensity::Compact => "compact",
        ChatDensity::Balanced => "balanced",
        ChatDensity::Spacious => "spacious",
    };
    let variant = match variant {
        ChatMessageBubbleVariant::Filled => "filled",
        ChatMessageBubbleVariant::Ghost => "ghost",
    };
    let group = group.map(|group| match group {
        ChatMessageBubbleGroup::First => "first",
        ChatMessageBubbleGroup::Middle => "middle",
        ChatMessageBubbleGroup::Last => "last",
    });
    let style = match width {
        Some(width) => format!("width:{};max-width:none;{style}", width.as_css()?),
        None => style,
    };
    let attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class" | "style" | "data-sender" | "data-variant" | "data-density" | "data-group"
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        if let Some(name) = name.filter(is_renderable) {
            div { class: "astryx-chat-message-bubble__name", "data-chat-name": "true", "data-density": density, "data-sender": sender, {name} }
        }
        div { class: "astryx-chat-message-bubble {class}", style, "data-sender": sender, "data-density": density,
            "data-variant": variant, "data-group": group, ..attributes, {children}
        }
        if let Some(metadata) = metadata.filter(is_renderable) {
            div { class: "astryx-chat-message-bubble__metadata", "data-density": density, "data-sender": sender, {metadata} }
        }
    }
}
