use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Section, SectionDivider, SectionVariant, SizeValue, SpacingStep};

#[test]
fn outer_attributes_and_inner_theme_are_separate_without_added_semantics() {
    let html = render_element(rsx! { Section {
        id: "outer", class: "caller", style: "height:80px", height: SizeValue::from(160),
        variant: SectionVariant::new("custom-wash").unwrap(),
        dividers: vec![SectionDivider::Start, SectionDivider::Start],
        h2 { "Heading" }
        button { "Continue" }
    } });
    let (outer, inner) = html.split_once('>').unwrap();
    assert!(outer.contains("id=\"outer\"") && outer.contains("caller"));
    assert!(outer.contains("--_section-height:160px;height:80px"));
    assert!(!outer.contains("data-variant="));
    assert!(inner.contains("data-variant=\"custom-wash\""));
    assert_eq!(html.matches("astryx-section--divider-start").count(), 1);
    assert!(html.find("Heading").unwrap() < html.find("Continue").unwrap());
    for attr in ["role=", "tabindex=", "aria-"] {
        assert!(!html.contains(attr));
    }
    assert!(SectionVariant::new("invalid name").is_err());
}

#[test]
fn publishes_each_resolved_edge_and_only_propagates_uniform_padding() {
    let html = render_element(rsx! { Section {
        padding: SpacingStep::Four, padding_inline: SpacingStep::Two,
        padding_inline_end: SpacingStep::Zero, padding_block_start: SpacingStep::Three,
        Section { "Nested" }
    } });
    for value in [
        "--_section-padding-propagated:var(--spacing-4)",
        "--container-padding-inline-start:var(--spacing-2)",
        "--container-padding-inline-end:var(--spacing-0)",
        "--container-padding-block-start:var(--spacing-3)",
        "--container-padding-block-end:var(--spacing-4)",
    ] {
        assert_eq!(html.matches(value).count(), 1, "{html}");
    }
}
