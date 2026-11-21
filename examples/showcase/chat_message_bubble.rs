use dioxus::prelude::*;

use astryx_dioxus::{
    Avatar, ChatDensity, ChatMessage, ChatMessageBubble, ChatMessageBubbleGroup,
    ChatMessageBubbleVariant, ChatMessageMetadata, ChatMessageSender, ChatMessageStatus,
};

#[component]
pub(super) fn ChatMessageBubbleExamples() -> Element {
    rsx! {
        section { id: "chat-message-bubble-examples",
            h2 { "ChatMessageBubble" }
            p { "Static transcript previews without ChatMessageList." }
            div { style: "width:min(100%, 500px);display:grid;gap:8px",
                h3 { "Showcase" }
                ChatMessage { sender: ChatMessageSender::User,
                    ChatMessageBubble { group: ChatMessageBubbleGroup::First, "Quick question." }
                    ChatMessageBubble { group: ChatMessageBubbleGroup::Last, metadata: rsx! { ChatMessageMetadata { status: ChatMessageStatus::Read } }, "Can you check the report?" }
                }
                ChatMessage { sender: ChatMessageSender::Assistant,
                    ChatMessageBubble { variant: ChatMessageBubbleVariant::Ghost, "The report is ready." }
                }
                h3 { "Variants" }
                ChatMessage { sender: ChatMessageSender::User, ChatMessageBubble { "Filled message" } }
                ChatMessage { sender: ChatMessageSender::Assistant, ChatMessageBubble { variant: ChatMessageBubbleVariant::Ghost, "Ghost message" } }
                h3 { "Grouping" }
                ChatMessage { sender: ChatMessageSender::Assistant, id: "chat-bubble-group", avatar: rsx! { Avatar { name: "Agent" } },
                    ChatMessageBubble { name: rsx! { "Agent" }, group: ChatMessageBubbleGroup::First, "First" }
                    ChatMessageBubble { id: "chat-bubble-middle", group: ChatMessageBubbleGroup::Middle, "Middle" }
                    ChatMessageBubble { group: ChatMessageBubbleGroup::Last, "Last" }
                }
                ChatMessage { sender: ChatMessageSender::User,
                    ChatMessageBubble { group: ChatMessageBubbleGroup::First, "Yes please!" }
                    ChatMessageBubble { group: ChatMessageBubbleGroup::Last, "Thanks." }
                }
                h3 { "Density" }
                for (density, label) in [(ChatDensity::Compact, "Compact"), (ChatDensity::Balanced, "Balanced"), (ChatDensity::Spacious, "Spacious")] {
                    div { key: "{label}",
                        ChatMessage { sender: ChatMessageSender::Assistant, density, ChatMessageBubble { "{label} assistant" } }
                        ChatMessage { sender: ChatMessageSender::User, density, ChatMessageBubble { "{label} user" } }
                    }
                }
                h3 { "Metadata" }
                ChatMessage { sender: ChatMessageSender::Assistant, avatar: rsx! { Avatar { name: "Agent" } },
                    ChatMessageBubble { name: rsx! { "Agent" }, metadata: rsx! { ChatMessageMetadata { timestamp: rsx! { "2:30 PM" }, footer: rsx! { "Claude Opus 4.6" } } }, "Here is the summary." }
                }
                ChatMessage { sender: ChatMessageSender::User,
                    ChatMessageBubble { metadata: rsx! { ChatMessageMetadata { status: ChatMessageStatus::Read } }, "Thank you!" }
                }
                h3 { "Custom content" }
                ChatMessage { sender: ChatMessageSender::User, ChatMessageBubble { "Show me the output." } }
                ChatMessage { sender: ChatMessageSender::Assistant,
                    ChatMessageBubble { id: "chat-bubble-wide", variant: ChatMessageBubbleVariant::Ghost, width: Some("100%".into()),
                        div { style: "padding:12px;border:1px solid var(--color-text-secondary)", "Report artifact" }
                    }
                }
            }
        }
    }
}
