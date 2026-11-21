use dioxus::prelude::*;

use astryx_dioxus::{Badge, BadgeVariant, Link, MetadataList, MetadataListItem};

#[component]
pub(super) fn MetadataListItemExamples() -> Element {
    rsx! {
        section { id: "metadata-list-item-examples", "data-astryx-theme": "neutral",
            h2 { "MetadataListItem" }
            MetadataList { id: "metadata-deployment",
                title: rsx! { h3 { "Deployment" } },
                MetadataListItem { label: "Environment", "data-testid": "metadata-environment", "Production" }
                MetadataListItem { label: "Status", Badge { label: "Healthy", variant: BadgeVariant::GREEN } }
                MetadataListItem { label: "Version", "v2.14.0" }
                MetadataListItem { label: "Last deployed", "June 12, 2026" }
            }
            MetadataList { id: "metadata-project",
                title: rsx! { h3 { "Project Details" } },
                MetadataListItem { label: "Name", "Design System v2" }
                MetadataListItem { label: "Status", Badge { label: "Active", variant: BadgeVariant::GREEN } }
                MetadataListItem { label: "Owner", Link { href: "#", "Alice Johnson" } }
                MetadataListItem { label: "Created", "January 15, 2025" }
                MetadataListItem { label: "Priority", Badge { label: "High", variant: BadgeVariant::RED } }
                MetadataListItem { label: "Repository", Link { href: "#", "github.com/org/design-system" } }
            }
        }
    }
}
