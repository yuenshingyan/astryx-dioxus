use dioxus::prelude::*;

use astryx_dioxus::ChatLayoutScrollButton;

#[component]
pub(super) fn ChatLayoutScrollButtonExamples() -> Element {
    let mut is_visible = use_signal(|| true);
    rsx! {
        section { id: "chat-layout-scroll-button-examples",
            h2 { "ChatLayoutScrollButton" }
            h3 { "States" }
            div { "Hidden" ChatLayoutScrollButton { is_visible: false, onclick: |_| {} } }
            div { "Collapsed" ChatLayoutScrollButton { is_visible: true, onclick: |_| {} } }
            div { "Expanded" ChatLayoutScrollButton { is_visible: true, label: "New messages", onclick: |_| {} } }
            h3 { "Labels" }
            div { style: "display:flex;gap:12px;flex-wrap:wrap",
                ChatLayoutScrollButton { is_visible: true, onclick: |_| {} }
                ChatLayoutScrollButton { is_visible: true, label: "New messages", onclick: |_| {} }
                ChatLayoutScrollButton { is_visible: true, label: "3 unread replies", onclick: |_| {} }
            }
            h3 { "Visibility control" }
            p { "This standalone example hides the action on activation; it does not control a transcript." }
            button { id: "chat-scroll-show", r#type: "button", onclick: move |_| is_visible.set(true), "Show action" }
            ChatLayoutScrollButton { id: "chat-scroll-probe", is_visible: is_visible(), onclick: move |_| is_visible.set(false) }
        }
    }
}
