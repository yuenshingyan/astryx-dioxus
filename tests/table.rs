use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Table, TableBody, TableCell, TableDensity, TableDividers, TableRow};

fn opening_tag<'a>(html: &'a str, tag: &str) -> &'a str {
    html.split_once(&format!("<{tag} "))
        .expect("opening tag")
        .1
        .split_once('>')
        .expect("end of opening tag")
        .0
}

#[test]
fn table_keeps_native_structure_and_names_its_scroll_region() {
    let html = render_element(rsx! {
        Table { label: "Usage", id: "usage", density: TableDensity::Compact, dividers: TableDividers::Grid,
            TableBody { TableRow { TableCell { "42" } } }
        }
    });
    let viewport = opening_tag(&html, "div");
    let table = opening_tag(&html, "table");
    assert!(viewport.contains("role=\"group\""), "{html}");
    assert!(!viewport.contains("id=\"usage\""), "{html}");
    for tag in [viewport, table] {
        assert!(tag.contains("aria-label=\"Usage\""), "{html}");
    }
    for expected in [
        "data-density=\"compact\"",
        "data-dividers=\"grid\"",
        "id=\"usage\"",
    ] {
        assert!(table.contains(expected), "missing {expected}: {html}");
    }
    assert!(
        html.contains("><div class=\"astryx-scrollable-area__content\" data-scroll-content=\"\""),
        "{html}"
    );
    assert!(html.contains("<tbody class=\"astryx-table-body\"><tr class=\"astryx-table-row\"><td class=\"astryx-table-cell\">42</td></tr></tbody></table></div></div>"), "{html}");
}

#[test]
fn table_uses_the_generic_label_only_on_the_viewport() {
    let html = render_element(rsx! { Table { caption { "Usage" } } });
    let viewport = opening_tag(&html, "div");
    let table = opening_tag(&html, "table");
    assert!(viewport.contains("aria-label=\"Table\""), "{html}");
    assert!(!viewport.contains("aria-labelledby="), "{html}");
    assert!(!table.contains("aria-label"), "{html}");
    assert!(html.contains("><caption>Usage</caption></table>"), "{html}");
}

#[test]
fn table_labelled_by_does_not_add_a_generic_viewport_label() {
    let html = render_element(rsx! {
        h2 { id: "usage-title", "Usage" }
        Table { labelled_by: "usage-title", TableBody {} }
    });
    for tag in [opening_tag(&html, "div"), opening_tag(&html, "table")] {
        assert!(tag.contains("aria-labelledby=\"usage-title\""), "{html}");
        assert!(!tag.contains("aria-label="), "{html}");
    }
}

#[test]
fn table_viewport_prefers_labelled_by_but_native_table_keeps_both_attributes() {
    let html = render_element(rsx! {
        h2 { id: "usage-title", "Usage" }
        Table { "aria-label": "Fallback", "aria-labelledby": "usage-title", TableBody {} }
    });
    let viewport = opening_tag(&html, "div");
    let table = opening_tag(&html, "table");
    for tag in [viewport, table] {
        assert!(tag.contains("aria-labelledby=\"usage-title\""), "{html}");
    }
    assert!(!viewport.contains("aria-label="), "{html}");
    assert!(table.contains("aria-label=\"Fallback\""), "{html}");
}
