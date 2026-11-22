use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{Badge, Spinner, SpinnerShade, SpinnerSize, VisuallyHidden};

/// Display state of a tool call; this does not execute or schedule any work.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ChatToolCallStatus {
    Pending,
    Running,
    #[default]
    Complete,
    Error,
}

impl ChatToolCallStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Complete => "complete",
            Self::Error => "error",
        }
    }
}

/// Caller-supplied display data. Use a stable, unique key for each call.
#[derive(Clone, Default, PartialEq)]
pub struct ChatToolCallItem {
    pub key: String,
    pub name: String,
    pub status: ChatToolCallStatus,
    pub target: Option<String>,
    pub duration: Option<String>,
    pub node: Option<String>,
    pub additions: Option<u32>,
    pub deletions: Option<u32>,
    pub stats: Option<Element>,
    pub error_message: Option<String>,
    pub result_detail: Option<Element>,
}

/// Shows one tool call inline or multiple calls behind a disclosure.
#[component]
pub fn ChatToolCalls(
    calls: Vec<ChatToolCallItem>,
    #[props(default)] label: Option<String>,
    #[props(default)] is_expanded: Option<bool>,
    #[props(default)] default_is_expanded: bool,
    #[props(default)] on_expanded_change: Option<EventHandler<bool>>,
    #[props(default)] class: String,
    #[props(default)] style: String,
) -> Element {
    let mut internal = use_signal(|| default_is_expanded);
    let id = use_hook(|| format!("astryx-tool-calls-{}", current_scope_id().0));
    let Some(latest) = calls.last().cloned() else {
        return VNode::empty();
    };
    if calls.len() == 1 {
        return rsx! { div { class: "astryx-chat-tool-calls {class}", style, CallRow { key: "{latest.key}", call: latest } } };
    }
    let count = calls.len();
    let summary = label.unwrap_or_else(|| format!("{count} tool calls"));
    let is_open = is_expanded.unwrap_or_else(|| *internal.read());
    rsx! {
        div { class: "astryx-chat-tool-calls {class}", style,
            button { class: "astryx-chat-tool-calls__row", r#type: "button", "aria-expanded": is_open.to_string(), "aria-controls": id.clone(),
                onclick: move |_| {
                    if is_expanded.is_none() { internal.set(!is_open); }
                    if let Some(handler) = on_expanded_change { handler.call(!is_open); }
                },
                if is_open { span { "{summary}" } }
                else {
                    StatusMark { status: latest.status, error_message: latest.error_message }
                    span { class: "astryx-chat-tool-calls__name", "{latest.name}" }
                    if let Some(target) = latest.target { span { class: "astryx-chat-tool-calls__target", "{target}" } }
                    span { class: "astryx-chat-tool-calls__count", "{summary}" }
                }
                Chevron {}
            }
            div { id, hidden: !is_open,
                for call in calls { CallRow { key: "{call.key}", call } }
            }
        }
    }
}

#[component]
fn CallRow(call: ChatToolCallItem) -> Element {
    let mut is_open = use_signal(|| false);
    let id = use_hook(|| format!("astryx-tool-detail-{}", current_scope_id().0));
    let content = rsx! {
        StatusMark { status: call.status, error_message: call.error_message }
        span { class: "astryx-chat-tool-calls__name", "{call.name}" }
        if let Some(node) = call.node { Badge { label: node } }
        if let Some(target) = call.target { span { class: "astryx-chat-tool-calls__target", "{target}" } }
        if let Some(additions) = call.additions { span { class: "astryx-chat-tool-calls__added", "+{additions}" } }
        if let Some(deletions) = call.deletions { span { class: "astryx-chat-tool-calls__removed", "-{deletions}" } }
        {call.stats}
        if call.status == ChatToolCallStatus::Complete && let Some(duration) = call.duration { span { "{duration}" } }
    };
    let Some(detail) = call.result_detail else {
        return rsx! { div { class: "astryx-chat-tool-calls__row", {content} } };
    };
    let is_expanded = is_open();
    let controls = is_expanded.then(|| id.clone());
    rsx! {
        div {
            button { class: "astryx-chat-tool-calls__row", r#type: "button", "aria-expanded": is_expanded.to_string(),
                "aria-controls": controls, onclick: move |_| is_open.set(!is_expanded),
                {content}
                Chevron {}
            }
            if is_expanded { div { class: "astryx-chat-tool-calls__detail", id, {detail} } }
        }
    }
}

#[component]
fn Chevron() -> Element {
    rsx! { svg { class: "astryx-chat-tool-calls__chevron", "aria-hidden": "true", view_box: "0 0 16 16", fill: "none", stroke: "currentColor", path { d: "m4 6 4 4 4-4" } } }
}

#[component]
fn StatusMark(status: ChatToolCallStatus, error_message: Option<String>) -> Element {
    let (name, symbol) = match status {
        ChatToolCallStatus::Pending => ("Pending", None),
        ChatToolCallStatus::Running => ("Running", None),
        ChatToolCallStatus::Complete => ("Complete", Some("✓")),
        ChatToolCallStatus::Error => ("Error", Some("!")),
    };
    let error_message = error_message.filter(|_| status == ChatToolCallStatus::Error);
    let announcement = error_message
        .as_ref()
        .map_or_else(|| name.to_owned(), |text| format!("{name}: {text}"));
    rsx! {
        span { class: "astryx-chat-tool-calls__status", "data-status": status.as_str(), title: error_message,
            span { "aria-hidden": "true",
                if let Some(symbol) = symbol { "{symbol}" }
                else { Spinner { size: SpinnerSize::Small, shade: SpinnerShade::Subtle } }
            }
            VisuallyHidden { "{announcement}" }
        }
    }
}
