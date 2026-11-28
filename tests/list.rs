use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{List, ListEdge, ListItem, ListMarkerStyle, STYLES};

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

#[test]
fn ordered_list_composes_native_and_list_item_rows_without_taking_focus() {
    let html = dioxus_ssr::render_element(rsx! {
        List { list_style: ListMarkerStyle::Decimal,
            li { a { href: "#requirements", "Read requirements" } }
            ListItem { label: "Install" }
            li { button { r#type: "button", "Verify installation" } }
        }
    });
    let (_, rows) = html.split_once('>').unwrap();
    assert!(html.starts_with("<ol "), "{html}");
    let rows = rows
        .strip_prefix("<li><a href=\"#requirements\">Read requirements</a></li><li ")
        .unwrap_or_else(|| panic!("expected a native row followed by a ListItem: {html}"));
    assert!(
        rows.contains("Install</")
            && rows.ends_with(
                "</li><li><button type=\"button\">Verify installation</button></li></ol>"
            ),
        "{html}"
    );
    assert!(!html.contains("tabindex="), "{html}");
}

#[test]
fn decimal_grid_spans_both_tracks_for_direct_native_rows() {
    let selector = ".astryx-list[data-list-style=\"decimal\"] > :is(li, .astryx-list-item)";
    let (_, rule) = STYLES.split_once(selector).expect("missing List row rule");
    let (declarations, _) = rule.split_once('}').unwrap();
    assert!(declarations.contains("grid-column: 1 / -1;"));
}
