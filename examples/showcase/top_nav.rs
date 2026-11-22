use dioxus::prelude::*;

use astryx_dioxus::{Button, ButtonVariant, TopNav, TopNavHeading, TopNavItem};

#[component]
pub(super) fn TopNavExamples() -> Element {
    rsx! {
        section { id: "top-nav-examples",
            h2 { "TopNav" }
            TopNav { id: "main-navigation-preview", label: "Main navigation",
                heading: rsx! { TopNavHeading { heading: "My App" } },
                start_content: rsx! {
                    TopNavItem { label: "Home", href: "#top-nav-examples", is_selected: true }
                    TopNavItem { label: "Products", href: "#top-nav-examples" }
                    TopNavItem { label: "About", href: "#top-nav-examples" }
                },
                end_content: rsx! {
                    Button { label: "Search", variant: ButtonVariant::Ghost }
                    Button { label: "Notifications", variant: ButtonVariant::Ghost }
                    Button { label: "Profile", variant: ButtonVariant::Ghost }
                },
            }
            TopNav { id: "balanced-navigation-preview", label: "Project navigation", dir: "rtl",
                heading: rsx! { TopNavHeading { heading: "Project" } },
                center_content: rsx! { TopNavItem { label: "Overview", href: "#top-nav-examples" } },
                end_content: rsx! { Button { label: "Settings", variant: ButtonVariant::Ghost } },
            }
        }
    }
}
