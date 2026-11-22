use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{List, ListEdge, ListMarkerStyle};

#[test]
fn ordered_list_uses_native_start_and_links_its_header() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            List { header: rsx! { "Steps" }, list_style: ListMarkerStyle::Decimal, start: 3, role: "group",
                li { "Install" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let id = html
        .split("aria-labelledby=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    assert!(html.contains(&format!("id=\"{id}\"")), "{html}");
    assert!(
        html.contains("<ol ") && html.contains("role=\"list\"") && html.contains("start=3"),
        "{html}"
    );
    assert!(!html.contains("role=\"group\""));
}

#[test]
fn unheaded_list_preserves_caller_name_and_does_not_emit_start() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            List { edge: ListEdge::Bleed, start: 3, "aria-labelledby": "external-name", li { "Option" } }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(
        html.starts_with("<ul ") && html.contains("aria-labelledby=\"external-name\""),
        "{html}"
    );
    assert!(html.contains("data-container-corner=") && !html.contains(" start="));
}
