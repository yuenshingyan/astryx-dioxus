use dioxus::prelude::*;

use astryx_dioxus::{SideNav, SideNavItem, SideNavSection};

#[component]
fn Groups(#[props(default)] has_settings: bool) -> Element {
    rsx! {
        SideNavSection { title: "Overview",
            SideNavItem { label: "Dashboard", href: "#side-nav-section-examples", is_selected: true }
            SideNavItem { label: "Analytics", href: "#side-nav-section-examples" }
        }
        SideNavSection { title: "Account",
            SideNavItem { label: "Profile", href: "#side-nav-section-examples" }
            if has_settings { SideNavItem { label: "Settings", href: "#side-nav-section-examples" } }
        }
    }
}

#[component]
pub(super) fn SideNavSectionExamples() -> Element {
    let mut is_hidden = use_signal(|| false);
    rsx! {
        section { id: "side-nav-section-examples", "data-astryx-theme": "neutral",
            h2 { "SideNavSection" }
            h3 { "Basic" }
            SideNav { label: "Basic navigation groups", Groups {} }
            h3 { "Showcase" }
            SideNav { label: "Navigation groups showcase", Groups { has_settings: true } }
            h3 { "Header visibility" }
            button { id: "side-nav-section-toggle", r#type: "button", onclick: move |_| is_hidden.toggle(), "Toggle group header" }
            SideNavSection { id: "side-nav-section-probe", dir: "rtl", style: "width:180px", title: "Workspace", subtitle: "Production", is_header_hidden: is_hidden(),
                end_content: rsx! { span { "2" } },
                a { href: "#side-nav-section-examples", "Projects" }
                a { href: "#side-nav-section-examples", "Reports" }
            }
        }
    }
}
