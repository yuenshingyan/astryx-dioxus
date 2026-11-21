use dioxus::prelude::*;

use astryx_dioxus::TableHeader;

#[component]
pub(super) fn TableHeaderExamples() -> Element {
    rsx! {
        section { id: "table-header-examples", "data-astryx-theme": "neutral",
            h2 { "TableHeader" }
            table { style: "border-spacing:0;text-align:start",
                caption { "Project status" }
                TableHeader { id: "project-table-header",
                    tr {
                        th { scope: "col", style: "padding:8px 16px", "Project" }
                        th { scope: "col", style: "padding:8px 16px", "Status" }
                    }
                }
                tbody {
                    tr { td { style: "padding:8px 16px", "Documentation" } td { style: "padding:8px 16px", "In review" } }
                }
            }
        }
    }
}
