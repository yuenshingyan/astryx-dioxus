use dioxus::prelude::*;

use astryx_dioxus::{BreadcrumbItem, Breadcrumbs, BreadcrumbsVariant};

const ROW: &str =
    "display:flex;flex-wrap:wrap;gap:var(--spacing-1);list-style:none;margin:0;padding:0";

#[component]
pub(super) fn BreadcrumbItemExamples() -> Element {
    let mut has_last = use_signal(|| true);
    let mut activation = use_signal(|| "");
    rsx! {
        section { id: "breadcrumb-item-examples", "data-astryx-theme": "neutral",
            h2 { "BreadcrumbItem" }
            h3 { "Basic" }
            Breadcrumbs { label: "Settings breadcrumbs",
                BreadcrumbItem { href: "/", "Home" }
                BreadcrumbItem { href: "/settings", "Settings" }
                BreadcrumbItem { is_current: true, "Profile" }
            }
            h3 { "Showcase" }
            Breadcrumbs { label: "Breadcrumb with a start icon",
                BreadcrumbItem { href: "/", start_icon: rsx! {
                    svg { width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", "aria-hidden": "true", path { d: "M3 11l9-8 9 8M6 9v12h12V9m-8 12v-6h4v6" } }
                }, "Home" }
                BreadcrumbItem { href: "/docs", "Docs" }
                BreadcrumbItem { is_current: true, "Components" }
            }
            Breadcrumbs { label: "Breadcrumb ending on the current page",
                BreadcrumbItem { href: "/", "Home" }
                BreadcrumbItem { href: "/settings", "Settings" }
                BreadcrumbItem { is_current: true, "Profile" }
            }
            Breadcrumbs { label: "Supporting breadcrumb", variant: BreadcrumbsVariant::SUPPORTING,
                BreadcrumbItem { href: "/", "Home" }
                BreadcrumbItem { href: "/admin", "Admin" }
                BreadcrumbItem { href: "/admin/users", "Users" }
                BreadcrumbItem { is_current: true, "Permissions" }
            }
            Breadcrumbs { label: "Breadcrumb with click handlers",
                BreadcrumbItem { on_click: move |_| activation.set("Dashboard"), "Dashboard" }
                BreadcrumbItem { id: "breadcrumb-projects", on_click: move |_| activation.set("Projects"), "Projects" }
                BreadcrumbItem { is_current: true, "Project Alpha" }
            }
            output { id: "breadcrumb-action-result", "{activation}" }
            h3 { "Automatic current page" }
            button { id: "breadcrumb-toggle", onclick: move |_| has_last.toggle(), "Toggle last item" }
            ol { id: "breadcrumb-auto", style: ROW,
                BreadcrumbItem { href: "#breadcrumb-item-examples", "Home" }
                if has_last() { BreadcrumbItem { href: "#breadcrumb-auto", "Details" } }
            }
        }
    }
}
