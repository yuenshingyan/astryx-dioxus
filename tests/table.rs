use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Table, TableBody, TableCell, TableDensity, TableDividers, TableRow};

#[test]
fn table_keeps_native_structure_and_names_its_scroll_region() {
    let html = render_element(rsx! {
        Table { label: "Usage", id: "usage", density: TableDensity::Compact, dividers: TableDividers::Grid,
            TableBody { TableRow { TableCell { "42" } } }
        }
    });
    assert_eq!(html.matches("aria-label=\"Usage\"").count(), 2, "{html}");
    for expected in [
        "role=\"group\"",
        "data-density=\"compact\"",
        "data-dividers=\"grid\"",
        "id=\"usage\"",
        "<tbody class=\"astryx-table-body\"><tr class=\"astryx-table-row\"><td class=\"astryx-table-cell\">42</td></tr></tbody></table>",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}
