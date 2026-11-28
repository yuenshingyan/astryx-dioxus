use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{
    ChatSendButton, ChatSendButtonSize, InternationalizationProvider, MessagesByLocale,
};

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

#[test]
fn accessible_names_follow_the_selected_state_and_preserve_explicit_labels() {
    let mut dom = VirtualDom::new(|| {
        let messages = MessagesByLocale::from([(
            "fr".into(),
            [
                ("@astryx.chatSendButton.send".into(), "Envoyer".into()),
                ("@astryx.chatSendButton.stop".into(), "Arrêter".into()),
            ]
            .into(),
        )]);
        rsx! {
            InternationalizationProvider { locale: "fr", messages,
                ChatSendButton { id: "send", is_disabled: false }
                ChatSendButton { id: "stop", is_stop_shown: true }
                ChatSendButton { id: "custom", send_label: "Send" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for (id, name) in [("send", "Envoyer"), ("stop", "Arrêter"), ("custom", "Send")] {
        let button = html
            .split("<button ")
            .filter_map(|markup| markup.split_once('>').map(|(attrs, _)| attrs))
            .find(|attrs| attrs.contains(&format!("id=\"{id}\"")))
            .unwrap_or_else(|| panic!("missing button {id}: {html}"));
        assert!(
            button.contains(&format!("aria-label=\"{name}\"")),
            "{button}"
        );
    }
}

#[test]
fn blank_names_fall_back_to_english() {
    let mut dom = VirtualDom::new(|| {
        let messages = MessagesByLocale::from([(
            "fr".into(),
            [("@astryx.chatSendButton.stop".into(), " ".into())].into(),
        )]);
        rsx! {
            InternationalizationProvider { locale: "fr", messages,
                ChatSendButton { is_stop_shown: true, stop_label: " " }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(html.contains("aria-label=\"Stop\""), "{html}");
}
