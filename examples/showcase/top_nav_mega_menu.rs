use dioxus::prelude::*;

use astryx_dioxus::{
    TopNav, TopNavHeading, TopNavItem, TopNavMegaMenu, TopNavMegaMenuFeaturedCard,
    TopNavMegaMenuItem,
};

#[component]
pub(super) fn TopNavMegaMenuExamples() -> Element {
    rsx! {
        section { id: "top-nav-mega-menu-examples",
            h2 { "Top Nav Mega Menu" }
            div { style: "max-inline-size:600px",
                TopNav { label: "Mega menu demo", heading: rsx! { TopNavHeading { heading: "DevTools" } },
                    start_content: rsx! {
                        TopNavItem { label: "Overview", href: "#top-nav-mega-menu-examples", is_selected: true }
                        TopNavMegaMenu { label: "Products",
                            items: rsx! {
                                for (title, description, href) in [
                                    ("Deploy", "Ship to production in seconds", "#mega-deploy"),
                                    ("Documentation", "Guides, references, and tutorials", "#mega-docs"),
                                    ("API", "Programmatic access to all features", "#mega-api"),
                                    ("Security", "Enterprise-grade protection", "#mega-security"),
                                ] {
                                    TopNavMegaMenuItem { key: "{title}", title, description, href }
                                }
                            },
                            featured: rsx! {
                                TopNavMegaMenuFeaturedCard {
                                    title: "What's New", description: "Check out our latest features and improvements in the Q2 release.",
                                    link_label: "Read the changelog", link_href: "#mega-changelog",
                                }
                            },
                        }
                    },
                }
            }
            p { id: "mega-deploy", "Deploy" }
            p { id: "mega-docs", "Documentation" }
            p { id: "mega-api", "API" }
            p { id: "mega-security", "Security" }
            p { id: "mega-changelog", "Changelog" }
        }
    }
}
