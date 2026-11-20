use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{IndicatorSize, RadioIndicator, RadioIndicatorState};

#[test]
fn unchecked_retains_decorative_circle_without_a_dot() {
    let html = render_element(rsx! {
        RadioIndicator { state: RadioIndicatorState::Unchecked, id: "radio", "aria-hidden": "false", "data-checked": "spoof" }
    });
    for expected in ["id=\"radio\"", "data-size=\"md\"", "aria-hidden=\"true\""] {
        assert!(html.contains(expected), "{html}");
    }
    for absent in [
        "data-checked",
        "data-disabled",
        "indicator-dot",
        "role=",
        "tabindex=",
        "<input",
    ] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn checked_reflects_state_and_size_and_composes_caller_styles() {
    let html = render_element(rsx! {
        RadioIndicator { state: RadioIndicatorState::Checked, size: IndicatorSize::Small,
            is_disabled: true, class: "host-mark", style: "margin:2px", "data-note": "kept" }
    });
    for expected in [
        "astryx-radio-indicator host-mark",
        "data-checked=\"checked\"",
        "data-disabled=\"disabled\"",
        "data-note=\"kept\"",
        "margin:2px",
        "astryx-radio-indicator-dot",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert_eq!(html.matches("data-size=\"sm\"").count(), 2);
}

#[test]
fn replacement_keeps_circle_and_supersedes_dot_in_either_state() {
    let html = render_element(rsx! {
        RadioIndicator { state: RadioIndicatorState::Checked, replacement: rsx! {} }
        RadioIndicator { state: RadioIndicatorState::Unchecked, replacement: rsx! { "Pending" } }
    });
    assert_eq!(html.matches("aria-hidden=\"true\"").count(), 2);
    assert!(html.contains("Pending"));
    assert!(!html.contains("indicator-dot"));
}
