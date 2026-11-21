use dioxus::prelude::*;

use astryx_dioxus::NavHeadingMenuItem;

#[component]
pub(super) fn NavHeadingMenuItemExamples() -> Element {
    let mut selected = use_signal(|| false);
    rsx! {
        section { id: "nav-heading-menu-item-examples", "data-astryx-theme": "neutral",
            h2 { "NavHeadingMenuItem" }
            p { "Standalone row preview; the parent menu will supply keyboard navigation." }
            div { role: "menu", "aria-label": "Products", style: "max-width:240px",
                NavHeadingMenuItem { id: "heading-menu-dashboard", label: rsx! { "Dashboard" }, href: "#nav-heading-menu-item-examples" }
                NavHeadingMenuItem { label: rsx! { "Analytics" }, href: "#nav-heading-menu-item-examples" }
                NavHeadingMenuItem { id: "heading-menu-settings", label: rsx! { "Settings" }, description: rsx! { "Manage this workspace" }, on_select: move |_| selected.set(true) }
                NavHeadingMenuItem { id: "heading-menu-disabled", label: rsx! { "Unavailable" }, href: "#unavailable", is_disabled: true }
            }
            if selected() { p { id: "heading-menu-selection", "Settings selected" } }
        }
    }
}
