use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Layout, LayoutContent, SpacingStep};

#[path = "../examples/showcase/layout_content.rs"]
mod showcase;

use showcase::LayoutContentExamples;

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

#[test]
fn every_explicit_spacing_step_publishes_a_local_override() {
    for (padding, token) in [
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
        let html = render_element(rsx! {
            Layout { padding: SpacingStep::Four,
                LayoutContent { padding, span { "Direct child" } }
            }
        });
        assert!(
            html.contains("--layout-padding-own-outer-x:var(--spacing-4)"),
            "{html}"
        );
        assert!(
            html.contains(&format!("--_layout-content-padding:var(--spacing-{token})")),
            "{html}"
        );
        assert_eq!(
            html.contains("astryx-layout-content--full-bleed"),
            padding == SpacingStep::Zero
        );
        assert!(html.contains("><span>Direct child</span></div>"), "{html}");
    }
}

#[test]
fn showcase_patterns_render_named_native_roots_and_real_layout_slots() {
    let html = render_element(rsx! { LayoutContentExamples {} });
    for (id, label) in [("basic", "Main Content"), ("showcase", "Main Content Area")] {
        let id_attr = format!("id=\"layout-content-{id}\"");
        let root = html
            .split('<')
            .filter_map(|node| node.split_once('>'))
            .map(|(tag, _)| tag)
            .find(|tag| tag.contains(&id_attr))
            .unwrap();
        assert!(
            root.starts_with("div ") && root.contains("role=\"region\""),
            "{root}"
        );
        assert!(root.contains(&format!("aria-label=\"{label}\"")), "{root}");
        assert!(!root.contains("--open-header"), "{root}");
        assert_eq!(root.contains("--open-start"), id == "basic", "{root}");
        assert_eq!(root.contains("--open-footer"), id == "basic", "{root}");
    }
}
