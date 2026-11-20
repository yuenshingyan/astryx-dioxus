use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{LayoutFooter, SizeValue, SpacingStep};

#[test]
fn default_footer_adds_no_landmark_or_divider() {
    let html = render_element(rsx! { LayoutFooter { "Actions" } });
    assert_eq!(html.matches("<div").count(), 2);
    assert!(html.contains("astryx-layout-footer__inner") && html.contains("Actions"));
    for absent in ["role=", "aria-label=", "tabindex=", "data-divider="] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn outer_attributes_and_full_bleed_have_separate_owners() {
    let html = render_element(rsx! {
        LayoutFooter { id: "footer", role: "contentinfo", label: "Site information",
            has_divider: false, "data-divider": "true", height: SizeValue::from(72),
            style: "height:80px", padding: SpacingStep::Zero, "Actions"
        }
    });
    let (outer, inner) = html.split_once('>').unwrap();
    for expected in [
        "id=\"footer\"",
        "role=\"contentinfo\"",
        "aria-label=\"Site information\"",
        "--_layout-footer-height:72px;height:80px",
    ] {
        assert!(outer.contains(expected), "{html}");
    }
    assert!(!outer.contains("data-divider"));
    assert!(inner.contains("--_layout-footer-padding:var(--spacing-0)"));
}
