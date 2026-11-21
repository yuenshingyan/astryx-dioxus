use dioxus::prelude::*;

use astryx_dioxus::{
    ChatDensity, ChatMessage, ChatMessageBubble, ChatMessageList, ChatMessageListAlign,
    ChatMessageMetadata, ChatMessageSender, ChatMessageStatus, ChatSystemMessage,
    ChatSystemMessageVariant, SpacingStep,
};

#[component]
fn Transcript(density: ChatDensity) -> Element {
    rsx! {
        ChatMessageList { density,
            ChatSystemMessage { variant: ChatSystemMessageVariant::Divider, "Today" }
            ChatMessage { density, sender: ChatMessageSender::User,
                ChatMessageBubble { metadata: rsx! { ChatMessageMetadata { timestamp: rsx! { "2:30 PM" }, status: ChatMessageStatus::Read } }, "How is the report looking?" }
            }
            ChatMessage { density, sender: ChatMessageSender::Assistant, ChatMessageBubble { "The draft is ready to review." } }
            ChatMessage { density, sender: ChatMessageSender::User, ChatMessageBubble { "Thanks!" } }
        }
    }
}

#[component]
pub(super) fn ChatMessageListExamples() -> Element {
    let mut is_streaming = use_signal(|| false);
    rsx! {
        section { id: "chat-message-list-examples",
            h2 { "ChatMessageList" }
            div { style: "max-width:500px",
                h3 { "Showcase" }
                Transcript { density: ChatDensity::Balanced }
                h3 { "Density" }
                Transcript { density: ChatDensity::Compact }
                Transcript { density: ChatDensity::Spacious }
                h3 { "Full featured" }
                p { "Plain text and static timestamps stand in for Markdown and Timestamp." }
                ChatMessageList {
                    ChatSystemMessage { variant: ChatSystemMessageVariant::Divider, "Today" }
                    ChatMessage { sender: ChatMessageSender::User,
                        div { "q1-report.pdf" }
                        ChatMessageBubble { "Summarize the Q1 report." }
                    }
                    ChatMessage { sender: ChatMessageSender::Assistant,
                        ChatMessageBubble { "Revenue grew 12% year over year." }
                        ChatMessageBubble { metadata: rsx! { ChatMessageMetadata { timestamp: rsx! { "2:31 PM" }, footer: rsx! { "Model summary" } } }, "The full breakdown is in the report." }
                    }
                    ChatSystemMessage { "Conversation marked as resolved" }
                }
                h3 { "Alignment and streaming" }
                button { id: "chat-list-toggle", onclick: move |_| is_streaming.toggle(), "Toggle streaming" }
                div { style: "display:flex;height:120px;overflow:auto",
                    ChatMessageList { id: "chat-list-top", align: ChatMessageListAlign::Top, gap: SpacingStep::One, is_streaming: is_streaming(), "Starting at the top" }
                }
                div { style: "display:flex;height:120px;overflow:auto",
                    ChatMessageList { id: "chat-list-bottom", gap: SpacingStep::One, "Resting at the bottom" }
                }
                ChatMessageList { id: "chat-list-empty", empty_state: rsx! { "No messages yet" } }
            }
        }
    }
}
