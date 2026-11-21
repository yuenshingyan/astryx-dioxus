use dioxus::prelude::*;

use astryx_dioxus::InputGroupText;

const ROW: &str = "display:flex;width:100%;max-width:400px";
const INPUT: &str = "min-width:0;flex:1;padding:4px 8px;border:var(--border-width) solid var(--color-border-emphasized);font:inherit";

#[component]
pub(super) fn InputGroupTextExamples() -> Element {
    rsx! {
        section { id: "input-group-text-examples", "data-astryx-theme": "neutral",
            h2 { "InputGroupText" }
            h3 { "Basic" }
            div { role: "group", "aria-label": "Price", style: ROW,
                InputGroupText { "$" }
                input { "aria-label": "Amount", placeholder: "0.00", style: INPUT }
                InputGroupText { "USD" }
            }
            h3 { "Showcase" }
            div { role: "group", "aria-label": "Price", style: ROW,
                InputGroupText { "$" }
                input { "aria-label": "Amount", placeholder: "0.00", style: INPUT }
            }
            div { role: "group", "aria-label": "Website", style: ROW,
                InputGroupText { "https://" }
                input { "aria-label": "URL", placeholder: "example", style: INPUT }
                InputGroupText { ".com" }
            }
            h3 { "RTL group" }
            div { dir: "rtl",
                div { id: "input-group-text-rtl-url", dir: "ltr", role: "group", "aria-label": "Website in RTL layout", style: ROW,
                    InputGroupText { "https://" }
                    input { "aria-label": "URL", placeholder: "example", style: INPUT }
                    InputGroupText { ".com" }
                }
                div { id: "input-group-text-rtl", role: "group", "aria-label": "السعر", style: ROW,
                    InputGroupText { "السعر" }
                    input { "aria-label": "المبلغ", placeholder: "0.00", style: INPUT }
                    InputGroupText { "USD" }
                }
            }
        }
    }
}
