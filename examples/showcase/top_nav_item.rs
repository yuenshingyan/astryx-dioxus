use dioxus::prelude::*;

use astryx_dioxus::TopNavItem;

#[component]
pub(super) fn TopNavItemExamples() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        section { id: "top-nav-item-examples",
            h2 { "TopNavItem" }
            nav { "aria-label": "Navigation items demo",
                TopNavItem { label: "Dashboard", href: "#top-nav-item-examples", is_selected: true }
                TopNavItem { label: "Projects", href: "#top-nav-item-examples" }
                TopNavItem { label: "Reports", href: "#top-nav-item-examples" }
                TopNavItem { label: "Archived", href: "#top-nav-item-examples", is_disabled: true }
                TopNavItem { label: "Refresh", onclick: move |_| count += 1 }
            }
            output { id: "top-nav-item-count", "{count}" }
        }
    }
}
