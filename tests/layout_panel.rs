use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{LayoutPanel, SizeValue, SpacingStep};

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
