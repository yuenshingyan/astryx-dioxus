use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{LayoutContent, SpacingStep};

#[test]
fn default_adds_one_scroll_owner_without_landmark_or_tab_stop() {
    let html = render_element(rsx! { LayoutContent { "Body" } });
    assert_eq!(html.matches("<div").count(), 1);
    assert!(html.contains("astryx-layout-content--scrollable"));
    assert!(html.contains("data-astryx-content-scroll="));
    for absent in ["role=", "aria-label=", "tabindex="] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn caller_landmark_and_full_bleed_do_not_install_overflow_naming() {
    let html = render_element(rsx! {
        LayoutContent { id: "body", role: "main", label: "Fallback", "aria-label": "Primary",
            padding: SpacingStep::Zero, class: "caller", "data-test": "kept", "Body"
        }
    });
    for expected in [
        "id=\"body\"",
        "role=\"main\"",
        "aria-label=\"Primary\"",
        "caller",
        "data-test=\"kept\"",
        "astryx-layout-content--full-bleed",
        "--_layout-content-padding:var(--spacing-0)",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert!(!html.contains("data-astryx-content-scroll") && !html.contains("Fallback"));
}

#[test]
fn clipped_region_preserves_the_caller_name() {
    let html =
        render_element(rsx! { LayoutContent { is_scrollable: false, label: "Named", "Body" } });
    assert!(html.contains("aria-label=\"Named\""));
    assert!(!html.contains("--scrollable") && !html.contains("data-astryx-content-scroll"));
}
