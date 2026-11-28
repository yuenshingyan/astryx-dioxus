use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::date_input::is_valid_date;
use crate::field::input_clear_button::InputClearButton;
use crate::time_input::is_valid_time;
use crate::{Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SizeValue};

/// A local Gregorian date and time, entered through separate native controls.
/// The value uses `YYYY-MM-DDTHH:MM`, or includes seconds when enabled.
#[component]
pub fn DateTimeInput(
    label: String,
    #[props(default)] value: Option<String>,
    #[props(default)] on_change: Option<EventHandler<Option<String>>>,
    #[props(default = "Time".to_owned())] time_label: String,
    #[props(default)] html_name: Option<String>,
    /// Describes both date and time controls.
    #[props(default)]
    description: Option<String>,
    /// Caller-owned feedback for the combined value, shown below the controls.
    #[props(default)]
    status: Option<FieldStatusInput>,
    #[props(default)] has_seconds: bool,
    /// Inclusive lower bound in local ISO datetime format.
    #[props(default)]
    min: Option<String>,
    /// Inclusive upper bound in local ISO datetime format.
    #[props(default)]
    max: Option<String>,
    #[props(default)] is_required: bool,
    #[props(default)] is_disabled: bool,
    /// Show a clear action when the combined value is set and the field is enabled.
    #[props(default)]
    has_clear: bool,
    /// Accessible clear button name. Defaults to `Clear {label}`.
    #[props(default)]
    clear_label: Option<String>,
    #[props(default)] width: Option<SizeValue>,
) -> Element {
    let id = use_hook(|| format!("astryx-date-time-input-{}", current_scope_id().0));
    let mut time_draft = use_signal(|| None::<String>);
    let mut pending_date = use_signal(|| None::<String>);
    let (date, time) = if let Some(value) = value.as_deref() {
        let Some((date, time)) = value.split_once('T') else {
            return VNode::empty();
        };
        if !is_valid_date(date) || !is_valid_time(time, has_seconds) {
            return VNode::empty();
        }
        (
            date.to_owned(),
            Some(normalize_time(time.to_owned(), has_seconds)),
        )
    } else {
        (String::new(), time_draft.read().clone())
    };
    let Ok(min_bound) = min
        .as_deref()
        .map(|value| parse_bound(value, has_seconds))
        .transpose()
    else {
        return VNode::empty();
    };
    let Ok(max_bound) = max
        .as_deref()
        .map(|value| parse_bound(value, has_seconds))
        .transpose()
    else {
        return VNode::empty();
    };
    if min_bound
        .as_ref()
        .zip(max_bound.as_ref())
        .is_some_and(|(min, max)| min > max)
    {
        return VNode::empty();
    }
    let min_date = min_bound.as_ref().map(|(date, _)| date.clone());
    let max_date = max_bound.as_ref().map(|(date, _)| date.clone());
    let time_min = bound_time(min_bound.as_ref(), &date).map(str::to_owned);
    let time_max = bound_time(max_bound.as_ref(), &date).map(str::to_owned);
    let date_id = format!("{id}-date");
    let time_id = format!("{id}-time");
    let time_value = time.clone().unwrap_or_default();
    let description_id = description
        .as_ref()
        .filter(|text| !text.is_empty())
        .map(|_| format!("{date_id}-desc"));
    let severity = status.as_ref().map(|status| status.status_type);
    let status_id = status
        .as_ref()
        .filter(|status| status.message.as_ref().is_some_and(|text| !text.is_empty()))
        .map(|status| {
            status
                .message_id
                .clone()
                .unwrap_or_else(|| format!("{date_id}-status"))
        });
    let described_by = [description_id.as_deref(), status_id.as_deref()]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
    let described_by = (!described_by.is_empty()).then_some(described_by);
    let invalid = (severity == Some(InputStatusType::Error)).then_some("true");
    let should_show_clear = has_clear && value.is_some() && !is_disabled;
    let clear_label = clear_label
        .filter(|text| !text.trim().is_empty())
        .unwrap_or_else(|| format!("Clear {label}"));
    rsx! {
        Field { class: "astryx-date-time-input", label, input_id: date_id.clone(), is_required, is_disabled,
            width, description, description_id, status, status_variant: FieldStatusPlacement::Detached,
            div { class: "astryx-date-time-input__segments",
                input { id: date_id, class: "astryx-date-input__control", r#type: "date", value: date.clone(), min: min_date, max: max_date,
                    required: is_required, disabled: is_disabled, "aria-describedby": described_by.clone(),
                    "aria-invalid": invalid, "data-status": severity.map(InputStatusType::as_str),
                    onchange: move |event| {
                        pending_date.set(None);
                        if is_disabled { return; }
                        let Some(change) = on_change else { return };
                        let next = event.value();
                        if next.is_empty() {
                            time_draft.set(None);
                            change.call(None);
                        } else if is_valid_date(&next) {
                            if let Some(time) = &time {
                                time_draft.set(None);
                                let time = clamp_time(time, &next, min_bound.as_ref(), max_bound.as_ref());
                                change.call(Some(format!("{next}T{time}")));
                            } else {
                                pending_date.set(Some(next.clone()));
                                let min_bound = min_bound.clone();
                                let max_bound = max_bound.clone();
                                spawn(async move {
                                    let result = document::eval(include_str!("local_time.js")).join::<String>().await;
                                    if let Ok(time) = result && pending_date.peek().as_deref() == Some(&next) {
                                        pending_date.set(None);
                                        let time = normalize_time(time, has_seconds);
                                        let time = clamp_time(&time, &next, min_bound.as_ref(), max_bound.as_ref());
                                        change.call(Some(format!("{next}T{time}")));
                                    }
                                });
                            }
                        }
                    },
                }
                input { id: time_id, class: "astryx-time-input__control", r#type: "time", value: time_value,
                    "aria-label": time_label, min: time_min, max: time_max, step: if has_seconds { "1" } else { "60" },
                    required: is_required, disabled: is_disabled, "aria-describedby": described_by,
                    "aria-invalid": invalid, "data-status": severity.map(InputStatusType::as_str),
                    onchange: move |event| {
                        if is_disabled { return; }
                        let next = event.value();
                        if next.is_empty() {
                            pending_date.set(None);
                            time_draft.set(None);
                            if !date.is_empty() && let Some(change) = on_change { change.call(None); }
                        } else if is_valid_time(&next, has_seconds) {
                            let next = normalize_time(next, has_seconds);
                            let selected_date = pending_date.take();
                            if let Some(date) = selected_date.as_deref().or_else(|| (!date.is_empty()).then_some(date.as_str())) {
                                time_draft.set(None);
                                if let Some(change) = on_change { change.call(Some(format!("{date}T{next}"))); }
                            } else {
                                time_draft.set(Some(next));
                            }
                        }
                    },
                }
                if should_show_clear {
                    InputClearButton { label: clear_label, onclick: move |_| {
                        pending_date.set(None);
                        time_draft.set(None);
                        // Native picker mode avoids programmatically refocusing a segment.
                        if let Some(change) = on_change { change.call(None); }
                    } }
                }
            }
            if let Some(name) = html_name {
                input { r#type: "hidden", name, value: value.unwrap_or_default(), disabled: is_disabled }
            }
        }
    }
}

type DateTimeBound = (String, String);

fn parse_bound(value: &str, has_seconds: bool) -> Result<DateTimeBound, ()> {
    let (date, time) = value.split_once('T').ok_or(())?;
    if !is_valid_date(date) || !is_valid_time(time, has_seconds) {
        return Err(());
    }
    Ok((
        date.to_owned(),
        normalize_time(time.to_owned(), has_seconds),
    ))
}

fn bound_time<'a>(bound: Option<&'a DateTimeBound>, date: &str) -> Option<&'a str> {
    bound
        .filter(|(bound_date, _)| bound_date == date)
        .map(|(_, time)| time.as_str())
}

fn clamp_time<'a>(
    time: &'a str,
    date: &str,
    min: Option<&'a DateTimeBound>,
    max: Option<&'a DateTimeBound>,
) -> &'a str {
    let time = bound_time(min, date).map_or(time, |min| time.max(min));
    bound_time(max, date).map_or(time, |max| time.min(max))
}

fn normalize_time(mut time: String, has_seconds: bool) -> String {
    if has_seconds && time.len() == 5 {
        time.push_str(":00");
    }
    if !has_seconds {
        time.truncate(5);
    }
    time
}
