use dioxus::prelude::*;

use astryx_dioxus::{Center, CenterAxis, SizeValue, SpacingStep};

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
    assert!(html.contains("data-axis=\"both\""));
    assert!(html.contains("data-extra=\"kept\""));
    assert!(html.find("First") < html.find("Second"));
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
