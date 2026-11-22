use dioxus::prelude::*;

use astryx_dioxus::{Badge, Item, ItemDensity, ItemLayout};

#[component]
pub(super) fn ItemExamples() -> Element {
    rsx! {
        section { id: "item-examples",
            h2 { "Item" }
            h3 { "Basic rows" }
            Item { label: "Quarterly planning", description: Some("Agenda, notes, and action items".into()) }
            Item { label: "Customer research", description: Some("Interview notes from the latest study".into()) }
            Item { label: "Launch checklist", description: Some("Remaining tasks before release".into()) }
            h3 { "Inline metadata" }
            div { style: "width: min(100%, 270px)",
                Item { id: "item-inline-preview", label: "Report", description: Some("Published by the documentation team".into()),
                    density: ItemDensity::Compact, layout: ItemLayout::Inline,
                    end_content: rsx! { Badge { label: "Draft" } },
                }
            }
        }
    }
}
