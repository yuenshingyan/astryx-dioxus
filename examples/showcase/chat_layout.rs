use dioxus::prelude::*;

use astryx_dioxus::{
    Button, ChatDensity, ChatLayout, ChatLayoutScrollAction, ChatMessage, ChatMessageBubble,
    ChatMessageList, ChatMessageSender,
};

#[component]
fn Transcript(is_panel: bool) -> Element {
    let mut count = use_signal(|| if is_panel { 2 } else { 12 });
    rsx! {
        div { style: "height:360px;display:flex;border:1px solid var(--color-border)",
            ChatLayout { id: if is_panel { "chat-layout-panel" } else { "chat-layout-main" },
                density: if is_panel { ChatDensity::Compact } else { ChatDensity::Balanced },
                scroll_button: if is_panel { ChatLayoutScrollAction::Hidden } else { ChatLayoutScrollAction::Default },
                composer: rsx! {
                    div { style: "display:flex;gap:8px;align-items:center;background:var(--color-background-surface)",
                        input { "aria-label": "Message draft", placeholder: "Write a message" }
                        Button { label: "Add demo reply", onclick: move |_| count += 1 }
                    }
                },
                ChatMessageList {
                    for index in 1..=count() {
                        ChatMessage { key: "{index}", sender: if index % 2 == 0 { ChatMessageSender::User } else { ChatMessageSender::Assistant },
                            ChatMessageBubble { "Demo message {index}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub(super) fn ChatLayoutExamples() -> Element {
    rsx! {
        section { id: "chat-layout-examples",
            h2 { "ChatLayout" }
            p { "A native input and demo reply action stand in for ChatComposer." }
            h3 { "Showcase" }
            Transcript { is_panel: false }
            h3 { "Panel chat" }
            div { style: "max-width:380px", Transcript { is_panel: true } }
        }
    }
}
