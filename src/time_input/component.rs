use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::field::input_clear_button::InputClearButton;
use crate::{Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SizeValue};

/// Native time-of-day field for the browser picker presentation.
/// Values and bounds use `HH:MM` or `HH:MM:SS` without a date or time zone.
#[component]
pub fn TimeInput(
    label: String,
    #[props(default)] value: Option<String>,
    #[props(default)] on_change: Option<EventHandler<Option<String>>>,
    #[props(default)] min: Option<String>,
    #[props(default)] max: Option<String>,
    #[props(default)] has_seconds: bool,
    #[props(default)] description: Option<String>,
    /// Caller-owned feedback, independent of native time constraints.
    #[props(default)]
    status: Option<FieldStatusInput>,
    /// Placement of the validation message.
    #[props(default)]
    status_variant: FieldStatusPlacement,
    #[props(default)] html_name: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] is_required: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_optional: bool,
    /// Show a clear action when a time is set and the field is enabled.
    #[props(default)]
    has_clear: bool,
    /// Accessible clear button name. Defaults to `Clear {label}`.
    #[props(default)]
    clear_label: Option<String>,
    #[props(default)] is_label_hidden: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-time-input-{}", current_scope_id().0));
    if [&value, &min, &max]
        .into_iter()
        .flatten()
        .any(|time| !is_valid_time(time, has_seconds))
    {
        return VNode::empty();
    }
    let description = description.filter(|text| !text.is_empty());
    let description_id = description.as_ref().map(|_| format!("{id}-description"));
    let severity = status.as_ref().map(|status| status.status_type);
    let status_id = status
        .as_ref()
        .filter(|status| {
            status
                .message
                .as_ref()
                .is_some_and(|message| !message.is_empty())
        })
        .map(|status| {
            status
                .message_id
                .clone()
                .unwrap_or_else(|| format!("{id}-status"))
        });
    let described_by = [description_id.as_deref(), status_id.as_deref()]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
    let should_show_clear = has_clear && value.is_some() && !is_disabled;
    let clear_label = clear_label
        .filter(|text| !text.trim().is_empty())
        .unwrap_or_else(|| format!("Clear {label}"));
    rsx! {
        Field { class: "astryx-time-input", label, input_id: id.clone(), is_label_hidden, is_disabled, is_required, is_optional,
            width, description, description_id, status, status_variant,
            div { class: "astryx-time-input__entry", "data-has-clear": should_show_clear.then_some("true"),
            input { class: "astryx-time-input__control", id, r#type: "time", name: html_name,
                value: value.unwrap_or_default(), min, max, step: if has_seconds { "1" } else { "60" },
                required: is_required, disabled: is_disabled,
                "aria-describedby": (!described_by.is_empty()).then_some(described_by),
                "aria-invalid": (severity == Some(InputStatusType::Error)).then_some("true"),
                "data-status": severity.map(InputStatusType::as_str),
                onchange: move |event| {
                    if !is_disabled && let Some(handler) = on_change {
                        let mut time = event.value();
                        if time.is_empty() { handler.call(None); }
                        else if is_valid_time(&time, has_seconds) {
                            if has_seconds && time.len() == 5 { time.push_str(":00"); }
                            handler.call(Some(time));
                        }
                    }
                },
            }
            if should_show_clear {
                InputClearButton { label: clear_label, onclick: move |_| {
                    // Focusing the native time input can reopen the picker on iOS.
                    if let Some(handler) = on_change { handler.call(None); }
                } }
            }
            }
        }
    }
}

pub(crate) fn is_valid_time(value: &str, has_seconds: bool) -> bool {
    fn pair(first: u8, second: u8, upper: u8) -> bool {
        first.is_ascii_digit()
            && second.is_ascii_digit()
            && (first - b'0') * 10 + second - b'0' < upper
    }
    match *value.as_bytes() {
        [a, b, b':', c, d] => pair(a, b, 24) && pair(c, d, 60),
        [a, b, b':', c, d, b':', e, f] if has_seconds => {
            pair(a, b, 24) && pair(c, d, 60) && pair(e, f, 60)
        }
        _ => false,
    }
}
