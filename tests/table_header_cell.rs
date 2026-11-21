use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{TableCellScope, TableHeaderCell};

#[test]
fn standalone_header_cell_adds_only_its_theme_target() {
    let html = render_element(rsx! { TableHeaderCell { "Hours" } });
    assert_eq!(html, "<th class=\"astryx-table-header-cell\">Hours</th>");
}

#[test]
fn header_cell_forwards_scope_and_caller_attributes() {
    for (scope, value) in [
        (TableCellScope::Column, "col"),
        (TableCellScope::Row, "row"),
        (TableCellScope::ColumnGroup, "colgroup"),
        (TableCellScope::RowGroup, "rowgroup"),
    ] {
        let html = render_element(rsx! {
            TableHeaderCell { scope, id: "hours", class: "custom", style: "color:inherit", colspan: 2,
                "data-testid": "heading", "Hours"
            }
        });
        for expected in [
            format!("scope=\"{value}\""),
            "id=\"hours\"".into(),
            "class=\"astryx-table-header-cell custom\"".into(),
            "style=\"color:inherit\"".into(),
            "colspan=2 ".into(),
            "data-testid=\"heading\"".into(),
        ] {
            assert!(html.contains(&expected), "missing {expected}: {html}");
        }
    }
}
