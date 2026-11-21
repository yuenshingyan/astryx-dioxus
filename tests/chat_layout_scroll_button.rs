use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::ChatLayoutScrollButton;

#[test]
fn hidden_control_is_inert_and_keeps_its_accessible_label_markup() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ChatLayoutScrollButton { id: "position", is_visible: false, scroll_label: "Jump to latest", onclick: |_| {} }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "id=\"position\"",
        " inert",
        "aria-label=\"Jump to latest\"",
        "type=\"button\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("data-visible"), "{html}");
}

#[test]
fn visible_label_expands_the_action_without_inert() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ChatLayoutScrollButton { is_visible: true, label: "New messages", onclick: |_| {} }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(
        html.contains("data-expanded=\"true\"") && html.contains("New messages"),
        "{html}"
    );
    assert!(!html.contains(" inert"), "{html}");
}
