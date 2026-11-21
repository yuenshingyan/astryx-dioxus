use std::collections::HashMap;

use dioxus::prelude::*;

use super::chat_composer_token::ChatComposerToken;
use crate::Badge;

/// Renders serialized message text with literal token replacements.
#[component]
pub fn ChatTokenizedText(
    /// Serialized plain text; token values are matched literally.
    text: String,
    #[props(default)] tokens: Vec<ChatComposerToken>,
    #[props(default)] class: String,
    #[props(default)] style: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let parts = tokenize(&text, &tokens);
    let attributes = attributes
        .into_iter()
        .filter(|attr| !matches!(attr.name, "class" | "style"))
        .collect::<Vec<_>>();
    rsx! {
        span { class: "astryx-chat-tokenized-text {class}", style, ..attributes,
            for part in parts {
                match part {
                    Part::Text(text) => rsx! { "{text}" },
                    Part::Token(offset, token) => rsx! {
                        span { key: "{token.value()}-{offset}", class: "astryx-chat-tokenized-text__token",
                            match token {
                                ChatComposerToken::Badge { label, variant, icon, .. } => rsx! {
                                    Badge { label: label.clone(), variant: variant.clone(), icon: icon.clone() }
                                },
                                ChatComposerToken::Custom { render, .. } => render.call(()),
                            }
                        }
                    },
                }
            }
        }
    }
}

enum Part<'a> {
    Text(&'a str),
    Token(usize, &'a ChatComposerToken),
}

fn tokenize<'a>(text: &'a str, tokens: &'a [ChatComposerToken]) -> Vec<Part<'a>> {
    let mut definitions = HashMap::new();
    let mut by_first = HashMap::<u8, Vec<&str>>::new();
    for token in tokens {
        let value = token.value();
        if let Some(&first) = value.as_bytes().first()
            && definitions.insert(value, token).is_none()
        {
            by_first.entry(first).or_default().push(value);
        }
    }
    let mut parts = Vec::new();
    let mut start = 0;
    for (offset, _) in text.char_indices() {
        if offset < start {
            continue;
        }
        let Some(value) = by_first.get(&text.as_bytes()[offset]).and_then(|values| {
            values
                .iter()
                .find(|&&value| text[offset..].starts_with(value))
        }) else {
            continue;
        };
        if offset > start {
            parts.push(Part::Text(&text[start..offset]));
        }
        parts.push(Part::Token(offset, definitions[value]));
        start = offset + value.len();
    }
    if start < text.len() {
        parts.push(Part::Text(&text[start..]));
    }
    parts
}
