use dioxus::prelude::*;

use astryx_dioxus::TopNavHeading;

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
                    span { class: "astryx-nav-icon", style: "display:grid;place-items:center;width:32px;height:32px",
                        svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", "aria-hidden": "true",
                            rect { x: "3", y: "5", width: "18", height: "14", rx: "2" }
                            path { d: "M9 5v14m6-14v14" }
                        }
                    }
                } }
            }
            h3 { "Showcase" }
            div { style: "display:grid;gap:var(--spacing-4)",
                nav { "aria-label": "Plain heading example",
                    TopNavHeading { heading: "Acme Platform", logo: rsx! { span { class: "astryx-nav-icon", style: "display:grid;place-items:center;width:32px;height:32px", AppIcon {} } } }
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
