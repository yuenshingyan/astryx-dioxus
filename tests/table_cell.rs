use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{TableCell, TableCellScope};

#[test]
fn standalone_cell_adds_only_its_theme_target() {
    let html = render_element(rsx! { TableCell { "Hours" } });
    assert_eq!(html, "<td class=\"astryx-table-cell\">Hours</td>");
}

#[test]
fn cell_forwards_native_relationships_and_caller_attributes() {
    let html = render_element(rsx! {
        TableCell { id: "hours", class: "custom", style: "color:inherit", headers: "project hours-column",
            scope: TableCellScope::ColumnGroup, colspan: 2, rowspan: 3, "data-testid": "hours-cell", "12" }
    });
    for expected in [
        "id=\"hours\"",
        "class=\"astryx-table-cell custom\"",
        "style=\"color:inherit\"",
        "headers=\"project hours-column\"",
        "scope=\"colgroup\"",
        "colspan=2 ",
        "rowspan=3 ",
        "data-testid=\"hours-cell\"",
        ">12</td>",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}
