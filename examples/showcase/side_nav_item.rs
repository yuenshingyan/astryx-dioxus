use dioxus::prelude::*;

use astryx_dioxus::{Badge, SideNavItem, SideNavSection};

#[component]
pub(super) fn SideNavItemExamples() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        section { id: "side-nav-item-examples",
            h2 { "SideNavItem" }
            nav { "aria-label": "Workspace", style: "max-inline-size:280px",
                SideNavSection { title: "Workspace",
                    SideNavItem { label: "Inbox", href: "#side-nav-item-examples", is_selected: true,
                        end_content: rsx! { Badge { label: "12" } },
                    }
                    SideNavItem { label: "Documents", href: "#side-nav-item-examples" }
                    SideNavItem { label: "Settings", href: "#side-nav-item-examples", is_disabled: true, onclick: move |_| count += 1 }
                    SideNavItem { label: "Refresh", onclick: move |_| count += 1 }
                }
            }
            output { id: "side-nav-item-count", "{count}" }
        }
    }
}
