use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use super::chat_message_context::ChatMessageContext;
use crate::utils::is_renderable::is_renderable;

/// Sender determines message alignment and the default accessible name.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChatMessageSender {
    Assistant,
    User,
    System,
}

/// Spacing between the avatar and body and between body children.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ChatDensity {
    Compact,
    #[default]
    Balanced,
    Spacious,
}

/// An article that groups one sender's message content and metadata.
#[component]
pub fn ChatMessage(
    sender: ChatMessageSender,
    children: Element,
    #[props(default)] avatar: Option<Element>,
    #[props(default)] name: Option<Element>,
    #[props(default)] metadata: Option<Element>,
    #[props(default)] density: ChatDensity,
    /// Localized accessible fallback when no visible sender name is available.
    #[props(default)]
    label: Option<String>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut context = use_context_provider(|| ChatMessageContext(Signal::new((sender, density))));
    if *context.0.peek() != (sender, density) {
        context.0.set((sender, density));
    }
    let name_id = use_hook(|| format!("astryx-chat-message-name-{}", current_scope_id().0));
    let (sender_name, fallback_label) = match sender {
        ChatMessageSender::Assistant => ("assistant", "Message from assistant"),
        ChatMessageSender::User => ("user", "Message from user"),
        ChatMessageSender::System => ("system", "Message from system"),
    };
    let density_name = match density {
        ChatDensity::Compact => "compact",
        ChatDensity::Balanced => "balanced",
        ChatDensity::Spacious => "spacious",
    };
    let has_slots = sender != ChatMessageSender::System;
    let avatar = avatar.filter(|slot| has_slots && is_renderable(slot));
    let name = name.filter(|slot| has_slots && is_renderable(slot));
    let labelled_by = name.as_ref().map(|_| name_id.clone());
    let metadata = metadata.filter(|slot| has_slots && is_renderable(slot));
    let attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name,
                "class"
                    | "style"
                    | "data-sender"
                    | "data-density"
                    | "aria-label"
                    | "aria-labelledby"
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        article { class: "astryx-chat-message {class}", style,
            "data-sender": sender_name, "data-density": density_name,
            "aria-label": label.as_deref().unwrap_or(fallback_label),
            "aria-labelledby": labelled_by,
            ..attributes,
            if let Some(avatar) = avatar { div { class: "astryx-chat-message__avatar", {avatar} } }
            div { class: "astryx-chat-message__column",
                if let Some(name) = name { div { id: name_id, class: "astryx-chat-message__name", {name} } }
                div { class: "astryx-chat-message__body", {children} }
                if let Some(metadata) = metadata { div { class: "astryx-chat-message__metadata", {metadata} } }
            }
        }
    }
}
