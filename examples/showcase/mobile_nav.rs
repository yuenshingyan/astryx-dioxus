use dioxus::prelude::*;

use astryx_dioxus::{Button, MobileNav, MobileNavSide, SideNavItem, SideNavSection};

#[component]
pub(super) fn MobileNavExamples() -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        section { id: "mobile-nav-examples",
            h2 { "MobileNav" }
            div { dir: "rtl",
                Button { id: "open-mobile-nav-preview", label: "Open navigation", "aria-controls": "mobile-nav-preview",
                    "aria-haspopup": "dialog", onclick: move |_| is_open.set(true),
                }
                MobileNav { id: "mobile-nav-preview", label: "Workspace navigation", side: MobileNavSide::Start,
                    is_open: is_open(), on_open_change: move |open| is_open.set(open),
                    header: rsx! { h3 { "Navigation" } },
                    SideNavSection { title: "Main",
                        SideNavItem { label: "Dashboard", href: "#mobile-nav-examples", is_selected: true }
                        SideNavItem { label: "Projects", href: "#mobile-nav-examples" }
                    }
                    SideNavSection { title: "Settings",
                        SideNavItem { label: "General", href: "#mobile-nav-examples" }
                    }
                }
            }
        }
    }
}
