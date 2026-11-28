use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Layout, LayoutPanel, SizeValue, SpacingStep};

#[test]
fn default_panel_scrolls_without_inventing_a_landmark() {
    let html = render_element(rsx! { LayoutPanel { "Body" } });
    assert_eq!(html.matches("<div").count(), 1);
    assert!(html.contains("astryx-layout-panel--scrollable"));
    assert!(html.contains("data-astryx-panel-scroll"));
    for absent in ["role=", "aria-label=", "tabindex="] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn numeric_zero_disables_scroll_measurement_without_overriding_caller_semantics() {
    let html = render_element(rsx! { LayoutPanel {
        width: SizeValue::from(0), padding: SpacingStep::Two, role: "navigation",
        label: "Local navigation", id: "panel", "Body"
    } });
    for expected in [
        "astryx-layout-panel--zero-width",
        "width:0px",
        "--_layout-panel-padding:var(--spacing-2)",
        "role=\"navigation\"",
        "aria-label=\"Local navigation\"",
        "id=\"panel\"",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert!(!html.contains("--scrollable") && !html.contains("data-astryx-panel-scroll"));
}

#[test]
fn numeric_zero_removes_the_side_slot_divider_but_css_width_does_not() {
    for (width, has_divider) in [(SizeValue::from(0), false), (SizeValue::from("0px"), true)] {
        let html = render_element(rsx! { Layout {
            start: rsx! { LayoutPanel { width: width.clone(), has_divider: true, "Start" } },
            end: rsx! { LayoutPanel { width, has_divider: true, "End" } },
        } });
        assert!(html.contains("astryx-layout-panel--start"));
        assert!(html.contains("astryx-layout-panel--end"));
        assert_eq!(
            html.matches("astryx-layout-panel--divider").count(),
            if has_divider { 2 } else { 0 },
            "{html}"
        );
        assert!(
            !html.contains("astryx-layout-panel--collapse-gap"),
            "{html}"
        );
    }
}

#[test]
fn explicit_inset_replaces_the_automatic_gap_for_every_step() {
    for (step, token) in [
        (SpacingStep::Zero, "0"),
        (SpacingStep::Half, "0-5"),
        (SpacingStep::One, "1"),
        (SpacingStep::OneAndHalf, "1-5"),
        (SpacingStep::Two, "2"),
        (SpacingStep::Three, "3"),
        (SpacingStep::Four, "4"),
        (SpacingStep::Five, "5"),
        (SpacingStep::Six, "6"),
        (SpacingStep::Eight, "8"),
        (SpacingStep::Ten, "10"),
    ] {
        let html = render_element(rsx! { Layout { default_has_dividers: true,
            end: rsx! { LayoutPanel { padding: step, "Panel content" } },
        } });
        assert!(
            html.contains(&format!("--_layout-panel-padding:var(--spacing-{token})")),
            "{html}"
        );
        for absent in ["--collapse-gap", "astryx-layout-panel--divider"] {
            assert!(!html.contains(absent), "{html}");
        }
    }
}

#[test]
fn caller_content_and_native_attributes_stay_on_the_panel_boundary() {
    let html = render_element(rsx! { LayoutPanel {
        id: "inspector", class: "custom", role: "complementary", label: "Default name",
        "aria-label": "Inspector", "data-owner": "caller", is_scrollable: false,
        width: SizeValue::from("calc(100% - 24px)"), style: "width:9rem;",
        h2 { "Settings" }
        button { "Reset" }
    } });
    for expected in [
        "id=\"inspector\"",
        " custom\"",
        "role=\"complementary\"",
        "aria-label=\"Inspector\"",
        "data-owner=\"caller\"",
        "width:calc(100% - 24px);width:9rem;",
        "><h2>Settings</h2><button>Reset</button></div>",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    for absent in ["--scrollable", "data-astryx-panel-scroll", "Default name"] {
        assert!(!html.contains(absent), "{html}");
    }
}
