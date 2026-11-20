use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    Layout, LayoutContent, LayoutFooter, LayoutHeader, LayoutHeight, LayoutPanel, SizeValue,
    SpacingStep,
};

#[test]
fn slot_order_and_owned_root_metadata() {
    let html = render_element(rsx! { Layout {
        id: "shell", "data-height": "caller", height: LayoutHeight::Auto,
        header: rsx! { LayoutHeader { "First" } },
        start: rsx! { LayoutPanel { "Second" } },
        content: rsx! { LayoutContent { "Third" } },
        end: rsx! { LayoutPanel { "Fourth" } },
        footer: rsx! { LayoutFooter { "Fifth" } },
        "Ignored shorthand"
    } });
    assert!(html.contains("id=\"shell\"") && html.contains("data-height=\"auto\""));
    assert!(!html.contains("caller") && !html.contains("Ignored shorthand"));
    let positions =
        ["First", "Second", "Third", "Fourth", "Fifth"].map(|label| html.find(label).unwrap());
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(
        html.contains("astryx-layout-panel--start") && html.contains("astryx-layout-panel--end")
    );
}

#[test]
fn empty_slots_width_fallback_and_divider_inheritance() {
    let html = render_element(
        rsx! { Layout { default_has_dividers: true, padding: SpacingStep::Zero,
            start: rsx! { if false { "Absent panel" } },
            content_width: SizeValue::from("calc(100% - 1px)"),
            LayoutContent {
                Layout { header: rsx! { LayoutHeader { "Inherited" } }, footer: rsx! { LayoutFooter { has_divider: false, "Override" } } }
            }
        } },
    );
    assert!(html.contains("astryx-layout-content--open-start"));
    assert!(html.contains("astryx-layout__middle--constrained"));
    assert!(html.contains("--layout-padding-own-outer-x:var(--spacing-0)"));
    assert_eq!(html.matches("data-divider=\"true\"").count(), 1, "{html}");
}
