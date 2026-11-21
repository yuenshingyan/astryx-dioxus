use dioxus::prelude::*;

use astryx_dioxus::{
    Button, ElementSize, FieldStatusInput, InputGroup, InputGroupText, InputStatusType,
};

const INPUT: &str = "min-width:0;flex:1;margin-inline-start:-1px;padding-inline:8px;border:var(--border-width) solid var(--color-border-emphasized);border-radius:0;font:inherit";

#[component]
pub(super) fn InputGroupExamples() -> Element {
    rsx! {
        section { id: "input-group-examples", "data-astryx-theme": "neutral",
            h2 { "InputGroup" }
            div { style: "max-width:400px;display:grid;gap:16px",
                h3 { "Basic" }
                InputGroup { label: "Price",
                    InputGroupText { "$" }
                    input { "aria-label": "Amount", placeholder: "0.00", style: INPUT }
                    InputGroupText { "USD" }
                }
                h3 { "Showcase" }
                InputGroup { label: "Price",
                    InputGroupText { "$" }
                    input { "aria-label": "Amount", placeholder: "0.00", style: INPUT }
                }
                InputGroup { label: "Website", dir: "ltr",
                    InputGroupText { "https://" }
                    input { "aria-label": "URL", placeholder: "example", style: INPUT }
                    InputGroupText { ".com" }
                }
                h3 { "Button addon and status" }
                InputGroup { id: "input-group-probe", label: "Search", description: "Search your workspace", size: ElementSize::Small,
                    status: FieldStatusInput { status_type: InputStatusType::Warning, message: Some("Results may be incomplete".into()), message_id: None },
                    InputGroupText { "Query" }
                    Button { id: "input-group-addon", label: "Search" }
                    InputGroupText { "All" }
                }
            }
        }
    }
}
