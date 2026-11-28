use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::field::input_clear_button::InputClearButton;
use crate::{
    Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SizeValue, Spinner, SpinnerSize,
};

/// Gregorian date field for the native picker presentation. Values and bounds
/// use `YYYY-MM-DD`; invalid dates or reversed bounds render no field.
#[component]
pub fn DateInput(
    label: String,
    #[props(default)] value: Option<String>,
    #[props(default)] on_change: Option<EventHandler<Option<String>>>,
    #[props(default)] min: Option<String>,
    #[props(default)] max: Option<String>,
    #[props(default)] description: Option<String>,
    /// IDs of additional elements that describe the date control.
    #[props(default)]
    described_by: Option<String>,
    /// Caller-owned validation feedback, independent of native date constraints.
    #[props(default)]
    status: Option<FieldStatusInput>,
    /// Placement of the validation message.
    #[props(default)]
    status_variant: FieldStatusPlacement,
    #[props(default)] html_name: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] is_required: bool,
    #[props(default)] is_disabled: bool,
    /// Lock editing while caller-owned work runs, without removing the value from form data.
    #[props(default)]
    is_loading: bool,
    #[props(default)] is_optional: bool,
    /// Show a clear action when a date is set and the field is enabled.
    #[props(default)]
    has_clear: bool,
    /// Accessible clear button name. Defaults to `Clear {label}`.
    #[props(default)]
    clear_label: Option<String>,
    #[props(default)] is_label_hidden: bool,
    /// Receives the mounted native date control.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
) -> Element {
    let id = use_hook(|| format!("astryx-date-input-{}", current_scope_id().0));
    if [&value, &min, &max]
        .into_iter()
        .flatten()
        .any(|date| !is_valid_date(date))
        || min
            .as_ref()
            .zip(max.as_ref())
            .is_some_and(|(min, max)| min > max)
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
    let described_by = [
        description_id.as_deref(),
        status_id.as_deref(),
        described_by.as_deref(),
    ]
    .into_iter()
    .flatten()
    .flat_map(str::split_ascii_whitespace)
    .collect::<Vec<_>>()
    .join(" ");
    let should_show_clear = has_clear && value.is_some() && !is_disabled && !is_loading;
    let clear_label = clear_label
        .filter(|text| !text.trim().is_empty())
        .unwrap_or_else(|| format!("Clear {label}"));
    let mut input_node = use_signal(|| None::<MountedEvent>);
    rsx! {
        Field { class: "astryx-date-input", label, input_id: id.clone(), is_label_hidden, is_disabled, is_required, is_optional,
            width, description, description_id, status, status_variant,
            div { class: "astryx-date-input__entry", "data-has-clear": should_show_clear.then_some("true"),
                "data-loading": is_loading.then_some("true"),
            input { class: "astryx-date-input__control", id, r#type: "date", name: html_name,
                value: value.unwrap_or_default(), min, max,
                required: is_required, disabled: is_disabled, readonly: is_loading,
                "aria-busy": is_loading.then_some("true"), "aria-disabled": is_loading.then_some("true"),
                "aria-describedby": (!described_by.is_empty()).then_some(described_by),
                "aria-invalid": (severity == Some(InputStatusType::Error)).then_some("true"),
                "data-status": severity.map(InputStatusType::as_str),
                onmounted: move |event: MountedEvent| {
                    input_node.set(Some(event.clone()));
                    if let Some(handler) = onmounted { handler.call(event); }
                },
                onchange: move |event| {
                    if !is_disabled && !is_loading && let Some(handler) = on_change {
                        let date = event.value();
                        if date.is_empty() { handler.call(None); }
                        else if is_valid_date(&date) { handler.call(Some(date)); }
                    }
                },
            }
            if is_loading { Spinner { size: SpinnerSize::Small, is_decorative: true } }
            if should_show_clear {
                InputClearButton { label: clear_label, onclick: move |_| {
                    let input = input_node.peek().clone();
                    if let Some(handler) = on_change { handler.call(None); }
                    if let Some(input) = input {
                        spawn(async move { let _ = input.data().set_focus(true).await; });
                    }
                } }
            }
            }
        }
    }
}

pub(crate) fn is_valid_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 10
        || bytes.iter().enumerate().any(|(index, &byte)| {
            if index == 4 || index == 7 {
                byte != b'-'
            } else {
                !byte.is_ascii_digit()
            }
        })
    {
        return false;
    }
    let Ok(year) = value[..4].parse::<u16>() else {
        return false;
    };
    let Ok(month) = value[5..7].parse::<u8>() else {
        return false;
    };
    let Ok(day) = value[8..].parse::<u8>() else {
        return false;
    };
    let month_end = match month {
        4 | 6 | 9 | 11 => 30,
        2 if year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) => {
            29
        }
        2 => 28,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => return false,
    };
    year != 0 && (1..=month_end).contains(&day)
}

#[cfg(test)]
mod tests {
    use super::is_valid_date;

    #[test]
    fn accepts_only_real_four_digit_gregorian_dates() {
        for date in ["0001-01-01", "2000-02-29", "2024-02-29", "9999-12-31"] {
            assert!(is_valid_date(date), "{date}");
        }
        for date in [
            "0000-01-01",
            "1900-02-29",
            "2024-04-31",
            "2024-00-01",
            "2024-01-00",
            "2024-1-01",
            "2024-01-0é",
        ] {
            assert!(!is_valid_date(date), "{date}");
        }
    }
}
