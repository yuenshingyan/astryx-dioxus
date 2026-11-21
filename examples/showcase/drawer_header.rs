use dioxus::prelude::*;

use astryx_dioxus::{
    Badge, BadgeVariant, Button, DrawerHeader, Layout, LayoutContent, LayoutHeight,
};

#[component]
fn HeaderPreview(is_host: bool) -> Element {
    let mut is_open = use_signal(|| false);
    let (title, trigger) = if is_host {
        ("web-prod-04", "Open host details")
    } else {
        ("Details", "Open drawer")
    };
    rsx! {
        Button { label: trigger, onclick: move |_| is_open.set(true) }
        if is_open() {
            Layout { height: LayoutHeight::Auto, style: "max-width:420px",
                header: rsx! {
                    DrawerHeader { title: rsx! { "{title}" }, on_open_change: move |next| is_open.set(next),
                        subtitle: is_host.then(|| rsx! { "us-east-1" }),
                        end_content: is_host.then(|| rsx! { Badge { label: "Healthy", variant: BadgeVariant::SUCCESS } }),
                    }
                },
                content: rsx! { LayoutContent { "Drawer body content goes here." } },
            }
        }
    }
}

#[component]
pub(super) fn DrawerHeaderExamples() -> Element {
    rsx! {
        section { id: "drawer-header-examples", "data-astryx-theme": "neutral",
            h2 { "DrawerHeader" }
            p { "Inline previews; the Drawer container is not yet ported." }
            h3 { "Showcase" }
            Layout { height: LayoutHeight::Auto, style: "max-width:420px",
                header: rsx! {
                    DrawerHeader { title: rsx! { "Host details" }, subtitle: rsx! { "web-prod-04" },
                        has_divider: true, id: "drawer-header-preview", dir: "rtl", on_open_change: |_| {},
                    }
                },
                content: rsx! { LayoutContent { "Drawer body content goes here." } },
            }
            h3 { "With close button" }
            HeaderPreview { is_host: false }
            h3 { "Subtitle and end content" }
            HeaderPreview { is_host: true }
        }
    }
}
