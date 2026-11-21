use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{ChatSystemMessage, ChatSystemMessageVariant};

#[test]
fn notice_renders_icon_first_without_adding_live_semantics() {
    let html = render_element(rsx! {
        ChatSystemMessage { id: "notice", class: "custom", icon: rsx! { b { "Glyph" } }, "Conversation started" }
    });
    assert!(html.contains("id=\"notice\"") && html.contains("custom"));
    assert!(html.find("Glyph").unwrap() < html.find("Conversation started").unwrap());
    assert!(!html.contains("role=") && !html.contains("aria-live"));
    let empty_icon = render_element(rsx! { ChatSystemMessage { icon: rsx! {}, "Notice" } });
    assert!(!empty_icon.contains("astryx-chat-system-message__icon"));
}

#[test]
fn divider_delegates_semantics_and_ignores_icon() {
    let html = render_element(rsx! {
        ChatSystemMessage { variant: ChatSystemMessageVariant::Divider, "data-probe": "date",
            icon: rsx! { "Ignored glyph" }, strong { "Today" }
        }
    });
    for expected in [
        "data-probe=\"date\"",
        "role=\"separator\"",
        "aria-labelledby=",
        "<strong>Today</strong>",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("Ignored glyph") && !html.contains("aria-live"));
}
