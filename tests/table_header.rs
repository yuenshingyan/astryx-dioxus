use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::TableHeader;

#[test]
fn native_header_keeps_rows_and_forwards_attributes() {
    let html = render_element(rsx! {
        table {
            TableHeader { id: "projects", class: "custom", style: "color:inherit", "data-testid": "header",
                tr { th { scope: "col", "Project" } }
            }
            tbody { tr { td { "Documentation" } } }
        }
    });
    for expected in [
        "class=\"astryx-table-header custom\"",
        "id=\"projects\"",
        "data-testid=\"header\"",
        "style=\"color:inherit\"",
        "<tr><th scope=\"col\">Project</th></tr></thead><tbody>",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}

#[test]
fn default_header_does_not_add_styles_or_roles() {
    let html = render_element(rsx! { TableHeader { tr { th { "Status" } } } });
    assert_eq!(
        html,
        "<thead class=\"astryx-table-header\"><tr><th>Status</th></tr></thead>"
    );
}
