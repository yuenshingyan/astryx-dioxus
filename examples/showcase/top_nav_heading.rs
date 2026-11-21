use dioxus::prelude::*;

use astryx_dioxus::{Icon, NavIcon, TopNavHeading};

#[component]
fn AppIcon() -> Element {
    rsx! {
        svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", "aria-hidden": "true",
            for (x, y) in [(4, 4), (14, 4), (4, 14), (14, 14)] {
                rect { key: "{x}-{y}", x: "{x}", y: "{y}", width: "6", height: "6", rx: "2" }
            }
        }
    }
}

#[component]
pub(super) fn TopNavHeadingExamples() -> Element {
    rsx! {
        section { id: "top-nav-heading-examples", "data-astryx-theme": "neutral",
            h2 { "TopNavHeading" }
            h3 { "Basic" }
            nav { "aria-label": "Product navigation",
                TopNavHeading { heading: "Acme Platform", heading_href: "/", logo: rsx! {
                    NavIcon { icon: rsx! { Icon { icon: rsx! {
                        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
                            path { d: "M9 4.5v15m6-15v15m-10.875 0h15.75c.621 0 1.125-.504 1.125-1.125V5.625c0-.621-.504-1.125-1.125-1.125H4.125C3.504 4.5 3 5.004 3 5.625v12.75c0 .621.504 1.125 1.125 1.125z" }
                        }
                    } } } }
                } }
            }
            h3 { "Showcase" }
            div { style: "display:grid;gap:var(--spacing-4)",
                nav { "aria-label": "Plain heading example",
                    TopNavHeading { heading: "Acme Platform", logo: rsx! { NavIcon { icon: rsx! { AppIcon {} } } } }
                }
                nav { "aria-label": "Linked heading example",
                    TopNavHeading { heading: "Acme Platform", heading_href: "/", logo: rsx! { AppIcon {} } }
                }
            }
            h3 { "Logo link" }
            TopNavHeading { id: "top-nav-logo-only", logo: rsx! { AppIcon {} }, logo_label: "Acme home", heading_href: "#top-nav-heading-examples", style: "width:max-content" }
            h3 { "Narrow heading" }
            TopNavHeading { id: "top-nav-narrow", style: "width:160px", dir: "rtl",
                heading: "Acme Platform product and account", heading_href: "#top-nav-heading-examples",
                logo: rsx! { AppIcon {} }, header_end_content: rsx! { button { r#type: "button", "Settings" } },
            }
            TopNavHeading { id: "top-nav-narrow-links", style: "width:160px",
                heading: "Acme Platform product account", heading_href: "#top-nav-heading-examples", superheading: "Choose a workspace", superheading_href: "#top-nav-heading-examples",
                logo: rsx! { AppIcon {} },
            }
        }
    }
}
