use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{LayoutHeader, SizeValue, SpacingStep};

#[test]
fn default_band_has_no_landmark_or_divider() {
    let html = render_element(rsx! { LayoutHeader { "Dashboard" } });
    assert_eq!(html.matches("<div").count(), 2);
    assert!(html.contains("astryx-layout-header__inner") && html.contains("Dashboard"));
    for absent in ["role=", "aria-label=", "tabindex=", "data-divider="] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn caller_attributes_stay_on_the_band_and_zero_overrides_the_block_end() {
    let html = render_element(rsx! {
        LayoutHeader { id: "header", label: "Site tools", role: "banner", "data-divider": "spoof",
            has_divider: true, height: SizeValue::from(72), style: "height:80px",
            padding: SpacingStep::Two, padding_block_end: SpacingStep::Zero, "Tools"
        }
    });
    let (outer, inner) = html.split_once('>').unwrap();
    for expected in [
        "id=\"header\"",
        "aria-label=\"Site tools\"",
        "role=\"banner\"",
        "data-divider=\"true\"",
        "--_layout-header-height:72px;height:80px",
    ] {
        assert!(outer.contains(expected), "{html}");
    }
    assert!(inner.contains(
        "--_layout-header-padding:var(--spacing-2);--_layout-header-padding-end:var(--spacing-0)"
    ));
    assert!(!html.contains("spoof"));
}
