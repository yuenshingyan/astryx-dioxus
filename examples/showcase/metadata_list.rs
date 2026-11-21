use dioxus::prelude::*;

use astryx_dioxus::{
    Badge, MetadataList, MetadataListColumns, MetadataListItem, MetadataListOrientation,
};

const ITEMS: [(&str, &str); 6] = [
    ("Name", "MetadataList"),
    ("Status", "Active"),
    ("Owner", "Joey"),
    ("Created", "Jan 15, 2026"),
    ("Updated", "Mar 26, 2026"),
    ("Priority", "Tier 1"),
];

#[component]
pub(super) fn MetadataListExamples() -> Element {
    rsx! {
        section { id: "metadata-list-examples", "data-astryx-theme": "neutral",
            h2 { "MetadataList" }
            for (id, title) in [("showcase", "Showcase"), ("basic", "Basic metadata")] {
                h3 { key: "{id}-title", "{title}" }
                MetadataList { id: "metadata-list-{id}",
                    for (label, value) in ITEMS.into_iter().take(3) {
                        MetadataListItem { key: "{label}", label, "{value}" }
                    }
                }
            }
            h3 { "Horizontal metadata" }
            MetadataList { id: "metadata-list-horizontal", orientation: MetadataListOrientation::Horizontal,
                MetadataListItem { label: "Status", "Active" }
                MetadataListItem { label: "Type", "Premium" }
                MetadataListItem { label: "Owner", "Joey" }
                MetadataListItem { label: "Created", "Jan 15, 2026" }
            }
            h3 { "Multi-column metadata" }
            MetadataList { id: "metadata-list-multi", columns: MetadataListColumns::Multi,
                for (label, value) in ITEMS.into_iter().take(4) {
                    MetadataListItem { key: "{label}", label, "{value}" }
                }
                MetadataListItem { label: "Tags", span { style: "display:inline-flex;gap:var(--spacing-1)",
                    Badge { label: "component" } Badge { label: "xds" }
                } }
                MetadataListItem { label: "Priority", "Tier 1" }
            }
            h3 { "Collapsible metadata" }
            MetadataList { id: "metadata-list-collapsible", max_num_of_items: 3,
                MetadataListItem { label: "Name", "MetadataList" }
                for (label, value) in ITEMS.into_iter().skip(1) {
                    MetadataListItem { key: "{label}", label, "{value}" }
                }
            }
        }
    }
}
