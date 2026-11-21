use dioxus::prelude::*;

use astryx_dioxus::{Button, DialogHeader, Layout, LayoutContent, LayoutHeight};

#[component]
fn Preview(title: &'static str, subtitle: &'static str) -> Element {
    rsx! {
        Layout { height: LayoutHeight::Auto, style: "max-width:420px",
            header: rsx! { DialogHeader { title: rsx! { "{title}" }, subtitle: rsx! { "{subtitle}" },
                is_inline: true, on_open_change: |_| {},
            } },
            content: rsx! { LayoutContent { "Dialog body content goes here." } },
        }
    }
}

#[component]
pub(super) fn DialogHeaderExamples() -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        section { id: "dialog-header-examples", "data-astryx-theme": "neutral",
            h2 { "DialogHeader" }
            p { "Inline previews; the Dialog container is not yet ported." }
            h3 { "Basic" }
            Preview { title: "Invite teammates", subtitle: "Send invitations to join your workspace" }
            h3 { "Showcase" }
            Preview { title: "Edit Profile", subtitle: "Update your personal information" }
            h3 { "Title focus" }
            Button { id: "dialog-header-open", label: "Mount header", onclick: move |_| is_open.set(true) }
            if is_open() {
                div { role: "dialog", "aria-labelledby": "dialog-header-test-title",
                    DialogHeader { title: rsx! { "Details" }, title_id: "dialog-header-test-title", on_open_change: move |next| is_open.set(next) }
                }
            }
        }
    }
}
