use dioxus::prelude::*;

use astryx_dioxus::{BreadcrumbItem, Breadcrumbs, BreadcrumbsVariant, SpacingStep, VStack};

#[component]
fn TrailIcon(is_home: bool) -> Element {
    let path = if is_home {
        "M3 11l9-8 9 8M6 9v12h12V9m-8 12v-6h4v6"
    } else {
        "M12 3v4m0 10v4M3 12h4m10 0h4M6 6l3 3m6 6 3 3M6 18l3-3m6-6 3-3"
    };
    rsx! { svg { width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", "aria-hidden": "true", path { d: path } } }
}

#[component]
pub(super) fn BreadcrumbsExamples() -> Element {
    let mut is_supporting = use_signal(|| false);
    rsx! {
        section { id: "breadcrumbs-examples", "data-astryx-theme": "neutral",
            h2 { "Breadcrumbs" }
            VStack { gap: SpacingStep::Two,
                h3 { "Showcase" }
                Breadcrumbs { id: "breadcrumbs-default",
                    BreadcrumbItem { href: "/", "Home" }
                    BreadcrumbItem { href: "/projects", "Projects" }
                    BreadcrumbItem { is_current: true, "My Project" }
                }
                h3 { "Supporting variant" }
                for (label, variant) in [("Default breadcrumb", BreadcrumbsVariant::DEFAULT), ("Supporting breadcrumb", BreadcrumbsVariant::SUPPORTING)] {
                    Breadcrumbs { key: "{label}", label, variant,
                        BreadcrumbItem { href: "/", "Home" }
                        BreadcrumbItem { href: "/projects", "Projects" }
                        BreadcrumbItem { is_current: true, "My Project" }
                    }
                }
                h3 { "Custom separators" }
                for (name, mark) in [("Chevron", "›"), ("Arrow", "→"), ("Dot", "·")] {
                    div { key: "{name}",
                        p { "{name}" }
                        Breadcrumbs { label: "Breadcrumb, {name} separator", separator: rsx! {
                            span { class: if mark == "→" { "astryx-breadcrumb-item__slash" } else { "" }, "{mark}" }
                        },
                            BreadcrumbItem { href: "/", "Home" }
                            BreadcrumbItem { href: "/docs", "Docs" }
                            BreadcrumbItem { is_current: true, "API Reference" }
                        }
                    }
                }
                h3 { "With icons" }
                Breadcrumbs { label: "Breadcrumbs with icons",
                    BreadcrumbItem { href: "/", start_icon: rsx! { TrailIcon { is_home: true } }, "Home" }
                    BreadcrumbItem { href: "/settings", start_icon: rsx! { TrailIcon { is_home: false } }, "Settings" }
                    BreadcrumbItem { is_current: true, "Profile" }
                }
                h3 { "Deep hierarchy" }
                Breadcrumbs { id: "breadcrumbs-deep", label: "Product breadcrumb",
                    BreadcrumbItem { href: "/", start_icon: rsx! { TrailIcon { is_home: true } }, "Home" }
                    BreadcrumbItem { href: "/products", "Products" }
                    BreadcrumbItem { href: "/products/electronics", "Electronics" }
                    BreadcrumbItem { href: "/products/electronics/phones", "Phones" }
                    BreadcrumbItem { is_current: true, "iPhone 15 Pro" }
                }
                h3 { "Change trail styling" }
                button { id: "breadcrumbs-toggle", onclick: move |_| is_supporting.toggle(), "Change variant and separator" }
                Breadcrumbs { id: "breadcrumbs-reactive", label: "Reactive breadcrumb",
                    variant: if is_supporting() { BreadcrumbsVariant::SUPPORTING } else { BreadcrumbsVariant::DEFAULT },
                    separator: is_supporting().then(|| rsx! { "·" }),
                    BreadcrumbItem { href: "#breadcrumbs-examples", "Home" }
                    BreadcrumbItem { href: "#breadcrumbs-reactive", "Details" }
                }
            }
        }
    }
}
