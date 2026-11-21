use dioxus::prelude::*;

use astryx_dioxus::{ChatSendButton, ChatSendButtonSize, Icon};

const SPARK: &str = "m12 3 2.5 6.5L21 12l-6.5 2.5L12 21l-2.5-6.5L3 12l6.5-2.5z";

fn glyph(path_data: &'static str) -> Element {
    rsx! { Icon { icon: rsx! {
        svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            path { d: path_data }
        }
    } } }
}

#[component]
pub(super) fn ChatSendButtonExamples() -> Element {
    let mut message = use_signal(|| "Hello, how can you help?".to_owned());
    let mut is_sending = use_signal(|| false);
    let mut result = use_signal(|| "Ready");
    rsx! {
        section { id: "chat-send-button-examples",
            h2 { "ChatSendButton" }
            h3 { "Showcase" }
            div { style: "display:flex;gap:8px",
                ChatSendButton { is_disabled: false, on_send: |_| {} }
                ChatSendButton { is_disabled: false, send_icon: glyph(SPARK), on_send: |_| {} }
                ChatSendButton { is_stop_shown: true, on_stop: |_| {} }
            }
            h3 { "States" }
            div { id: "chat-send-states", style: "display:flex;gap:8px",
                ChatSendButton { on_send: |_| {} }
                ChatSendButton { is_disabled: false, size: ChatSendButtonSize::Small, on_send: |_| {} }
                ChatSendButton { is_stop_shown: true, on_stop: |_| {} }
            }
            h3 { "Custom icons" }
            div { style: "display:flex;gap:8px",
                ChatSendButton { is_disabled: false, send_icon: glyph("m3 3 18 9-18 9 4-9zm4 9h14"), on_send: |_| {} }
                ChatSendButton { is_disabled: false, send_icon: glyph("m5 12 4 4L19 6"), on_send: |_| {} }
                ChatSendButton { is_disabled: false, send_icon: glyph(SPARK), on_send: |_| {} }
                ChatSendButton { is_stop_shown: true, stop_icon: glyph("M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0ZM9 9l6 6m0-6-6 6"), on_stop: |_| {} }
            }
            h3 { "In composer" }
            p { "Native text input stands in for ChatComposer." }
            div { style: "display:flex;gap:8px;max-width:450px;align-items:center",
                input { "aria-label": "Message", value: "{message}", oninput: move |event| message.set(event.value()) }
                ChatSendButton { id: "chat-send-action", is_stop_shown: is_sending(), is_disabled: message.read().trim().is_empty(),
                    on_send: move |_| { message.set(String::new()); is_sending.set(true); result.set("Sent"); },
                    on_stop: move |_| { is_sending.set(false); result.set("Stopped"); },
                    onclick: move |_| result.set(if result() == "Sent" { "Send action ran first" } else { "Stop action ran first" }),
                }
            }
            p { id: "chat-send-result", "{result}" }
        }
    }
}
