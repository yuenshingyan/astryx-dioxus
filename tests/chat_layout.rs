use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{ChatDensity, ChatLayout, ChatLayoutScrollAction};

#[test]
fn empty_layout_preserves_dock_and_omits_hidden_scroll_action() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ChatLayout { id: "chat", density: ChatDensity::Compact, scroll_button: ChatLayoutScrollAction::Hidden,
                composer: rsx! { "Composer" }, empty_state: rsx! { "Empty transcript" },
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for value in [
        "id=\"chat\"",
        "data-density=\"compact\"",
        "Empty transcript",
        "Composer",
    ] {
        assert!(html.contains(value), "missing {value}: {html}");
    }
    assert!(!html.contains("Scroll to bottom"));
}

#[test]
fn custom_action_replaces_the_default() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ChatLayout { composer: rsx! { "Composer" }, scroll_button: ChatLayoutScrollAction::Custom(rsx! { button { "Jump" } }), "Message" }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(
        html.contains("Jump") && html.contains("Message") && !html.contains("Scroll to bottom"),
        "{html}"
    );
}
