use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{ChatDensity, ChatMessage, ChatMessageMetadata, ChatMessageSender};

#[test]
fn user_article_owns_name_density_and_metadata_direction() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ChatMessage { sender: ChatMessageSender::User, density: ChatDensity::Compact,
                name: rsx! { "Alex" }, "aria-label": "Not the sender", id: "message",
                metadata: rsx! { ChatMessageMetadata { timestamp: rsx! { "2:30 PM" } } },
                "Body"
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "<article",
        "id=\"message\"",
        "data-density=\"compact\"",
        "aria-label=\"Message from user\"",
        "aria-labelledby=\"astryx-chat-message-name-",
        "data-user=\"true\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("Not the sender"));
}

#[test]
fn system_article_omits_identity_and_metadata_slots() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ChatMessage { sender: ChatMessageSender::System, name: rsx! { "Ignored name" },
                avatar: rsx! { "Ignored avatar" }, metadata: rsx! { "Ignored metadata" },
                "System message"
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(
        html.contains("System message") && html.contains("aria-label=\"Message from system\""),
        "{html}"
    );
    assert!(
        !html.contains("Ignored") && !html.contains("aria-labelledby"),
        "{html}"
    );
}
