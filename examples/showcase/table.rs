use dioxus::prelude::*;

use astryx_dioxus::{
    Table, TableBody, TableCell, TableCellScope, TableDensity, TableDividers, TableHeader,
    TableHeaderCell, TableRow,
};

#[component]
pub(super) fn TableExamples() -> Element {
    rsx! {
        section { id: "table-examples", "data-astryx-theme": "neutral",
            h2 { "Table" }
            h3 { "Showcase" }
            Table { label: "Team members", has_hover: true,
                TableHeader {
                    TableRow {
                        for heading in ["Name", "Email", "Role", "Age"] {
                            TableHeaderCell { key: "{heading}", scope: TableCellScope::Column, "{heading}" }
                        }
                    }
                }
                TableBody {
                    for (name, email, role, age) in [("Alice Johnson", "alice@example.com", "Engineer", 30), ("Bob Smith", "bob@example.com", "Designer", 25), ("Charlie Brown", "charlie@example.com", "PM", 35)] {
                        TableRow { key: "{name}", TableCell { "{name}" } TableCell { "{email}" } TableCell { "{role}" } TableCell { "{age}" } }
                    }
                }
            }
            h3 { id: "quarterly-heading", "Grid dividers" }
            div { style: "max-width:320px",
                Table { id: "quarterly-table", labelled_by: "quarterly-heading", density: TableDensity::Compact, dividers: TableDividers::Grid, style: "min-width:480px",
                    TableHeader {
                        TableRow {
                            for heading in ["Metric", "Q1", "Q2", "Q3", "Q4"] {
                                TableHeaderCell { key: "{heading}", scope: TableCellScope::Column, "{heading}" }
                            }
                        }
                    }
                    TableBody {
                        for values in [["Revenue", "$1.2M", "$1.4M", "$1.3M", "$1.8M"], ["Users", "12,400", "14,200", "15,800", "18,100"], ["Retention", "82%", "85%", "84%", "88%"], ["NPS", "42", "45", "48", "51"], ["CSAT", "4.2", "4.3", "4.5", "4.6"]] {
                            TableRow { key: "{values[0]}",
                                for (index, value) in values.into_iter().enumerate() {
                                    TableCell { key: "{index}", style: (index > 0).then_some("text-align:end".to_owned()), "{value}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
