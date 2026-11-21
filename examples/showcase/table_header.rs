use dioxus::prelude::*;

use astryx_dioxus::{TableFooter, TableHeader};

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
        section { id: "table-footer-examples", "data-astryx-theme": "neutral",
            h2 { "TableFooter" }
            table { style: "border-spacing:0;text-align:start",
                caption { "Project hours" }
                TableHeader {
                    tr { th { scope: "col", "Project" } th { scope: "col", "Hours" } }
                }
                tbody {
                    tr { th { scope: "row", "Documentation" } td { "8" } }
                    tr { th { scope: "row", "Review" } td { "4" } }
                }
                TableFooter { id: "project-table-footer",
                    tr { th { scope: "row", "Total" } td { "12" } }
                }
            }
        }
    }
}
