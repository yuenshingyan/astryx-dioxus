use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{ChatMessageMetadata, ChatMessageStatus};

#[test]
fn empty_metadata_has_no_row() {
    let html =
        render_element(rsx! { ChatMessageMetadata { timestamp: rsx! {}, footer: rsx! { "" } } });
    assert!(!html.contains("astryx-chat-message-metadata"), "{html}");
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
