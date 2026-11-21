use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{TableBody, TableCell, TableRow};

#[test]
fn default_row_adds_only_its_theme_target() {
    let html = render_element(rsx! { TableRow { td { "Review" } } });
    assert_eq!(html, "<tr class=\"astryx-table-row\"><td>Review</td></tr>");
}

#[test]
fn row_preserves_its_section_and_forwards_attributes() {
    let html = render_element(rsx! {
        table {
            TableBody {
                TableRow { id: "review", class: "custom", style: "color:inherit", "data-testid": "row",
                    TableCell { "Review" }
                }
            }
        }
    });
    for expected in [
        "<tbody class=\"astryx-table-body\"><tr ",
        "class=\"astryx-table-row custom\"",
        "id=\"review\"",
        "style=\"color:inherit\"",
        "data-testid=\"row\"",
        "<td class=\"astryx-table-cell\">Review</td></tr></tbody>",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}
