use dioxus::prelude::*;

use super::chat_message::{ChatDensity, ChatMessageSender};

#[derive(Clone, Copy)]
pub(super) struct ChatMessageContext(pub(super) Signal<(ChatMessageSender, ChatDensity)>);
