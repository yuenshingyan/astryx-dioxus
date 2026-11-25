use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    ChatMessageMetadata, ChatMessageStatus, InternationalizationProvider, MessagesByLocale,
};

#[test]
fn empty_metadata_has_no_row() {
    let html =
        render_element(rsx! { ChatMessageMetadata { timestamp: rsx! {}, footer: rsx! { "" } } });
    assert!(!html.contains("astryx-chat-message-metadata"), "{html}");
}

#[test]
fn status_caption_uses_catalog_unless_explicitly_overridden() {
    let messages = MessagesByLocale::from([(
        "fr".into(),
        [("@astryx.chat.status.read".into(), "Lu".into())].into(),
    )]);
    let html = render_element(rsx! {
        InternationalizationProvider { locale: "fr-CA", messages,
            ChatMessageMetadata { status: ChatMessageStatus::Read }
            ChatMessageMetadata { status: ChatMessageStatus::Read, status_label: "Vu" }
            ChatMessageMetadata { status: ChatMessageStatus::Sent }
        }
    });
    for caption in ["Lu", "Vu", "Sent"] {
        assert!(html.contains(&format!("title=\"{caption}\"")), "{html}");
        assert!(html.contains(&format!(">{caption}</span>")), "{html}");
    }
}

#[test]
fn separates_present_slots_and_preserves_dom_order() {
    let html = render_element(rsx! {
        ChatMessageMetadata { id: "detail", is_user: true, timestamp: rsx! { "0" },
            footer: rsx! { button { "Retry" } }, status: ChatMessageStatus::Error, status_label: "Not sent",
        }
    });
    assert_eq!(html.matches('·').count(), 2, "{html}");
    assert!(
        html.contains("<span>0</span>")
            && html.contains("id=\"detail\"")
            && html.contains("data-user=\"true\""),
        "{html}"
    );
    assert!(html.find("Retry").unwrap() < html.find("Not sent").unwrap());
    assert!(
        !html.contains("aria-live") && !html.contains("aria-label"),
        "{html}"
    );
}
