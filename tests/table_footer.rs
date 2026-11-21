use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::TableFooter;

#[test]
fn native_footer_keeps_rows_and_forwards_attributes() {
    let html = render_element(rsx! {
        table {
            tbody { tr { td { "8" } } }
            TableFooter { id: "summary", class: "custom", style: "color:inherit", "data-testid": "footer",
                tr { th { scope: "row", "Total" } td { "8" } }
            }
        }
    });
    for expected in [
        "</tbody><tfoot",
        "class=\"astryx-table-footer custom\"",
        "id=\"summary\"",
        "data-testid=\"footer\"",
        "style=\"color:inherit\"",
        "<tr><th scope=\"row\">Total</th><td>8</td></tr></tfoot>",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}

#[test]
fn default_footer_does_not_add_styles_or_roles() {
    let html = render_element(rsx! { TableFooter { tr { td { "Total" } } } });
    assert_eq!(
        html,
        "<tfoot class=\"astryx-table-footer\"><tr><td>Total</td></tr></tfoot>"
    );
}
