use dioxus::prelude::*;

use astryx_dioxus::SideNavHeading;

#[component]
fn AppIcon() -> Element {
    rsx! {
        svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linejoin: "round", "aria-hidden": "true",
            for (x, y) in [(4, 4), (14, 4), (4, 14), (14, 14)] {
                rect { key: "{x}-{y}", x: "{x}", y: "{y}", width: "6", height: "6", rx: "2" }
            }
        }
    }
}

#[component]
pub(super) fn SideNavHeadingExamples() -> Element {
    rsx! {
        section { id: "side-nav-heading-examples", "data-astryx-theme": "neutral",
            h2 { "SideNavHeading" }
            h3 { "Basic" }
            nav { "aria-label": "Analytics navigation", style: "max-width:240px",
                SideNavHeading { id: "side-nav-heading-basic", heading: "Analytics", heading_href: "/", icon: rsx! { AppIcon {} } }
            }
            h3 { "Showcase" }
            div { style: "display:flex;gap:var(--spacing-6);flex-wrap:wrap",
                nav { "aria-label": "Side navigation with a heading",
                    SideNavHeading { heading: "Analytics", heading_href: "/", icon: rsx! { AppIcon {} } }
                }
                nav { "aria-label": "Side navigation with a heading, superheading and subheading",
                    SideNavHeading { heading: "Analytics", heading_href: "/", icon: rsx! { AppIcon {} }, superheading: "Acme Corp", subheading: "Production" }
                }
            }
            h3 { "Trailing action" }
            SideNavHeading { id: "side-nav-heading-trailing", heading: "Analytics", heading_href: "#side-nav-heading-examples",
                header_end_content: rsx! { button { r#type: "button", "Settings" } },
            }
        }
    }
}
