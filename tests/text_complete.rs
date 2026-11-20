use astryx_dioxus::{LayerPlacement, Text, TextTag, Truncation, use_truncation};
use dioxus::prelude::*;

fn render_element<F: FnOnce() -> Element + Clone + 'static>(render: F) -> String {
    let mut dom = VirtualDom::new_with_props(|render: F| render(), render);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn placements_are_additive_and_keep_existing_boolean_callers() {
    for (placement, spelling) in [
        (LayerPlacement::Above, "above"),
        (LayerPlacement::Below, "below"),
        (LayerPlacement::Start, "start"),
        (LayerPlacement::End, "end"),
    ] {
        assert_eq!(placement.as_str(), spelling);
        let html = render_element(move || {
            rsx! {
                Text {
                    max_lines: 2,
                    has_truncate_tooltip: false,
                    truncate_tooltip_placement: placement,
                    "Contents"
                }
            }
        });
        assert!(html.contains(&format!("data-astryx-truncation-placement=\"{spelling}\"")));
        assert!(!html.contains("role=\"tooltip\""));
        assert!(!html.contains("tabindex"));
        assert!(!html.contains("aria-describedby"));
    }
}

#[test]
fn clamp_wins_over_inline_style_and_native_passthrough_remains_available() {
    let html = render_element(move || {
        rsx! {
            Text {
                tag: TextTag::P, max_lines: 3,
                style: "color:rebeccapurple;-webkit-line-clamp:1",
                lang: "fr", hidden: true,
                "aria-describedby": "caller-description",
                "data-native": "kept",
                onwheel: move |_| {}, onscroll: move |_| {},
                onanimationstart: move |_| {}, ontransitionend: move |_| {},
                ondragstart: move |_| {},
                "Rich " strong { "content" }
            }
        }
    });
    assert!(html.starts_with("<p"));
    for expected in [
        "color:rebeccapurple;-webkit-line-clamp:1;-webkit-line-clamp:3",
        "lang=\"fr\"",
        "hidden=true",
        "aria-describedby=\"caller-description\"",
        "data-native=\"kept\"",
        "<strong>content</strong>",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
}

#[test]
fn standalone_hook_is_ssr_safe_and_has_no_eager_browser_answer() {
    let html = render_element(move || {
        let Truncation {
            onmounted,
            is_truncated,
            full_text,
        } = use_truncation("advanced-text".to_string(), 2, false);
        rsx! {
            span { "data-astryx-truncation-id": "advanced-text", onmounted, "Full content" }
            output { "{is_truncated()}|{full_text()}" }
        }
    });
    assert!(html.contains("Full content"));
    assert!(html.contains("false|"));
    assert!(!html.contains("role=\"tooltip\""));
    assert!(!html.contains("title="));
}
