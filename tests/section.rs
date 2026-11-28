use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Section, SectionDivider, SectionVariant, SizeValue, SpacingStep};
use showcase::SectionExamples;

#[path = "../examples/showcase/section.rs"]
mod showcase;

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
    assert!(outer.starts_with("<div "));
    assert!(inner.starts_with("<div "));
    assert!(outer.contains("id=\"outer\"") && outer.contains("caller"));
    assert!(!inner.contains("id=\"outer\"") && !inner.contains("caller"));
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
fn preserves_caller_semantics_on_the_wrapper_and_reflects_each_divider() {
    for (side, suffix) in [
        (SectionDivider::Top, "top"),
        (SectionDivider::Bottom, "bottom"),
        (SectionDivider::Start, "start"),
        (SectionDivider::End, "end"),
    ] {
        let html = render_element(rsx! { Section {
            role: "region", aria_label: "Account settings", "data-context": "settings",
            dividers: vec![side],
            "Content"
        } });
        let (outer, inner) = html.split_once('>').unwrap();
        for attr in [
            "role=\"region\"",
            "aria-label=\"Account settings\"",
            "data-context=\"settings\"",
        ] {
            assert!(outer.contains(attr) && !inner.contains(attr), "{html}");
        }
        assert!(inner.contains("data-variant=\"section\""));
        assert!(inner.contains(&format!("astryx-section--divider-{suffix}")));
        assert_eq!(html.matches("astryx-section--divider-").count(), 1);
    }
}

#[test]
fn sizing_keeps_css_wide_keywords_on_actual_dimensions() {
    let html = render_element(rsx! { Section {
        width: SizeValue::from("calc(100% - 2rem)"), height: SizeValue::from("inherit"),
        max_width: SizeValue::from(1280), min_height: SizeValue::from("/* caller */ revert-layer"),
        style: "height:10rem;",
        "Content"
    } });
    let (outer, inner) = html.split_once('>').unwrap();
    for declaration in [
        "--_section-width:calc(100% - 2rem);",
        "--_section-max-width:1280px;",
        "min-height:/* caller */ revert-layer;height:10rem;",
    ] {
        assert!(
            outer.contains(declaration) && !inner.contains(declaration),
            "{html}"
        );
    }
    assert!(outer.contains(";height:inherit;"));
    assert!(!outer.contains("--_section-height"));
}

#[test]
fn publishes_each_resolved_edge_and_only_propagates_uniform_padding() {
    let html = render_element(rsx! { Section {
        padding: SpacingStep::Four, padding_inline: SpacingStep::Two,
        padding_inline_end: SpacingStep::Zero, padding_block: SpacingStep::One,
        padding_block_start: SpacingStep::Three,
        Section { padding_inline_start: SpacingStep::Half, padding_block_end: SpacingStep::OneAndHalf,
            "Nested"
        }
    } });
    for value in [
        "--_section-padding-propagated:var(--spacing-4)",
        "--container-padding-inline-start:var(--spacing-2)",
        "--container-padding-inline-end:var(--spacing-0)",
        "--container-padding-block-start:var(--spacing-3)",
        "--container-padding-block-end:var(--spacing-1)",
        "--container-padding-inline-start:var(--spacing-0-5)",
        "--container-padding-block-end:var(--spacing-1-5)",
    ] {
        assert_eq!(html.matches(&format!("{value};")).count(), 1, "{html}");
    }
}

#[test]
fn supports_the_full_spacing_scale_including_zero() {
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
        let html = render_element(rsx! { Section { padding: step } });
        for edge in ["inline-start", "inline-end", "block-start", "block-end"] {
            assert!(html.contains(&format!(
                "--container-padding-{edge}:var(--spacing-{token});"
            )));
        }
    }
}

#[test]
fn named_examples_render_sections_and_caller_content() {
    let html = render_element(rsx! { SectionExamples {} });
    let (_, centered) = html.split_once("id=\"section-centered\"").unwrap();
    let (centered, patterns) = centered.split_once("id=\"section-variants\"").unwrap();
    assert!(centered.contains("--_section-max-width:1280px;"));
    assert!(centered.contains("margin-inline-start:auto; margin-inline-end:auto;"));
    let (variants, settings) = patterns.split_once("id=\"section-with-dividers\"").unwrap();
    let (settings, wash) = settings
        .split_once("id=\"section-wash-highlight\"")
        .unwrap();
    for variant in ["section", "muted", "transparent"] {
        assert!(variants.contains(&format!("data-variant=\"{variant}\"")));
    }
    assert_eq!(
        settings.matches("astryx-section--divider-bottom").count(),
        2
    );
    for title in ["Account", "Notifications", "Privacy"] {
        assert!(settings.contains(title));
    }
    for text in ["Pro Plan", "Advanced analytics", "$49", "Upgrade"] {
        assert!(wash.contains(text));
    }
    assert!(wash.contains("data-variant=\"muted\""));
}
