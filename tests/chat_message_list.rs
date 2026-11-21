use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{ChatDensity, ChatMessageList, ChatMessageListAlign, SpacingStep};

#[test]
fn log_owns_semantics_and_reflects_streaming() {
    let html = render_element(rsx! {
        ChatMessageList { id: "transcript", "aria-label": "Conversation", role: "region", tabindex: "-1",
            is_streaming: true, density: ChatDensity::Compact, gap: SpacingStep::Zero, align: ChatMessageListAlign::Top,
            "Message"
        }
    });
    for expected in [
        "id=\"transcript\"",
        "aria-label=\"Conversation\"",
        "role=\"log\"",
        "tabindex=\"0\"",
        "aria-live=\"polite\"",
        "aria-busy=\"true\"",
        "gap:var(--spacing-0)",
        "data-density=\"compact\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(
        !html.contains("region") && !html.contains("__spacer"),
        "{html}"
    );
}

#[test]
fn empty_state_only_replaces_omitted_children() {
    let html = render_element(rsx! { ChatMessageList { empty_state: rsx! { "0" } } });
    assert!(
        html.contains(">0</div>") && html.contains("__spacer") && !html.contains("aria-busy"),
        "{html}"
    );
    let html =
        render_element(rsx! { ChatMessageList { empty_state: rsx! { "Empty" }, "Message" } });
    assert!(
        html.contains("Message") && !html.contains("Empty"),
        "{html}"
    );
}
