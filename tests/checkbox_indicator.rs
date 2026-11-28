use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{CheckboxIndicator, CheckboxIndicatorState, IndicatorSize};

#[path = "../examples/showcase/checkbox_indicator.rs"]
mod showcase;

use showcase::CheckboxIndicatorExamples;

#[test]
fn box_and_mark_targets_keep_owned_state_and_native_attributes() {
    let html = render_element(rsx! {
        CheckboxIndicator { state: CheckboxIndicatorState::Indeterminate, size: IndicatorSize::Small,
            is_disabled: true, class: "host", style: "margin:2px", id: "mark", dir: "rtl",
            "aria-hidden": "false", "data-checked": "checked", "data-size": "md", "data-disabled": "false"
        }
    });
    for expected in [
        "astryx-checkbox-indicator host",
        "data-checked=\"indeterminate\"",
        "data-disabled=\"disabled\"",
        "id=\"mark\"",
        "dir=\"rtl\"",
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
        "data-size=\"md\"",
        "data-disabled=\"false\"",
        "<input",
        "role=",
        "tabindex=",
    ] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn every_state_has_one_hidden_box_and_defaults_to_medium_and_enabled() {
    for (state, checked) in [
        (CheckboxIndicatorState::Unchecked, None),
        (CheckboxIndicatorState::Checked, Some("checked")),
        (CheckboxIndicatorState::Indeterminate, Some("indeterminate")),
    ] {
        let html = render_element(rsx! { CheckboxIndicator { state } });
        assert!(html.starts_with("<span "), "{html}");
        assert_eq!(html.matches("aria-hidden=\"true\"").count(), 1, "{html}");
        assert_eq!(html.matches("data-size=\"md\"").count(), 3, "{html}");
        assert!(!html.contains("data-disabled="), "{html}");
        if let Some(checked) = checked {
            assert!(
                html.contains(&format!("data-checked=\"{checked}\"")),
                "{html}"
            );
        } else {
            assert!(!html.contains("data-checked="), "{html}");
        }
        assert!(html.contains("astryx-checkbox-indicator-check"), "{html}");
        assert!(html.contains("astryx-checkbox-indicator-dash"), "{html}");
    }
}

#[test]
fn replacement_keeps_the_box_and_removes_both_marks() {
    let html = render_element(rsx! {
        CheckboxIndicator { state: CheckboxIndicatorState::Checked, replacement: rsx! {} }
        CheckboxIndicator { state: CheckboxIndicatorState::Unchecked, replacement: rsx! { "Pending" } }
        CheckboxIndicator { state: CheckboxIndicatorState::Indeterminate, replacement: rsx! { span { id: "empty-container" } } }
    });
    assert_eq!(html.matches("aria-hidden=\"true\"").count(), 3);
    assert!(html.contains("Pending"));
    assert!(html.contains("id=\"empty-container\""));
    assert!(!html.contains("indicator-check") && !html.contains("indicator-dash"));
}

#[test]
fn showcase_keeps_the_checkbox_label_on_the_native_owner_and_renders_the_source_examples() {
    let html = render_element(rsx! { CheckboxIndicatorExamples {} });
    assert_eq!(html.matches("type=\"checkbox\"").count(), 1);
    assert!(html.contains("for=\"ci-native-owner\""), "{html}");
    assert!(html.contains("Show completed tasks"));
    assert!(html.contains("id=\"ci-mixed\""));
    assert!(html.contains("id=\"ci-brand-disabled\""));
    assert_eq!(html.matches('★').count(), 1);
    assert_eq!(html.matches("data-shade=\"inherit\"").count(), 2);
}
