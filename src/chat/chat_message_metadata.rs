use dioxus::prelude::*;

use super::chat_message::ChatMessageSender;
use super::chat_message_context::ChatMessageContext;
use crate::i18n::use_message;
use crate::utils::is_renderable::is_renderable;
use crate::{Icon, IconSize};

/// Delivery state displayed beside a chat message.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChatMessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Error,
}

impl ChatMessageStatus {
    fn message_key(self) -> &'static str {
        match self {
            Self::Sending => "@astryx.chat.status.sending",
            Self::Sent => "@astryx.chat.status.sent",
            Self::Delivered => "@astryx.chat.status.delivered",
            Self::Read => "@astryx.chat.status.read",
            Self::Error => "@astryx.chat.status.failed",
        }
    }

    fn presentation(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Sending => (
                "sending",
                "Sending",
                "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0ZM12 7v5l3 2",
            ),
            Self::Sent => ("sent", "Sent", "m5 12 4 4L19 6"),
            Self::Delivered => ("delivered", "Delivered", "m2 12 4 4L16 6m-5 8 2 2L23 6"),
            Self::Read => ("read", "Read", "m2 12 4 4L16 6m-5 8 2 2L23 6"),
            Self::Error => (
                "error",
                "Failed",
                "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0ZM12 7v6m0 3h.01",
            ),
        }
    }
}

/// Optional timestamp, footer and delivery status. This row does not announce changes.
#[component]
pub fn ChatMessageMetadata(
    #[props(default)] timestamp: Option<Element>,
    #[props(default)] footer: Option<Element>,
    #[props(default)] status: Option<ChatMessageStatus>,
    /// Localized visible caption and title for the current delivery status.
    #[props(default)]
    status_label: Option<String>,
    /// Reverses the visual row outside ChatMessage, where no sender is inherited.
    #[props(default)]
    is_user: bool,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let is_user = try_consume_context::<ChatMessageContext>().map_or(is_user, |context| {
        (context.0)().0 == ChatMessageSender::User
    });
    let timestamp = timestamp.filter(is_renderable);
    let footer = footer.filter(is_renderable);
    if timestamp.is_none() && footer.is_none() && status.is_none() {
        return rsx! {};
    }
    let attributes = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style" | "data-user"))
        .collect::<Vec<_>>();
    rsx! {
        div { class: "astryx-chat-message-metadata {class}", style, "data-user": is_user.then_some("true"), ..attributes,
            if let Some(timestamp) = timestamp {
                span { {timestamp} }
                if footer.is_some() || status.is_some() { span { "·" } }
            }
            if let Some(footer) = footer {
                {footer}
                if status.is_some() { span { "·" } }
            }
            if let Some(status) = status {
                {
                    let (name, fallback, path_data) = status.presentation();
                    let label = status_label.filter(|label| !label.trim().is_empty())
                        .unwrap_or_else(|| use_message(status.message_key(), fallback));
                    rsx! {
                        span { class: "astryx-chat-message-metadata__status", "data-status": name, title: label,
                            Icon { size: IconSize::ExtraSmall, icon: rsx! {
                                svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linejoin: "round", stroke_linecap: "round",
                                    path { d: path_data }
                                }
                            } }
                            span { "{label}" }
                        }
                    }
                }
            }
        }
    }
}
