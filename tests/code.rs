use std::cell::Cell;
use std::rc::Rc;

use dioxus::core::{AttributeValue, Mutation};
use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Code, CodeColor, CodeSize};

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
fn optional_direction_keeps_the_ltr_default_until_a_caller_supplies_a_value() {
    for (direction, expected) in [(None, "ltr"), (Some("rtl"), "rtl"), (Some("auto"), "auto")] {
        let html = render_element(rsx! {
            p { dir: "rtl", "טקסט " Code { dir: direction, "foo(1)" } " טקסט" }
        });
        let (_, code) = html.split_once("<code ").expect("native code in RTL prose");
        let (attrs, content) = code.split_once('>').expect("code opening tag");
        assert!(attrs.contains(&format!("dir=\"{expected}\"")), "{html}");
        assert_eq!(attrs.matches("dir=").count(), 1, "{html}");
        assert!(content.starts_with("foo(1)</code>"), "{html}");
    }
}

#[test]
fn changing_direction_updates_the_same_root_attribute() {
    type Direction = Rc<Cell<Option<&'static str>>>;

    fn app() -> Element {
        let direction = use_context::<Direction>().get();
        rsx! { Code { dir: direction, "foo(1)" } }
    }

    let direction: Direction = Rc::new(Cell::new(Some("rtl")));
    let mut dom = VirtualDom::new(app).with_root_context(direction.clone());
    let root = dom
        .rebuild_to_vec()
        .edits
        .into_iter()
        .find_map(|edit| match edit {
            Mutation::SetAttribute {
                name: "dir",
                value: AttributeValue::Text(value),
                id,
                ..
            } if value == "rtl" => Some(id),
            _ => None,
        })
        .expect("initial RTL direction");

    for (next, expected) in [(None, "ltr"), (Some("auto"), "auto")] {
        direction.set(next);
        dom.mark_dirty(ScopeId::APP);
        let edits: Vec<_> = dom
            .render_immediate_to_vec()
            .edits
            .into_iter()
            .filter(|edit| matches!(edit, Mutation::SetAttribute { name: "dir", .. }))
            .collect();
        assert!(
            matches!(
                edits.as_slice(),
                [Mutation::SetAttribute { value: AttributeValue::Text(value), id, .. }]
                    if value == expected && *id == root
            ),
            "{edits:?}"
        );
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
