use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    STYLES, ScrollAxis, ScrollableArea, ScrollableAreaRole, SizeValue, SpacingStep,
};

#[test]
fn native_viewport_owns_semantics_and_content_publishes_padding() {
    let html = render_element(rsx! { ScrollableArea {
        id: "viewport", label: "Files", role: ScrollableAreaRole::Region, axis: ScrollAxis::Both,
        height: SizeValue::from(200), padding: SpacingStep::Two, padding_inline: SpacingStep::Four,
        padding_inline_end: SpacingStep::Zero,
        "aria-label": "Ignored", tabindex: "5", "Files go here"
    } });
    for expected in [
        "role=\"region\"",
        "aria-label=\"Files\"",
        "data-axis=\"both\"",
        "height:200px",
        "id=\"viewport\"",
        "data-scroll-content",
        "--container-padding-inline-start:var(--spacing-4)",
        "--container-padding-inline-end:var(--spacing-0)",
        "--container-padding-block-start:var(--spacing-2)",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert_eq!(html.matches("<div").count(), 2);
    assert!(!html.contains("Ignored") && !html.contains("tabindex"));
}

#[test]
fn bundled_viewport_styles_gate_scroll_state_on_support() {
    let styles = STYLES.split_whitespace().collect::<Vec<_>>().join(" ");
    let (_, viewport_styles) = styles
        .split_once(".astryx-scrollable-area {")
        .expect("bundled viewport styles");
    let (base, _) = viewport_styles.split_once('}').expect("viewport rule end");
    assert!(!base.contains("container-type"), "{base}");
    assert!(styles.contains(
        "@supports (container-type: scroll-state) { .astryx-scrollable-area { container-type: scroll-state; } }"
    ));
}
