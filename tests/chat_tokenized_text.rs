use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{BadgeVariant, ChatComposerToken, ChatTokenizedText};

fn badge(value: &str, label: &str) -> ChatComposerToken {
    ChatComposerToken::Badge {
        value: value.into(),
        label: label.into(),
        variant: BadgeVariant::BLUE,
        icon: None,
    }
}

#[test]
fn literal_matches_respect_order_and_last_duplicate_definition() {
    let mut dom = VirtualDom::new(|| {
        let tokens = vec![
            badge("", "Ignored"),
            badge("a+", "Old"),
            badge("a+b", "Long"),
            badge("a+", "Short"),
        ];
        rsx! { ChatTokenizedText { tokens, id: "tokens", text: "🙂 a+b a+" } }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert_eq!(html.matches("astryx-chat-tokenized-text__token").count(), 2);
    assert!(
        html.contains("🙂 ") && html.contains("</span>b ") && html.contains("Short"),
        "{html}"
    );
    assert!(!html.contains("Ignored") && !html.contains("Old") && !html.contains("Long"));
}

#[test]
fn repeated_custom_tokens_preserve_surrounding_text() {
    let mut dom = VirtualDom::new(|| {
        let tokens = vec![ChatComposerToken::Custom {
            value: "猫".into(),
            render: Callback::new(|()| rsx! { em { "Cat" } }),
        }];
        rsx! { ChatTokenizedText { tokens, text: "猫 & 猫" } }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert_eq!(html.matches("<em>Cat</em>").count(), 2);
    assert!(
        html.contains(" &#38; ") || html.contains(" &amp; "),
        "{html}"
    );
}
