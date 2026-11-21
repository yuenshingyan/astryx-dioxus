use dioxus::prelude::*;

use astryx_dioxus::{ChatDictationButton, ChatDictationButtonSize, ChatDictationState};

#[component]
fn DictationPreview(is_composer: bool) -> Element {
    let mut is_listening = use_signal(|| false);
    let dictation = if is_listening() {
        ChatDictationState::Listening {
            bands: [0.0, 0.02, 0.12, 0.2, 0.06],
            volume: 0.25,
        }
    } else {
        ChatDictationState::Idle
    };
    rsx! {
        div { style: "display:flex;gap:8px;align-items:center",
            if is_composer {
                input { "aria-label": "Message draft", placeholder: "Write a message" }
            }
            ChatDictationButton {
                id: if is_composer { "dictation-composer" } else { "dictation-basic" },
                size: if is_composer { ChatDictationButtonSize::Medium } else { ChatDictationButtonSize::Small },
                dictation, on_toggle: move |_| is_listening.toggle(),
            }
        }
    }
}

#[component]
pub(super) fn ChatDictationButtonExamples() -> Element {
    rsx! {
        section { id: "chat-dictation-button-examples",
            h2 { "ChatDictationButton" }
            p { "These previews simulate listening with fixed meter samples. They do not access your microphone." }
            h3 { "Basic" }
            DictationPreview { is_composer: false }
            h3 { "Showcase" }
            DictationPreview { is_composer: true }
            h3 { "Unsupported" }
            ChatDictationButton { id: "dictation-unsupported", dictation: ChatDictationState::Unsupported, is_hidden_when_unsupported: false, on_toggle: |_| {} }
        }
    }
}
