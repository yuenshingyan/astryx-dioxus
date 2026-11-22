use dioxus::prelude::*;

use super::chat_composer_token::ChatComposerToken;
use crate::Badge;

/// An atomic inline composer token with a caller-supplied serialized value.
#[component]
pub fn ChatComposerTokenElement(
    token: ChatComposerToken,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let value = token.value().to_owned();
    let body = match token {
        ChatComposerToken::Badge {
            label,
            variant,
            icon,
            ..
        } => rsx! { Badge { label, variant, icon } },
        ChatComposerToken::Custom { render, .. } => render.call(()),
    };
    let attributes = attributes
        .into_iter()
        .filter(|attr| {
            !matches!(
                attr.name.to_ascii_lowercase().as_str(),
                "class"
                    | "style"
                    | "contenteditable"
                    | "data-astryx-token"
                    | "data-astryx-token-value"
            )
        })
        .collect::<Vec<_>>();
    rsx! {
        span { class: format!("astryx-chat-composer-token-element {class}"), style,
            "data-astryx-token": "", "data-astryx-token-value": value, contenteditable: "false",
            ..attributes, {body}
        }
    }
}
