use dioxus::prelude::*;

use astryx_dioxus::{
    Avatar, ChatDensity, ChatMessage, ChatMessageMetadata, ChatMessageSender, ChatMessageStatus,
};

#[component]
fn Bubble(children: Element, #[props(default)] is_ghost: bool) -> Element {
    rsx! {
        div { style: if is_ghost { "padding:8px 12px" } else { "padding:8px 12px;border-radius:12px;background:var(--color-background-muted)" }, {children} }
    }
}

#[component]
pub(super) fn ChatMessageExamples() -> Element {
    let mut is_user = use_signal(|| true);
    rsx! {
        section { id: "chat-message-examples",
            h2 { "ChatMessage" }
            p { "Static transcript previews use simple styled blocks in place of ChatMessageBubble." }
            div { style: "max-width:500px;display:grid;gap:8px",
                h3 { "Showcase" }
                ChatMessage { sender: ChatMessageSender::User, id: "chat-message-user",
                    metadata: rsx! { ChatMessageMetadata { timestamp: rsx! { "2:30 PM" }, status: ChatMessageStatus::Read } },
                    Bubble { "Quick question about the PR." }
                    Bubble { "Can you review the token validation changes?" }
                }
                ChatMessage { sender: ChatMessageSender::Assistant,
                    metadata: rsx! { ChatMessageMetadata { timestamp: rsx! { "2:31 PM" }, footer: rsx! { "Claude Opus 4.6" } } },
                    Bubble { "Looks good — the error handling covers the edge cases." }
                }
                h3 { "Avatar and name" }
                ChatMessage { id: "chat-message-agent", sender: ChatMessageSender::Assistant, density: ChatDensity::Spacious,
                    name: rsx! { strong { "Agent" } }, avatar: rsx! { Avatar { name: "Agent" } },
                    Bubble { "The PR is ready for review." }
                }
                ChatMessage { sender: ChatMessageSender::User, name: rsx! { "Alex" }, Bubble { "Thanks! Merging it now." } }
                ChatMessage { sender: ChatMessageSender::Assistant, name: rsx! { "Agent" }, avatar: rsx! { Avatar { name: "Agent" } }, Bubble { "Let me know if you need anything else." } }
                h3 { "Ghost" }
                ChatMessage { sender: ChatMessageSender::Assistant, Bubble { is_ghost: true, "Here is an analysis of your production metrics from last week." } }
                ChatMessage { sender: ChatMessageSender::User, Bubble { "That looks great. Can you compare it to the week before?" } }
                ChatMessage { sender: ChatMessageSender::Assistant, Bubble { is_ghost: true, "Compared to the previous week, traffic is up 8%." } }
                h3 { "Multiple bubbles" }
                ChatMessage { sender: ChatMessageSender::User, density: ChatDensity::Compact,
                    Bubble { "I have two questions about the API." }
                    Bubble { "First, how should we handle pagination?" }
                    Bubble { "And second, what's the rate limit?" }
                }
                ChatMessage { sender: ChatMessageSender::Assistant, name: rsx! { "Agent" }, avatar: rsx! { Avatar { name: "Agent" } },
                    Bubble { "For pagination, use a cursor." }
                    Bubble { "The rate limit is 100 requests per minute." }
                }
                h3 { "Sender context" }
                button { id: "chat-message-toggle", onclick: move |_| is_user.toggle(), "Switch sender" }
                ChatMessage { id: "chat-message-reactive", sender: if is_user() { ChatMessageSender::User } else { ChatMessageSender::Assistant },
                    metadata: rsx! { ChatMessageMetadata { timestamp: rsx! { "2:30 PM" }, status: ChatMessageStatus::Sent } },
                    "Sender preview"
                }
                ChatMessage { id: "chat-message-system", sender: ChatMessageSender::System, "End of transcript" }
            }
        }
    }
}
