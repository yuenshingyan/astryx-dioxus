use dioxus::prelude::*;

use astryx_dioxus::{
    AppShellMobileProvider, AppShellMobileState, Icon, MobileNavToggle, SideNavSection,
};

#[component]
fn ToggleExample(is_custom: bool) -> Element {
    let mut is_open = use_signal(|| false);
    let id = if is_custom {
        "mobile-nav-basic-panel"
    } else {
        "mobile-nav-showcase-panel"
    };
    let state = AppShellMobileState {
        is_mobile: true,
        is_mobile_nav_enabled: true,
        is_mobile_nav_open: is_open(),
        mobile_nav_id: Some(id.into()),
        toggle_mobile_nav: Some(EventHandler::new(move |_| is_open.toggle())),
    };
    rsx! {
        AppShellMobileProvider { state,
            div { style: "display:flex;align-items:center;gap:12px",
                if is_custom {
                    MobileNavToggle { label: "Open menu",
                        Icon { icon: rsx! { svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5",
                            rect { x: "3", y: "4.5", width: "18", height: "15", rx: "1.125" }
                            path { d: "M9 4.5v15m6-15v15" }
                        } } }
                    }
                } else { MobileNavToggle {} }
                strong { "Page title" }
            }
            nav { id, hidden: !is_open(), "aria-label": "Navigation",
                SideNavSection { title: "Pages",
                    a { href: "#mobile-nav-toggle-examples", "aria-current": "page", "Home" }
                    a { href: "#mobile-nav-toggle-examples", "Settings" }
                }
            }
        }
    }
}

#[component]
pub(super) fn MobileNavToggleExamples() -> Element {
    rsx! {
        section { id: "mobile-nav-toggle-examples", "data-astryx-theme": "neutral",
            h2 { "MobileNavToggle" }
            p { "These examples use an inline navigation panel in place of a drawer." }
            h3 { "Basic" }
            ToggleExample { is_custom: true }
            h3 { "Showcase" }
            ToggleExample { is_custom: false }
        }
    }
}
