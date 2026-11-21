use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{ChatSendButton, ChatSendButtonSize};

#[test]
fn missing_readiness_defaults_to_disabled_send() {
    let mut dom = VirtualDom::new(|| rsx! { ChatSendButton {} });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "<button",
        "type=\"button\"",
        "aria-disabled=\"true\"",
        "aria-label=\"Send\"",
        "data-variant=\"primary\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}

#[test]
fn stop_ignores_send_readiness_and_uses_selected_icon() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ChatSendButton { is_disabled: true, is_stop_shown: true, size: ChatSendButtonSize::Small,
                stop_label: "Halt", id: "stop", send_icon: rsx! { "Wrong icon" }, stop_icon: rsx! { "Correct icon" },
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "id=\"stop\"",
        "aria-label=\"Halt\"",
        "data-variant=\"secondary\"",
        "data-size=\"sm\"",
        "Correct icon",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(
        !html.contains("aria-disabled") && !html.contains("Wrong icon"),
        "{html}"
    );
}
