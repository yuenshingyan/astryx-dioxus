use dioxus::prelude::*;

use astryx_dioxus::{Center, CenterAxis, SizeValue, SpacingStep};

#[path = "../examples/showcase/center.rs"]
mod showcase;

use showcase::CenterExamples;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn native_root_preserves_semantics_and_order() {
    let html = render(|| {
        rsx! { Center { id: "root", "data-axis": "spoof", "data-extra": "kept",
            button { "First" } a { href: "#root", "Second" }
        } }
    });
    assert_eq!(html.matches("<div").count(), 1);
    assert!(html.starts_with("<div"), "{html}");
    assert_eq!(html.matches("data-axis=").count(), 1);
    assert!(html.contains("data-axis=\"both\""));
    assert!(html.contains("id=\"root\""));
    assert!(html.contains("data-extra=\"kept\""));
    let first = html.find("<button").expect("native button");
    let second = html.find("<a href=\"#root\"").expect("native link");
    assert!(first < second, "{html}");
    for absent in [
        "role=",
        "tabindex=",
        "aria-",
        "spoof",
        "--container-padding",
    ] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn local_padding_prefers_edge_then_axis_then_all_and_keeps_zero() {
    let html = render(|| {
        rsx! { Center {
            axis: CenterAxis::Vertical, is_inline: true, class: "caller", style: "width:50%",
            width: SizeValue::from(200), height: SizeValue::from("12rem"),
            max_width: SizeValue::from("100%"), min_height: SizeValue::from(40),
            padding: SpacingStep::Four, padding_inline: SpacingStep::Two,
            padding_inline_start: SpacingStep::Zero, padding_block_end: SpacingStep::One,
            "Content"
        } }
    });
    for expected in [
        "astryx-center--inline caller",
        "data-axis=\"vertical\"",
        "--_center-width:200px",
        "--_center-height:12rem",
        "--_center-max-width:100%",
        "--_center-min-height:40px",
        "--_center-padding-inline-start:var(--spacing-0)",
        "--_center-padding-inline-end:var(--spacing-2)",
        "--_center-padding-block-start:var(--spacing-4)",
        "--_center-padding-block-end:var(--spacing-1)",
        "width:50%",
    ] {
        assert!(html.contains(expected), "{html}");
    }
}

#[test]
fn padding_axis_and_edge_overrides_are_independent() {
    let html = render(|| {
        rsx! {
            Center { padding: SpacingStep::Ten, padding_inline: SpacingStep::Half,
                padding_block: SpacingStep::OneAndHalf,
                padding_inline_end: SpacingStep::Eight, padding_block_start: SpacingStep::Zero,
                "Content"
            }
        }
    });
    for (edge, step) in [
        ("inline-start", "0-5"),
        ("inline-end", "8"),
        ("block-start", "0"),
        ("block-end", "1-5"),
    ] {
        assert!(
            html.contains(&format!("--_center-padding-{edge}:var(--spacing-{step});")),
            "{html}"
        );
    }
    assert!(!html.contains("--container-padding"), "{html}");
}

#[test]
fn css_wide_keywords_apply_to_dimensions_not_custom_properties() {
    let html = render(|| {
        rsx! {
            Center { axis: CenterAxis::Horizontal, class: "caller",
                width: SizeValue::from(" /* comment */ INHERIT "),
                height: SizeValue::from("initial"),
                max_width: SizeValue::from("revert-layer"),
                min_height: SizeValue::from("unset"),
                style: "width:50%", "Content"
            }
            Center { width: SizeValue::from("revert"), "Content" }
        }
    });
    for modifier in [
        "width-inherit",
        "height-initial",
        "max-width-revert-layer",
        "min-height-unset",
        "width-revert",
    ] {
        assert!(
            html.contains(&format!("astryx-center--{modifier}")),
            "{html}"
        );
    }
    assert!(!html.contains("--_center-"), "{html}");
    assert!(html.contains("data-axis=\"horizontal\""), "{html}");
    assert!(html.contains("style=\"width:50%\""), "{html}");
}

#[test]
fn pinned_patterns_are_present_in_the_showcase() {
    let html = render(CenterExamples);
    for expected in [
        "id=\"center-formatting\"",
        "id=\"center-empty-state\"",
        "id=\"center-showcase\"",
        "No messages yet",
        "Messages from your team will appear here.",
        "Centered content",
        "Horizontally and vertically aligned.",
    ] {
        assert!(html.contains(expected), "missing {expected}");
    }
    for label in ["Bold", "Italic", "Underline", "List", "Link", "Image"] {
        assert!(
            html.contains(&format!("aria-label=\"{label}\"")),
            "missing {label}"
        );
    }
}

#[test]
fn caller_can_opt_into_root_semantics() {
    let html = render(|| {
        rsx! {
            Center { role: "group", aria_label: "Tools", tabindex: "-1", "Content" }
        }
    });
    let root = html.split_once('>').expect("root start tag").0;
    for expected in ["role=\"group\"", "aria-label=\"Tools\"", "tabindex=\"-1\""] {
        assert!(root.contains(expected), "{html}");
    }
}
