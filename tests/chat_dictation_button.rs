use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{ChatDictationButton, ChatDictationState};

#[test]
fn unsupported_is_hidden_by_default() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ChatDictationButton { dictation: ChatDictationState::Unsupported, on_toggle: |_| {} }
        }
    });
    dom.rebuild_in_place();
    assert!(!render(&dom).contains("<button"));
}

#[test]
fn listening_has_a_named_button_and_five_decorative_bars() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ChatDictationButton { id: "dictation", dictation: ChatDictationState::Listening {
                bands: [f32::NAN, -1.0, 0.04, 0.2, 1.0], volume: 0.5,
            }, on_toggle: |_| {} }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(html.contains("id=\"dictation\"") && html.contains("aria-label=\"Stop dictation\""));
    assert!(html.contains("aria-hidden=\"true\"") && html.contains("--_dictation-clip:100%"));
    assert_eq!(html.matches("astryx-chat-dictation-button__bar").count(), 5);
    assert!(!html.contains("NaN") && !html.contains("<svg"));
}
