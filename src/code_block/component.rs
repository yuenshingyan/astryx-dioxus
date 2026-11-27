use dioxus::prelude::*;

use crate::{Button, ButtonSize, ButtonVariant, SizeValue, VisuallyHidden};

use super::tokenizer::{DiffLineKind, classify_diff_line};

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
    #[props(default)] is_wrapped: bool,
    #[props(default)] max_height: Option<SizeValue>,
    #[props(default)] has_copy_button: bool,
    #[props(default = "Copy code".to_owned())] copy_label: String,
    #[props(default = "Copied code".to_owned())] copied_message: String,
    #[props(default = "Could not copy code".to_owned())] copy_error_message: String,
) -> Element {
    let mut copy_status = use_signal(String::new);
    let mut is_copying = use_signal(|| false);
    let style = match max_height {
        Some(value) => format!("max-block-size:{}", value.as_css()?),
        None => String::new(),
    };
    let lines: Vec<_> = code.split('\n').collect();
    let first = u64::from(start_line_number.max(1));
    let last_index = lines.len() - 1;
    let digits = (first + last_index as u64).to_string().len();
    let copy_source = code.clone();
    rsx! {
        div { class: "astryx-code-block", style: "--_code-block-digits:{digits}ch",
            if title.is_some() || has_copy_button || is_diff {
                div { class: "astryx-code-block__header",
                    if let Some(title) = title.as_ref() { span { "{title}" } }
                    if is_diff { span { "diff" } }
                    if has_copy_button {
                        Button { label: copy_label, variant: ButtonVariant::Ghost, size: ButtonSize::Small,
                            onclick: move |_| {
                                if *is_copying.peek() { return; }
                                is_copying.set(true);
                                copy_status.set(String::new());
                                let source = copy_source.clone();
                                let success = copied_message.clone();
                                let failure = copy_error_message.clone();
                                spawn(async move {
                                    let eval = document::eval(include_str!("copy_code.js"));
                                    let copied = eval.send(source).is_ok() && eval.join::<bool>().await.unwrap_or(false);
                                    copy_status.set(if copied { success } else { failure });
                                    is_copying.set(false);
                                });
                            },
                        }
                    }
                }
            }
            pre { class: "astryx-code-block__pre", style, dir: "ltr", tabindex: 0,
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
            if has_copy_button {
                VisuallyHidden { "data-code-block-status": "", role: "status", "{copy_status}" }
            }
        }
    }
}
