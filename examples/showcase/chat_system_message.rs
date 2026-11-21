use dioxus::prelude::*;

use astryx_dioxus::{ChatSystemMessage, ChatSystemMessageVariant, Icon};

#[component]
fn DateMarker(children: Element) -> Element {
    rsx! { ChatSystemMessage { variant: ChatSystemMessageVariant::Divider, {children} } }
}

#[component]
fn Notice(glyph: &'static str, children: Element) -> Element {
    rsx! {
        ChatSystemMessage {
            icon: rsx! { Icon { icon: rsx! {
                svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
                    path { d: glyph }
                }
            } } },
            {children}
        }
    }
}

const JOIN: &str =
    "M3 20v-2a4 4 0 0 1 4-4h4a4 4 0 0 1 4 4v2M13 7a4 4 0 1 1-8 0 4 4 0 0 1 8 0M19 8v6m-3-3h6";
const LEAVE: &str =
    "M3 20v-2a4 4 0 0 1 4-4h4a4 4 0 0 1 4 4v2M13 7a4 4 0 1 1-8 0 4 4 0 0 1 8 0m3 4h6";
const LOCK: &str = "M6 10h12v10H6zm2 0V7a4 4 0 0 1 8 0v3";
const SPARK: &str = "m12 3 2.5 6.5L21 12l-6.5 2.5L12 21l-2.5-6.5L3 12l6.5-2.5z";
const SHIELD: &str = "m12 3 7 3v6c0 4-4 7-7 9-3-2-7-5-7-9V6zm-3 9 2 2 4-4";
const CHECK: &str = "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0ZM8 12l3 3 5-6";

#[component]
pub(super) fn ChatSystemMessageExamples() -> Element {
    rsx! {
        section { id: "chat-system-message-examples",
            h2 { "ChatSystemMessage" }
            p { "Static transcript previews; ChatMessageList is not yet ported." }
            div { style: "max-width:480px",
                h3 { "Showcase" }
                DateMarker { "March 15, 2026" }
                ChatSystemMessage { "Alex joined the conversation" }
                ChatSystemMessage { "Agent is thinking…" }
                DateMarker { "Today" }
                ChatSystemMessage { "Conversation marked as resolved" }
                h3 { "Variants" }
                ChatSystemMessage { "Alex joined the conversation" }
                ChatSystemMessage { "Conversation marked as resolved" }
                DateMarker { "March 15, 2026" }
                DateMarker { "Today" }
                h3 { "With icon" }
                Notice { glyph: JOIN, "Jordan was added to the conversation" }
                Notice { glyph: LOCK, "Messages are end-to-end encrypted" }
                Notice { glyph: SPARK, "Agent is generating a response…" }
                Notice { glyph: SHIELD, "Conversation verified by admin" }
                h3 { "Status updates" }
                DateMarker { "March 14, 2026" }
                Notice { glyph: JOIN, "Sarah Chen joined the conversation" }
                ChatSystemMessage { "Topic changed to \"Q2 Launch Planning\"" }
                DateMarker { "March 15, 2026" }
                Notice { glyph: LEAVE, "Alex Rivera left the conversation" }
                DateMarker { "Today" }
                Notice { glyph: CHECK, "Conversation marked as resolved" }
            }
        }
    }
}
