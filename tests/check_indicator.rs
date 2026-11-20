use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{CheckIndicator, CheckIndicatorState};

#[test]
fn unchecked_has_no_box_and_checked_uses_the_icon_root() {
    let html = render_element(rsx! {
        CheckIndicator { state: CheckIndicatorState::Unchecked, id: "absent" }
        CheckIndicator { state: CheckIndicatorState::Checked, id: "chosen", class: "host-mark", "aria-hidden": "false" }
    });
    assert!(!html.contains("absent"));
    assert!(html.starts_with("<span"), "{html}");
    let root = html.split_once('>').unwrap().0;
    for expected in [
        "astryx-icon host-mark",
        "id=\"chosen\"",
        "aria-hidden=\"true\"",
        "color:var(--color-accent)",
    ] {
        assert!(root.contains(expected), "{html}");
    }
    assert!(html.contains("<svg"));
    assert!(!html.contains("aria-hidden=\"false\""));
}

#[test]
fn replacement_in_either_state_preserves_the_box_and_native_attributes() {
    let html = render_element(rsx! {
        CheckIndicator { state: CheckIndicatorState::Unchecked, is_disabled: true, id: "busy", style: "opacity:.5", replacement: rsx! { "0" } }
        CheckIndicator { state: CheckIndicatorState::Checked, id: "empty", replacement: rsx! {} }
    });
    for expected in [
        "id=\"busy\"",
        "id=\"empty\"",
        "width:1rem;height:1rem",
        "color:var(--color-icon-disabled);opacity:.5",
        ">0</span>",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert_eq!(html.matches("aria-hidden=\"true\"").count(), 2);
    assert!(!html.contains("<svg"));
}
