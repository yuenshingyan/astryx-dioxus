use dioxus::prelude::*;

use astryx_dioxus::{
    ButtonSize, SideNavCollapseButton, SideNavControlledCollapsible, SideNavHeading, SideNavSection,
};

#[component]
fn Rail(#[props(default)] has_heading: bool) -> Element {
    let mut collapsed = use_signal(|| false);
    let mut cancel = use_signal(|| false);
    let config = SideNavControlledCollapsible {
        is_collapsed: collapsed(),
        on_collapsed_change: EventHandler::new(move |next| collapsed.set(next)),
    };
    let links = rsx! {
        for (glyph, label) in [("⌂", "Home"), ("▤", "Projects")] {
            a { key: "{label}", href: "#side-nav-collapse-button-examples", "aria-label": label,
                span { "aria-hidden": "true", "{glyph}" }
                if !collapsed() { " {label}" }
            }
        }
    };
    let width = if collapsed() { 52 } else { 200 };
    rsx! {
        nav { "aria-label": if has_heading { "Workspace navigation" } else { "Basic navigation" }, style: "width:{width}px;display:grid;gap:4px;padding:8px;border:1px solid var(--color-border)",
            if has_heading {
                if !collapsed() { SideNavHeading { heading: "Workspace" } }
                SideNavSection { title: "Main", is_header_hidden: collapsed(), {links} }
            } else { {links} }
            div { dir: if has_heading { "rtl" } else { "ltr" },
                SideNavCollapseButton { collapsible: config, size: ButtonSize::Small,
                    id: if has_heading { "collapse-workspace" } else { "collapse-basic" },
                    onclick: move |event: MouseEvent| { if cancel() { event.prevent_default(); } },
                }
            }
        }
        if has_heading {
            button { id: "collapse-cancel", r#type: "button", onclick: move |_| cancel.toggle(), "Cancel next toggle while enabled" }
        }
    }
}

#[component]
pub(super) fn SideNavCollapseButtonExamples() -> Element {
    rsx! {
        section { id: "side-nav-collapse-button-examples", "data-astryx-theme": "neutral",
            h2 { "SideNavCollapseButton" }
            h3 { "Basic" }
            Rail {}
            h3 { "Showcase" }
            Rail { has_heading: true }
        }
    }
}
