use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    LayerContentBoundary, Layout, LayoutContent, LayoutFooter, LayoutHeader, LayoutHeight,
    LayoutPanel, SizeValue, SpacingStep,
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

#[test]
fn arbitrary_slots_keep_native_semantics_without_an_inferred_landmark() {
    let html = render_element(rsx! { Layout {
        header: rsx! { h2 { "Title" } },
        start: rsx! { nav { "Navigation" } },
        end: rsx! { aside { "Details" } },
        footer: rsx! { small { "Fine print" } },
        article { "Body" }
    } });
    assert!(html.contains("data-height=\"fill\""), "{html}");
    assert!(
        !html.contains("role=") && !html.contains("aria-label="),
        "{html}"
    );
    for node in [
        "<h2>Title</h2>",
        "<nav>Navigation</nav>",
        "<article>Body</article>",
        "<aside>Details</aside>",
        "<small>Fine print</small>",
    ] {
        assert!(html.contains(node), "{html}");
    }

    let named = render_element(rsx! { Layout { role: "region", aria_label: "Workspace" } });
    let root = named.split('>').next().unwrap();
    assert!(
        root.contains("role=\"region\"") && root.contains("aria-label=\"Workspace\""),
        "{named}"
    );
}

#[test]
fn divider_default_does_not_reach_panels_or_cross_layer_boundary() {
    let html = render_element(rsx! { Layout { default_has_dividers: true,
        header: rsx! { LayoutHeader { "Outer header" } },
        start: rsx! { LayoutPanel { "Outer panel" } },
        LayerContentBoundary {
            Layout {
                header: rsx! { LayoutHeader { "Layer header" } },
                footer: rsx! { LayoutFooter { "Layer footer" } },
            }
        }
    } });
    assert_eq!(html.matches("data-divider=\"true\"").count(), 1, "{html}");
    assert!(!html.contains("astryx-layout-panel--divider"), "{html}");
}

#[test]
fn content_width_serialization_and_alignment_paths() {
    for (width, css, should_align) in [
        (SizeValue::from(0), "0px", true),
        (SizeValue::from(" 0 "), "0px", true),
        (SizeValue::from("60ch"), "60ch", true),
        (
            SizeValue::from("calc(var(--frame))"),
            "calc(var(--frame))",
            true,
        ),
        (SizeValue::from("var(--frame)"), "var(--frame)", false),
        (SizeValue::from("50%"), "50%", false),
        (
            SizeValue::from("clamp(200px, 50%, 400px)"),
            "clamp(200px, 50%, 400px)",
            false,
        ),
        (SizeValue::from("fit-content"), "fit-content", false),
    ] {
        let html =
            render_element(rsx! { Layout { content_width: width, LayoutContent { "Content" } } });
        assert!(
            html.contains(&format!("--layout-content-width:{css};")),
            "{html}"
        );
        assert_eq!(
            html.contains(&format!("--layout-alignment-width:{css};")),
            should_align,
            "{html}"
        );
        assert_eq!(
            html.contains("astryx-layout__middle--constrained"),
            !should_align,
            "{html}"
        );
    }

    let both = render_element(rsx! { Layout { content_width: SizeValue::from(360),
        start: rsx! { aside { "Start" } }, end: rsx! { aside { "End" } },
        LayoutContent { "Content" }
    } });
    assert!(
        both.contains("astryx-layout__middle--constrained"),
        "{both}"
    );
    assert!(!both.contains("--layout-alignment-width:"), "{both}");
}
