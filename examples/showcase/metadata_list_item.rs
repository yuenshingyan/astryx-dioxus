use dioxus::prelude::*;

use astryx_dioxus::{Badge, BadgeVariant, Link, MetadataListItem};

const GRID: &str = "display:grid;grid-template-columns:max-content minmax(0,1fr);column-gap:var(--spacing-4);row-gap:var(--spacing-2);max-width:480px";

#[component]
pub(super) fn MetadataListItemExamples() -> Element {
    rsx! {
        section { id: "metadata-list-item-examples", "data-astryx-theme": "neutral",
            h2 { "MetadataListItem" }
            h3 { id: "metadata-deployment-title", "Deployment" }
            dl { id: "metadata-deployment", "aria-labelledby": "metadata-deployment-title", style: GRID,
                MetadataListItem { label: "Environment", "data-testid": "metadata-environment", "Production" }
                MetadataListItem { label: "Status", Badge { label: "Healthy", variant: BadgeVariant::GREEN } }
                MetadataListItem { label: "Version", "v2.14.0" }
                MetadataListItem { label: "Last deployed", "June 12, 2026" }
            }
            h3 { id: "metadata-project-title", "Project Details" }
            dl { id: "metadata-project", "aria-labelledby": "metadata-project-title", style: GRID,
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
