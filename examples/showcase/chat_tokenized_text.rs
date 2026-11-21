use dioxus::prelude::*;

use astryx_dioxus::{
    BadgeVariant, ChatComposerToken, ChatMessage, ChatMessageBubble, ChatMessageSender,
    ChatTokenizedText,
};

fn badge(value: &str, label: &str, variant: BadgeVariant) -> ChatComposerToken {
    ChatComposerToken::Badge {
        value: value.into(),
        label: label.into(),
        variant,
        icon: None,
    }
}

#[component]
fn TokenPreview(is_basic: bool) -> Element {
    let mut tokens = vec![badge("@cindy", "@Cindy", BadgeVariant::BLUE)];
    let text = if is_basic {
        tokens.push(badge("@alex", "@Alex", BadgeVariant::BLUE));
        "Assign @cindy and @alex as reviewers."
    } else {
        tokens.extend([
            badge("#bug", "#bug", BadgeVariant::RED),
            badge("#feat", "#feature", BadgeVariant::GREEN),
        ]);
        "@cindy filed #bug and #feat for the sprint"
    };
    rsx! {
        ChatMessage { sender: ChatMessageSender::User,
            ChatMessageBubble { div { ChatTokenizedText { tokens, text } } }
        }
    }
}

#[component]
pub(super) fn ChatTokenizedTextExamples() -> Element {
    rsx! {
        section { id: "chat-tokenized-text-examples",
            h2 { "ChatTokenizedText" }
            div { style: "max-width:440px",
                h3 { "Showcase" }
                TokenPreview { is_basic: false }
                h3 { "Basic" }
                TokenPreview { is_basic: true }
                h3 { "Colors" }
                TokenPreview { is_basic: false }
            }
        }
    }
}
