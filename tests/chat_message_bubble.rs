use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{
    ChatDensity, ChatMessage, ChatMessageBubble, ChatMessageBubbleGroup, ChatMessageBubbleVariant,
    ChatMessageSender,
};

#[test]
fn bubble_slots_are_siblings_and_surface_inherits_message_context() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ChatMessage { sender: ChatMessageSender::User, density: ChatDensity::Spacious,
                ChatMessageBubble { id: "surface", group: ChatMessageBubbleGroup::Last,
                    variant: ChatMessageBubbleVariant::Ghost, width: Some("100%".into()),
                    name: rsx! { "Author" }, metadata: rsx! { "0" }, "Body"
                }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "data-chat-name=\"true\"",
        "data-sender=\"user\"",
        "data-density=\"spacious\"",
        "data-variant=\"ghost\"",
        "data-group=\"last\"",
        "width:100%;max-width:none;",
        "</div><div",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(html.find("Author").unwrap() < html.find("id=\"surface\"").unwrap());
    assert!(
        html.contains(">Body</div><div") && html.contains(">0</div>"),
        "{html}"
    );
}

#[test]
fn empty_slots_leave_only_the_standalone_surface() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ChatMessageBubble { name: rsx! {}, metadata: rsx! { "" }, {rsx! {}} }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert_eq!(html.matches("<div").count(), 1, "{html}");
    assert!(
        html.contains("data-sender=\"assistant\"") && html.contains("data-density=\"balanced\""),
        "{html}"
    );
}
