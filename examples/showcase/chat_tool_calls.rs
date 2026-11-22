use dioxus::prelude::*;

use astryx_dioxus::{ChatToolCallItem, ChatToolCallStatus, ChatToolCalls};

#[component]
pub(super) fn ChatToolCallsExamples() -> Element {
    let read = ChatToolCallItem {
        key: "read".into(),
        name: "read".into(),
        target: Some("src/utils/formatDate.ts".into()),
        duration: Some("0.2s".into()),
        result_detail: Some(rsx! { pre { "Date formatter source" } }),
        ..Default::default()
    };
    let edit = ChatToolCallItem {
        key: "edit".into(),
        name: "edit".into(),
        target: Some("src/utils/formatDate.ts".into()),
        status: ChatToolCallStatus::Running,
        additions: Some(0),
        ..Default::default()
    };
    rsx! {
        section { id: "chat-tool-calls-examples",
            h2 { "ChatToolCalls" }
            div { id: "single-tool-call", ChatToolCalls { calls: vec![read.clone()] } }
            div { id: "grouped-tool-calls", ChatToolCalls { calls: vec![read, edit] } }
        }
    }
}
