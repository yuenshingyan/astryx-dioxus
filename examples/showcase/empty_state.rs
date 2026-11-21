use dioxus::prelude::*;

use astryx_dioxus::{
    Button, ButtonSize, ButtonVariant, Card, EmptyState, HStack, HeadingLevel, Icon, IconSize,
    SpacingStep,
};

#[component]
fn Illustration(path: &'static str) -> Element {
    rsx! { Icon { size: IconSize::Large, icon: rsx! {
        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round", path { d: path } }
    } } }
}

#[component]
pub(super) fn EmptyStateExamples() -> Element {
    const SEARCH: &str = "M21 21l-5-5m2-5a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z";
    const INBOX: &str = "M3 14l3-8h12l3 8v5H3Zm0 0h5l2 3h4l2-3h5";
    const FOLDER: &str = "M3 7v12h18V8h-9l-2-3H3Zm9 3v6m-3-3h6";
    let mut message = use_signal(|| "");
    let mut phase = use_signal(|| 0);
    rsx! {
        section { id: "empty-state-examples", "data-astryx-theme": "neutral",
            h2 { "EmptyState" }
            h3 { "Showcase" }
            EmptyState {
                title: "No results found", description: "Try adjusting your search or filters to find what you need.",
                icon: rsx! { Illustration { path: SEARCH } },
                actions: rsx! { Button { label: "Clear filters", variant: ButtonVariant::Secondary, onclick: move |_| message.set("Demo: your filters would be cleared here") } },
            }
            h3 { "Actions" }
            EmptyState {
                title: "No results found", description: "Try adjusting your search terms or clearing filters to see more results.",
                icon: rsx! { Illustration { path: SEARCH } },
                actions: rsx! {
                    Button { label: "Go back", variant: ButtonVariant::Secondary, onclick: move |_| message.set("Demo: you would go back to the previous page here") }
                    Button { label: "Clear filters", variant: ButtonVariant::Primary, onclick: move |_| message.set("Demo: your filters would be cleared here") }
                },
            }
            h3 { "Compact" }
            EmptyState { id: "empty-state-compact", is_compact: true,
                title: "No notifications", description: "You're all caught up. New notifications will appear here.",
                icon: rsx! { Illustration { path: INBOX } },
                actions: rsx! { HStack { gap: SpacingStep::Two,
                    Button { label: "Settings", variant: ButtonVariant::Secondary, size: ButtonSize::Small, onclick: move |_| message.set("Demo: notification settings would open here") }
                    Button { label: "Refresh", variant: ButtonVariant::Primary, size: ButtonSize::Small, onclick: move |_| message.set("Demo: notifications would be checked again here") }
                } },
            }
            h3 { "Inside a container" }
            Card { EmptyState {
                title: "No projects yet", description: "Create your first project to start organizing your work. You can invite team members after.",
                icon: rsx! { Illustration { path: FOLDER } },
                actions: rsx! {
                    Button { label: "Import", variant: ButtonVariant::Secondary, onclick: move |_| message.set("Demo: a project import would start here") }
                    Button { label: "Create project", variant: ButtonVariant::Primary, onclick: move |_| message.set("Demo: a new project form would open here") }
                },
            } }
            output { id: "empty-state-action", "{message}" }
            h3 { "Dynamic announcement" }
            button { id: "empty-state-toggle", onclick: move |_| phase.set((phase() + 1) % 4), "Change search results" }
            if phase() != 3 {
                EmptyState { id: "empty-state-dynamic", heading_level: HeadingLevel::Four,
                    title: if phase() == 0 { "Nothing to review" } else { "No matching results" },
                    description: if phase() < 2 { "Try another query." } else { "Try fewer filters." },
                    actions: rsx! { Button { label: "Search again" } },
                }
            }
        }
    }
}
