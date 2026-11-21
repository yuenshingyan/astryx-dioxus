use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::TableBody;

#[test]
fn native_body_keeps_rows_and_forwards_attributes() {
    let html = render_element(rsx! {
        table {
            TableBody { id: "projects", class: "custom", style: "color:inherit", "data-testid": "body",
                tr { th { scope: "row", "Review" } td { "4" } }
            }
            TableBody { tr { td { "Other" } } }
        }
    });
    for expected in [
        "class=\"astryx-table-body custom\"",
        "id=\"projects\"",
        "style=\"color:inherit\"",
        "data-testid=\"body\"",
        "<tr><th scope=\"row\">Review</th><td>4</td></tr></tbody><tbody class=\"astryx-table-body\"><tr><td>Other</td></tr></tbody>",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}

#[test]
fn default_body_does_not_add_styles_or_roles() {
    let html = render_element(rsx! { TableBody { tr { td { "Review" } } } });
    assert_eq!(
        html,
        "<tbody class=\"astryx-table-body\"><tr><td>Review</td></tr></tbody>"
    );
}
