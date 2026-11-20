use astryx_dioxus::{
    Heading, HeadingLevel, HeadingType, LayerPlacement, TextColor, TextDisplay, TextJustify,
    TextWeight, TextWrap, WordBreak,
};
use dioxus::prelude::*;

fn render<F: FnOnce() -> Element + Clone + 'static>(render: F) -> String {
    let mut dom = VirtualDom::new_with_props(|render: F| render(), render);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn native_levels_have_required_baseline_and_no_synthetic_semantics() {
    for number in 1..=6 {
        let level = HeadingLevel::try_from(number).unwrap();
        assert_eq!(level.number(), number);
        let html = render(move || rsx! { Heading { level, "Title" } });
        assert!(html.starts_with(&format!("<h{number} ")), "{html}");
        assert!(html.ends_with(&format!("Title</h{number}>")), "{html}");
        assert!(html.contains(&format!("astryx-heading--level-{number}")));
        assert!(html.contains(&format!("data-level={number}")), "{html}");
        assert!(html.contains("data-color=\"primary\""));
        assert!(html.contains("astryx-text--block"));
        for forbidden in [
            "aria-level",
            "role=",
            "tabindex",
            "title=",
            "data-type=",
            "data-weight=",
            "aria-describedby",
        ] {
            assert!(!html.contains(forbidden), "{forbidden}: {html}");
        }
    }
    for number in [0, 7, 255] {
        assert!(HeadingLevel::try_from(number).is_err());
    }
}

#[test]
fn visual_types_and_weights_do_not_change_outline() {
    for heading_type in [
        HeadingType::DISPLAY_1,
        HeadingType::DISPLAY_2,
        HeadingType::DISPLAY_3,
    ] {
        let name = heading_type.as_str().to_string();
        let html = render(move || {
            rsx! {
                Heading { level: HeadingLevel::Two, heading_type: heading_type.clone(), weight: TextWeight::Bold, "Hero" }
            }
        });
        assert!(html.starts_with("<h2 "));
        assert!(html.contains(&format!("data-type=\"{name}\"")));
        assert!(html.contains(&format!("astryx-text--type-{name}")));
        assert!(html.contains("data-weight=\"bold\""));
        assert!(html.contains("astryx-text--weight-bold"));
        assert!(!html.contains("aria-level"));
    }
    let html = render(|| {
        rsx! {
            Heading { level: HeadingLevel::Three, heading_type: HeadingType::new("hero").unwrap(), color: TextColor::new("brand").unwrap(), "Custom" }
        }
    });
    for expected in [
        "data-type=\"hero\"",
        "data-color=\"brand\"",
        "astryx-heading--level-3",
        "astryx-text--color-primary",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert!(!html.contains("astryx-text--type-hero"));
    assert!(HeadingType::new("").is_err());
    assert!(HeadingType::new("bad name").is_err());
}

#[test]
fn accessibility_override_and_raw_caller_precedence() {
    let html = render(|| {
        rsx! {
            Heading { level: HeadingLevel::Two, accessibility_level: HeadingLevel::Three, "Outline" }
        }
    });
    assert!(html.starts_with("<h2 "));
    assert!(html.contains("aria-level=3"), "{html}");
    let equal = render(|| {
        rsx! {
            Heading { level: HeadingLevel::Two, accessibility_level: HeadingLevel::Two, "Equal" }
        }
    });
    assert!(!equal.contains("aria-level"));
    let caller = render(|| {
        rsx! {
            Heading { level: HeadingLevel::Two, accessibility_level: HeadingLevel::Three, "aria-level": "4", "Caller" }
        }
    });
    assert!(caller.contains("aria-level=\"4\""), "{caller}");
    assert!(!caller.contains("aria-level=3"), "{caller}");
}

#[test]
fn clamp_and_shared_layout_precedence() {
    let html = render(|| {
        rsx! {
            Heading {
                level: HeadingLevel::Two, max_lines: 3, display: TextDisplay::Inline,
                style: "opacity:0.5;-webkit-line-clamp:5", class: "consumer",
                text_wrap: TextWrap::Balance, justify: TextJustify::End,
                has_capsize: true, has_strikethrough: true,
                "Rich " strong { "text" }
            }
        }
    });
    for expected in [
        "astryx-text--multi-line",
        "astryx-text--break-word",
        "astryx-text--capsize",
        "astryx-text--wrap-balance",
        "astryx-text--justify-end",
        "astryx-text--strikethrough",
        "consumer",
        "opacity:0.5;-webkit-line-clamp:5;-webkit-line-clamp:3",
        "<strong>text</strong>",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    let single =
        render(|| rsx! { Heading { level: HeadingLevel::One, max_lines: 1, "Full text" } });
    assert!(single.contains("astryx-text--single-line"));
    assert!(single.contains("astryx-text--break-all"));
    assert!(!single.contains("title="));
    let custom = render(
        || rsx! { Heading { level: HeadingLevel::One, max_lines: 1, word_break: WordBreak::BreakWord, "Full text" } },
    );
    assert!(custom.contains("astryx-text--break-word"));
    let inline = render(
        || rsx! { Heading { level: HeadingLevel::Four, display: TextDisplay::Inline, word_break: WordBreak::BreakAll, "Inline" } },
    );
    assert!(inline.contains("astryx-text--inline"));
    assert!(!inline.contains("astryx-text--break-all"));
}

#[test]
fn placement_disabled_tooltip_and_native_passthrough() {
    for placement in [
        LayerPlacement::Above,
        LayerPlacement::Below,
        LayerPlacement::Start,
        LayerPlacement::End,
    ] {
        let html = render(move || {
            rsx! {
                Heading {
                    level: HeadingLevel::Six, max_lines: 2, has_truncate_tooltip: false,
                    truncate_tooltip_placement: placement, id: "native", lang: "fr", dir: "rtl",
                    "data-native": "kept", "aria-describedby": "caller",
                    onclick: move |_| {}, onwheel: move |_| {}, oncopy: move |_| {},
                    ondragstart: move |_| {}, onmounted: move |_| {},
                    "Full text remains"
                }
            }
        });
        for expected in [
            "id=\"native\"",
            "lang=\"fr\"",
            "dir=\"rtl\"",
            "data-native=\"kept\"",
            "aria-describedby=\"caller\"",
            "Full text remains",
        ] {
            assert!(html.contains(expected), "{html}");
        }
        assert!(html.contains(&format!(
            "data-astryx-truncation-placement=\"{}\"",
            placement.as_str()
        )));
        assert!(!html.contains("role=\"tooltip\""));
        assert!(!html.contains("tabindex"));
    }
}
