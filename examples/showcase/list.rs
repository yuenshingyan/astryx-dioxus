use dioxus::prelude::*;

use astryx_dioxus::{List, ListDensity, ListMarkerStyle};

#[component]
pub(super) fn ListExamples() -> Element {
    rsx! {
        section { id: "list-examples",
            h2 { "List" }
            p { "These examples use native list rows instead of ListItem, which is not yet ported." }
            List { id: "settings-list", header: rsx! { h3 { "Settings" } },
                li { "Notifications — Manage your alerts" }
                li { "Privacy — Control your data" }
                li { "Security — Password and 2FA" }
            }
            List { id: "steps-list", header: rsx! { h3 { "Ordered steps" } }, list_style: ListMarkerStyle::Decimal,
                density: ListDensity::Compact, start: 3,
                for step in ["Install the package", "Import components", "Start building"] {
                    li { key: "{step}", style: "grid-column:1 / -1;list-style:decimal inside", "{step}" }
                }
            }
        }
    }
}
