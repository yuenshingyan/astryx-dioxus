use astryx_dioxus::{Code, CodeColor, CodeSize};
use dioxus::prelude::*;
use dioxus_ssr::render_element;

#[test]
fn one_native_inline_root_preserves_rich_content_and_escapes_text() {
    let html = render_element(rsx! { Code { "<script>&" em { "rich" } } });
    assert!(html.starts_with("<code "), "{html}");
    assert!(html.ends_with("</code>"), "{html}");
    assert_eq!(html.matches("<code ").count(), 1);
    assert!(html.contains("&#60;script&#62;&#38;"), "{html}");
    assert!(html.contains("<em>rich</em>"), "{html}");
    assert!(html.contains("dir=\"ltr\""), "{html}");
    assert!(html.contains("data-color=\"primary\""), "{html}");
    for absent in [
        "data-size",
        "role=",
        "tabindex=",
        "aria-label=",
        "<pre",
        "<span",
    ] {
        assert!(!html.contains(absent), "{absent}: {html}");
    }
}

#[test]
fn every_closed_visual_axis_is_reflected() {
    for (color, name) in [
        (CodeColor::Primary, "primary"),
        (CodeColor::Secondary, "secondary"),
        (CodeColor::Inherit, "inherit"),
    ] {
        let html = render_element(rsx! { Code { color, size: CodeSize::Inherit, "code" } });
        assert!(html.contains(&format!("data-color=\"{name}\"")), "{html}");
        assert!(html.contains("data-size=\"inherit\""), "{html}");
    }
}

#[test]
fn root_attributes_compose_but_cannot_forge_visual_axes() {
    let html = render_element(rsx! {
        Code {
            color: CodeColor::Secondary,
            class: "consumer-class",
            style: "opacity:0.5;margin-top:1px",
            id: "sample",
            dir: "rtl",
            role: "note",
            tabindex: "0",
            "aria-label": "Code sample",
            "data-testid": "code",
            "data-color": "forged",
            "data-size": "forged",
            "x"
        }
    });
    for expected in [
        "class=\"astryx-code consumer-class\"",
        "opacity:0.5;margin-top:1px",
        "id=\"sample\"",
        "dir=\"rtl\"",
        "role=\"note\"",
        "tabindex=\"0\"",
        "aria-label=\"Code sample\"",
        "data-testid=\"code\"",
        "data-color=\"secondary\"",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(!html.contains("forged"), "{html}");
    assert!(!html.contains("data-size"), "{html}");
    assert_eq!(html.matches("dir=").count(), 1, "{html}");
}
