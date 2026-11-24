use dioxus::prelude::*;

use astryx_dioxus::{TopNav, TopNavHeading, TopNavItem, TopNavMenu, TopNavMenuItemData};

#[component]
pub(super) fn TopNavMenuExamples() -> Element {
    let items = [
        (
            "Analytics",
            "View traffic and engagement metrics",
            "#tools-analytics",
        ),
        (
            "Team Members",
            "Manage your team and permissions",
            "#tools-team",
        ),
        ("Settings", "Configure your workspace", "#tools-settings"),
    ]
    .into_iter()
    .map(|(title, description, href)| TopNavMenuItemData {
        title: title.into(),
        description: Some(description.into()),
        href: Some(href.into()),
        icon: None,
        onclick: None,
        should_keep_open: false,
    })
    .collect();
    rsx! {
        section { id: "top-nav-menu-examples",
            h2 { "Top Nav Menu" }
            div { style: "max-inline-size:600px",
                TopNav { label: "Menu demo", heading: rsx! { TopNavHeading { heading: "Platform" } },
                    start_content: rsx! {
                        TopNavItem { label: "Home", href: "#top-nav-menu-examples", is_selected: true }
                        TopNavMenu { label: "Tools", items }
                    },
                    end_content: rsx! { button { id: "tools-search", r#type: "button", "Search" } },
                }
            }
            p { id: "tools-analytics", "Analytics" }
            p { id: "tools-team", "Team Members" }
            p { id: "tools-settings", "Settings" }
        }
    }
}
