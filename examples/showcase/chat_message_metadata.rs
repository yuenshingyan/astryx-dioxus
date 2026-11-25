use dioxus::prelude::*;

use astryx_dioxus::{
    Button, ButtonSize, ButtonVariant, ChatMessageMetadata, ChatMessageStatus, Timestamp,
    TimestampFormat,
};

#[component]
fn FeedbackFooter() -> Element {
    let mut selected = use_signal(|| None::<&'static str>);
    rsx! {
        span { style: "display:inline-flex;align-items:center;flex-wrap:wrap;gap:4px",
            for label in ["Copy", "Retry", "Good response", "Bad response"] {
                Button { key: "{label}", label, size: ButtonSize::Small, variant: ButtonVariant::Ghost,
                    onclick: move |_| selected.set(Some(label)),
                }
            }
            "GPT-4o"
            if let Some(label) = selected() { span { "Demo: {label}" } }
        }
    }
}

#[component]
pub(super) fn ChatMessageMetadataExamples() -> Element {
    rsx! {
        section { id: "chat-message-metadata-examples",
            h2 { "ChatMessageMetadata" }
            p { "Absolute timestamps use UTC. The relative caption is a fixed example." }
            div { style: "max-width:500px",
                h3 { "Showcase" }
                div { "Sorry, something went wrong on my end." }
                ChatMessageMetadata { id: "chat-metadata-error", timestamp: rsx! { Timestamp { value: 1_773_585_000_i64, format: TimestampFormat::SystemTime } },
                    status: ChatMessageStatus::Error, footer: rsx! { Button { label: "Retry", size: ButtonSize::Small, variant: ButtonVariant::Ghost } },
                }
                div { style: "text-align:end", "No worries — try again with just the last 24 hours of logs." }
                ChatMessageMetadata { id: "chat-metadata-read", is_user: true, timestamp: rsx! { Timestamp { value: 1_773_585_060_i64, format: TimestampFormat::SystemTime } }, status: ChatMessageStatus::Read }
                div { "Here’s a summary of the last 24 hours." }
                ChatMessageMetadata { timestamp: rsx! { Timestamp { value: 1_773_585_120_i64, format: TimestampFormat::SystemTime } }, footer: rsx! { FeedbackFooter {} } }
                h3 { "Timestamp" }
                div { style: "text-align:end", "Thanks — any blockers I should know about?" }
                ChatMessageMetadata { is_user: true, timestamp: rsx! { Timestamp { value: 1_777_473_000_i64, format: TimestampFormat::SystemTime } } }
                div { "Relative timestamps work too — helpful for older messages where the exact time matters less than recency." }
                ChatMessageMetadata { timestamp: rsx! { time { datetime: "2026-04-28T16:45:00Z", "Yesterday" } } }
                h3 { "Status" }
                for (status, message) in [
                    (ChatMessageStatus::Sending, "Sending…"), (ChatMessageStatus::Sent, "Sent"), (ChatMessageStatus::Delivered, "Delivered"),
                    (ChatMessageStatus::Read, "Read"), (ChatMessageStatus::Error, "Failed"),
                ] {
                    div { key: "{message}",
                        div { style: "text-align:end", "Message — {message}" }
                        ChatMessageMetadata { is_user: true, timestamp: rsx! { Timestamp { value: 1_777_457_700_i64, format: TimestampFormat::SystemTime } }, status }
                    }
                }
                h3 { "Footer" }
                div { style: "text-align:end", "Summarize the Q1 revenue report." }
                ChatMessageMetadata { is_user: true, timestamp: rsx! { Timestamp { value: 1_777_455_660_i64, format: TimestampFormat::SystemTime } }, status: ChatMessageStatus::Read }
                div { "Here’s the Q1 summary." }
                ChatMessageMetadata { timestamp: rsx! { Timestamp { value: 1_777_455_720_i64, format: TimestampFormat::SystemTime } }, footer: rsx! { FeedbackFooter {} } }
            }
        }
    }
}
