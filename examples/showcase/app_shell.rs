use dioxus::prelude::*;

use astryx_dioxus::{
    AppShell, AppShellVariant, SideNav, SideNavItem, SpacingStep, TopNav, TopNavHeading,
};

#[component]
pub(super) fn AppShellExamples() -> Element {
    rsx! {
        section { id: "app-shell-examples",
            h2 { "AppShell" }
            AppShell { variant: AppShellVariant::Section, content_padding: SpacingStep::Four,
                style: "block-size:320px",
                top_nav: rsx! { TopNav { heading: rsx! { TopNavHeading { heading: "Workspace" } } } },
                side_nav: rsx! {
                    SideNav {
                        SideNavItem { label: "Dashboard", href: "#app-shell-examples", is_selected: true }
                        SideNavItem { label: "Projects", href: "#app-shell-examples" }
                    }
                },
                h3 { "Workspace overview" }
                for index in 1..=16 { p { key: "{index}", "Project {index}" } }
            }
        }
    }
}
