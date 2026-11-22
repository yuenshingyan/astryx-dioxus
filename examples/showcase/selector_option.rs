use dioxus::prelude::*;

use astryx_dioxus::{Badge, Icon, IconSize, ItemLayout, SelectorOption};

const ROLES: [(&str, &str); 4] = [
    ("Admin", "Full access to all resources"),
    ("Editor", "Can edit and publish content"),
    ("Viewer", "Read-only access"),
    ("Billing", "Manage plans and payments"),
];

fn role_icon() -> Element {
    rsx! {
        Icon { size: IconSize::Small, color: "var(--color-icon-secondary)", icon: rsx! {
            svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5",
                circle { cx: "12", cy: "8", r: "4" }
                path { d: "M4 21v-1a8 8 0 0 1 16 0v1Z" }
            }
        } }
    }
}

#[component]
pub(super) fn SelectorOptionExamples() -> Element {
    rsx! {
        section { id: "selector-option-examples",
            h2 { "SelectorOption" }
            p { "These previews show row content without a Selector trigger or popup." }
            h3 { "Showcase" }
            div { style: "max-width:300px",
                for (label, description) in ROLES {
                    SelectorOption { key: "{label}", label, description: Some(description.into()), icon: role_icon() }
                }
            }
            h3 { "Basic" }
            div { style: "max-width:300px",
                for (label, description) in ROLES.into_iter().take(3) {
                    SelectorOption { key: "{label}", label, description: Some(description.into()) }
                }
            }
            h3 { "Descriptions" }
            div { id: "selector-option-descriptions", style: "max-width:200px",
                SelectorOption { label: "Private" }
                SelectorOption { label: "Private", description: Some("Only members can access this space and its content.".into()) }
                SelectorOption { label: "Public", description: Some("Anyone at the company can find and join this space.".into()),
                    layout: ItemLayout::Inline, end_content: rsx! { Badge { label: "Default" } },
                }
            }
        }
    }
}
