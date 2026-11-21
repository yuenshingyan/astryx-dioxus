use dioxus::prelude::*;

use astryx_dioxus::SideNavSection;

#[component]
fn Groups(#[props(default)] has_settings: bool) -> Element {
    rsx! {
        SideNavSection { title: "Overview",
            a { href: "#side-nav-section-examples", "aria-current": "page", "Dashboard" }
            a { href: "#side-nav-section-examples", "Analytics" }
        }
        SideNavSection { title: "Account",
            a { href: "#side-nav-section-examples", "Profile" }
            if has_settings { a { href: "#side-nav-section-examples", "Settings" } }
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
            nav { "aria-label": "Basic navigation groups", Groups {} }
            h3 { "Showcase" }
            nav { "aria-label": "Navigation groups showcase", Groups { has_settings: true } }
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
