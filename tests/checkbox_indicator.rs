use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{CheckboxIndicator, CheckboxIndicatorState, IndicatorSize};

#[test]
fn box_and_mark_targets_keep_owned_state_and_native_attributes() {
    let html = render_element(rsx! {
        CheckboxIndicator { state: CheckboxIndicatorState::Indeterminate, size: IndicatorSize::Small,
            is_disabled: true, class: "host", style: "margin:2px", id: "mark",
            "aria-hidden": "false", "data-checked": "checked"
        }
    });
    for expected in [
        "astryx-checkbox-indicator host",
        "data-checked=\"indeterminate\"",
        "data-disabled=\"disabled\"",
        "id=\"mark\"",
        "margin:2px",
        "aria-hidden=\"true\"",
        "astryx-checkbox-indicator-check",
        "astryx-checkbox-indicator-dash",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert_eq!(html.matches("data-size=\"sm\"").count(), 3);
    for absent in [
        "aria-hidden=\"false\"",
        "data-checked=\"checked\"",
        "<input",
        "role=",
        "tabindex=",
    ] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn replacement_keeps_the_box_and_removes_both_marks() {
    let html = render_element(rsx! {
        CheckboxIndicator { state: CheckboxIndicatorState::Checked, replacement: rsx! {} }
        CheckboxIndicator { state: CheckboxIndicatorState::Unchecked, replacement: rsx! { "Pending" } }
    });
    assert_eq!(html.matches("aria-hidden=\"true\"").count(), 2);
    assert!(html.contains("Pending"));
    assert!(!html.contains("indicator-check") && !html.contains("indicator-dash"));
}
