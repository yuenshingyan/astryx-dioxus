use dioxus::prelude::*;

use astryx_dioxus::{Button, ButtonVariant, SideNav, SideNavItem, SideNavSection};

#[component]
pub(super) fn SideNavExamples() -> Element {
    rsx! {
        section { id: "side-nav-examples",
            h2 { "SideNav" }
            div { style: "block-size:240px",
                SideNav { id: "workspace-side-nav", "aria-label": "Workspace navigation",
                    header: rsx! { strong { "Workspace" } },
                    footer_icons: rsx! { Button { label: "Preferences", variant: ButtonVariant::Ghost } },
                    SideNavSection { title: "Main",
                        SideNavItem { label: "Dashboard", href: "#side-nav-examples", is_selected: true }
                        SideNavItem { label: "Projects", href: "#side-nav-examples" }
                        SideNavItem { label: "Analytics", href: "#side-nav-examples" }
                    }
                    SideNavSection { title: "Documents",
                        SideNavItem { label: "All Documents", href: "#side-nav-examples" }
                        SideNavItem { label: "Recent", href: "#side-nav-examples" }
                        SideNavItem { label: "Archived", href: "#side-nav-examples" }
                    }
                }
            }
        }
    }
}
