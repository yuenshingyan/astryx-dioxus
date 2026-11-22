use dioxus::prelude::*;

use astryx_dioxus::{BadgeVariant, ChatComposerToken, ChatComposerTokenElement};

#[component]
pub(super) fn ChatComposerTokenElementExamples() -> Element {
    rsx! {
        section { id: "chat-composer-token-element-examples",
            h2 { "ChatComposerTokenElement" }
            div { contenteditable: "true", "aria-label": "Editable token example",
                "Ask "
                ChatComposerTokenElement { token: ChatComposerToken::Badge { value: "@ada".into(), label: "Ada".into(), variant: BadgeVariant::BLUE, icon: None } }
                " about "
                ChatComposerTokenElement { token: ChatComposerToken::Custom { value: "#design".into(), render: Callback::new(|()| rsx! { strong { "Design" } }) } }
            }
        }
    }
}
