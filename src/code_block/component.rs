use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::utils::is_renderable::is_renderable;
use crate::{Button, ButtonSize, ButtonVariant, SizeValue, VisuallyHidden};

use super::tokenizer::{DiffLineKind, classify_diff_line};

enum CopyFeedback {
    Idle,
    Pending,
    Succeeded(String),
    Failed(String),
}

/// Source display with optional unified diff, line gutter, and exact-source copy.
#[component]
pub fn CodeBlock(
    code: String,
    #[props(default)] title: Option<String>,
    #[props(default)] has_line_numbers: bool,
    #[props(default = 1)] start_line_number: u32,
    #[props(default)] highlight_lines: Vec<u32>,
    /// Present unified diff markers in a separate column without changing the source.
    #[props(default)]
    is_diff: bool,
    /// Allow disclosure when a title or diff label is visible.
    #[props(default)]
    is_collapsible: bool,
    /// Minimum number of source lines required for disclosure.
    #[props(default = 10)]
    collapsible_threshold: usize,
    /// Accessible name when the disclosure has no visible label.
    #[props(default = "Code".to_owned())]
    disclosure_label: String,
    #[props(default)] is_wrapped: bool,
    #[props(default)] max_height: Option<SizeValue>,
    /// Preferred inline size. When omitted, size to the source with a 400px
    /// minimum that shrinks to fit the containing block.
    #[props(default)]
    width: Option<SizeValue>,
    #[props(default)] has_copy_button: bool,
    /// Caller-owned actions shown before Copy, outside the disclosure control.
    #[props(default)]
    end_content: Option<Element>,
    #[props(default = "Copy code".to_owned())] copy_label: String,
    #[props(default = "Copied code".to_owned())] copied_message: String,
    #[props(default = "Could not copy code".to_owned())] copy_error_message: String,
) -> Element {
    let mut copy_feedback = use_signal(|| CopyFeedback::Idle);
    let style = match max_height {
        Some(value) => format!("max-block-size:{}", value.as_css()?),
        None => String::new(),
    };
    let width = width.map(|value| value.as_css()).transpose()?;
    let has_intrinsic_width = width
        .as_deref()
        .is_none_or(|width| width.trim().eq_ignore_ascii_case("fit-content"));
    let lines: Vec<_> = code.split('\n').collect();
    let first = u64::from(start_line_number.max(1));
    let last_index = lines.len() - 1;
    let digits = (first + last_index as u64).to_string().len();
    let mut root_style = format!("--_code-block-digits:{digits}ch;");
    if let Some(width) = width {
        root_style.push_str("inline-size:");
        root_style.push_str(&width);
        root_style.push(';');
    }
    let copy_source = code.clone();
    let region_id = use_hook(|| format!("astryx-code-region-{}", current_scope_id().0));
    let mut collapsed = use_signal(|| false);
    let can_collapse =
        is_collapsible && (title.is_some() || is_diff) && lines.len() >= collapsible_threshold;
    if !can_collapse && *collapsed.peek() {
        collapsed.set(false);
    }
    let is_collapsed = can_collapse && collapsed();
    let header_text = match (title.as_deref(), is_diff) {
        (Some(title), true) if !title.trim().is_empty() => format!("{title} — diff"),
        (_, true) => "diff".to_owned(),
        (title, false) => title.unwrap_or_default().to_owned(),
    };
    let has_header = title.is_some() || is_diff;
    let end_content = end_content.filter(is_renderable);
    let has_actions = has_copy_button || end_content.is_some();
    let actions = has_actions.then(|| rsx! {
        div { class: "astryx-code-block__actions", {end_content}
            if has_copy_button {
                Button { label: copy_label, variant: ButtonVariant::Ghost, size: ButtonSize::Small,
                    onclick: move |_| {
                        if matches!(*copy_feedback.peek(), CopyFeedback::Pending) { return; }
                        copy_feedback.set(CopyFeedback::Pending);
                        let source = copy_source.clone();
                        let success = copied_message.clone();
                        let failure = copy_error_message.clone();
                        spawn(async move {
                            let eval = document::eval(include_str!("copy_code.js"));
                            let copied = eval.send(source).is_ok() && eval.join::<bool>().await.unwrap_or(false);
                            copy_feedback.set(if copied {
                                CopyFeedback::Succeeded(success)
                            } else {
                                CopyFeedback::Failed(failure)
                            });
                        });
                    },
                }
            }
        }
    });
    let (header_actions, corner_actions) = if has_header {
        (actions, None)
    } else {
        (None, actions)
    };
    let feedback = copy_feedback.read();
    rsx! {
        div { class: "astryx-code-block", style: root_style,
            "data-intrinsic-width": has_intrinsic_width.then_some("true"),
            "data-corner-actions": (!has_header && has_actions).then_some("true"),
            if has_header {
                div { class: "astryx-code-block__header",
                    if can_collapse {
                        button { r#type: "button", class: "astryx-code-block__disclosure",
                            "aria-expanded": (!is_collapsed).to_string(), "aria-controls": region_id.clone(),
                            "aria-label": header_text.trim().is_empty().then_some(disclosure_label),
                            onclick: move |_| collapsed.toggle(),
                            span { class: "astryx-code-block__chevron", "aria-hidden": "true", if is_collapsed { "›" } else { "⌄" } }
                            "{header_text}"
                        }
                    } else { span { "{header_text}" } }
                    {header_actions}
                }
            }
            pre { id: region_id, class: "astryx-code-block__pre", style, dir: "ltr", tabindex: 0, hidden: is_collapsed,
                "data-wrapped": is_wrapped.to_string(),
                code {
                    for (index, &line) in lines.iter().enumerate() {
                        {
                            let number = first + index as u64;
                            let highlighted = highlight_lines.iter().any(|&line| u64::from(line) == number);
                            let kind = is_diff.then(|| classify_diff_line(&lines, index));
                            let (marker, line) = match kind {
                                Some(DiffLineKind::Added | DiffLineKind::Removed) => line.split_at(1),
                                Some(DiffLineKind::Context) => line.strip_prefix(' ').map_or(("", line), |rest| (" ", rest)),
                                _ => ("", line),
                            };
                            let text = if index < last_index {
                                format!("{line}\n")
                            } else { line.to_owned() };
                            rsx! {
                                span { key: "{index}", class: "astryx-code-block__line",
                                    "data-diff": kind.map(DiffLineKind::as_str),
                                    "data-highlighted": highlighted.then_some("true"),
                                    if has_line_numbers {
                                        span { class: "astryx-code-block__number", "aria-hidden": "true", "data-number": "{number}" }
                                    }
                                    if matches!(kind, Some(DiffLineKind::Context | DiffLineKind::Added | DiffLineKind::Removed)) {
                                        span { class: "astryx-code-block__diff-marker", "{marker}" }
                                    }
                                    span { "{text}" }
                                }
                            }
                        }
                    }
                }
            }
            {corner_actions}
            if has_copy_button {
                // Mount both live regions before updating either so a copy result
                // changes content without changing announcement priority in place.
                VisuallyHidden { "data-code-block-status": "", role: "status",
                    if let CopyFeedback::Succeeded(message) = &*feedback { "{message}" }
                }
                VisuallyHidden { "data-code-block-error": "", role: "alert",
                    if let CopyFeedback::Failed(message) = &*feedback { "{message}" }
                }
            }
        }
    }
}
