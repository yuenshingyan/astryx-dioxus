use dioxus::prelude::*;

use astryx_dioxus::{List, ListDensity, ListEdge, ListItem, ListMarkerStyle};

#[component]
pub(super) fn ListExamples() -> Element {
    rsx! {
        section { id: "list-examples",
            h2 { "List" }
            List { id: "settings-list", header: rsx! { h3 { "Settings" } },
                ListItem { label: "Notifications", description: Some("Manage your alerts".into()) }
                ListItem { label: "Privacy", description: Some("Control your data".into()) }
                ListItem { label: "Security", description: Some("Password and 2FA".into()) }
            }
            List { id: "steps-list", header: rsx! { h3 { "Ordered steps" } }, list_style: ListMarkerStyle::Decimal,
                density: ListDensity::Compact, start: 3,
                for step in ["Install the package", "Import components", "Start building"] {
                    ListItem { key: "{step}", label: step }
                }
            }
            List { id: "native-rows-list", header: rsx! { h3 { "Setup resources" } }, list_style: ListMarkerStyle::Decimal,
                li { a { href: "#settings-list", "Review settings" } }
                li { a { href: "#steps-list", "Read the installation steps" } }
            }
            h3 { "ListItem: Bullets and dividers" }
            List { id: "list-item-features", list_style: ListMarkerStyle::Circle,
                edge: ListEdge::Flush, density: ListDensity::Spacious, has_dividers: true,
                ListItem { label: "Notifications" }
                ListItem { label: "Privacy" }
            }
        }
    }
}
