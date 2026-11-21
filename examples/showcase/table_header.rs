use dioxus::prelude::*;

use astryx_dioxus::{TableBody, TableCell, TableFooter, TableHeader};

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
                TableBody {
                    tr { td { style: "padding:8px 16px", "Documentation" } td { style: "padding:8px 16px", "In review" } }
                }
            }
        }
        section { id: "table-footer-examples", "data-astryx-theme": "neutral",
            h2 { "TableBody and TableFooter" }
            table { style: "border-spacing:0;text-align:start",
                caption { "Project hours" }
                TableHeader {
                    tr { th { scope: "col", "Project" } th { id: "project-hours-column", scope: "col", "Hours" } }
                }
                TableBody { id: "project-table-body",
                    tr { th { id: "project-docs-row", scope: "row", "Documentation" } TableCell { id: "project-hours-cell", headers: "project-docs-row project-hours-column", "8" } }
                    tr { th { scope: "row", "Review" } TableCell { "4" } }
                }
                TableFooter { id: "project-table-footer",
                    tr { th { scope: "row", "Total" } td { "12" } }
                }
            }
        }
    }
}
