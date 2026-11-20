use astryx_dioxus::{AspectRatio, AspectRatioFit, AspectRatioShape};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn required_ratio_defaults_and_caller_child_ownership() {
    let html = render(
        || rsx! { AspectRatio { ratio:16.0/9.0, img { src:"local.svg", alt:"Caller description" } } },
    );
    assert_eq!(html.matches("<div").count(), 2, "{html}");
    assert!(html.contains("--astryx-aspect-ratio:1.7777777777777777"));
    assert!(html.contains("data-shape=\"rectangle\""));
    assert!(html.contains("alt=\"Caller description\""));
    for absent in [
        "data-astryx-aspect-ratio-override",
        "role=",
        "aria-",
        "tabindex=",
        "aspect-ratio:var",
        "<script",
        "<style",
    ] {
        assert!(!html.contains(absent), "{absent}: {html}");
    }
}

#[test]
fn all_fits_only_mark_the_content_slot_and_never_edit_child_props() {
    let html = render(|| {
        rsx! {
            for fit in [AspectRatioFit::Cover,AspectRatioFit::Contain,AspectRatioFit::Center] {
                AspectRatio { ratio:1.0, fit, shape:AspectRatioShape::Ellipse,
                    img { class:"consumer", style:"object-fit:contain;width:40px", alt:"", src:"local.svg" }
                }
            }
        }
    });
    for fit in ["cover", "contain", "center"] {
        assert!(
            html.contains(&format!("data-astryx-aspect-ratio-override=\"{fit}\"")),
            "{html}"
        );
    }
    assert_eq!(html.matches("data-shape=\"ellipse\"").count(), 3);
    assert_eq!(html.matches("class=\"consumer\"").count(), 3);
    assert_eq!(
        html.matches("style=\"object-fit:contain;width:40px\"")
            .count(),
        3
    );
    assert!(!html.contains("data-fit="));
    assert!(!html.contains("astryx-aspect-ratio cover"));
}

#[test]
fn root_native_attributes_and_style_override_are_preserved() {
    let html = render(
        || rsx! { AspectRatio { ratio:3.0, class:"consumer", style:"aspect-ratio:3 / 2;opacity:.5", id:"root", lang:"fr", role:"group", "aria-label":"Caller group", "data-test":"passthrough", tabindex:0, div { "Content" } } },
    );
    for expected in [
        "astryx-aspect-ratio consumer",
        "--astryx-aspect-ratio:3;aspect-ratio:3 / 2;opacity:.5",
        "id=\"root\"",
        "lang=\"fr\"",
        "role=\"group\"",
        "aria-label=\"Caller group\"",
        "data-test=\"passthrough\"",
        "tabindex=0",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
}

#[test]
fn numeric_invalid_inputs_do_not_panic_or_invent_positive_fallbacks() {
    let html = render(|| {
        rsx! {
            for ratio in [0.0,-1.0,f64::NAN,f64::INFINITY] {
                AspectRatio { ratio, div {} }
            }
        }
    });
    for expected in [
        "--astryx-aspect-ratio:0;",
        "--astryx-aspect-ratio:-1;",
        "--astryx-aspect-ratio:invalid;",
    ] {
        assert!(html.contains(expected), "{html}");
    }
}
