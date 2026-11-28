use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::date_input::is_valid_date;
use crate::field::input_clear_button::InputClearButton;
use crate::{DateInput, FieldStatusInput, InputStatusType, SizeValue};

/// An inclusive Gregorian date range. Both endpoints use `YYYY-MM-DD`.
#[derive(Clone, PartialEq, Eq)]
pub struct DateRange {
    /// First day, inclusive.
    pub start: String,
    /// Last day, inclusive.
    pub end: String,
}

/// Date range entry using two native Gregorian date controls.
/// The caller owns the committed range; incomplete edits do not emit a range.
#[component]
pub fn DateRangeInput(
    label: String,
    on_change: EventHandler<Option<DateRange>>,
    #[props(default)] value: Option<DateRange>,
    #[props(default = "Start date".to_owned())] start_label: String,
    #[props(default = "End date".to_owned())] end_label: String,
    #[props(default = "Enter dates in order and within the allowed bounds.".to_owned())]
    invalid_range_message: String,
    #[props(default)] min: Option<String>,
    #[props(default)] max: Option<String>,
    /// Minimum span in calendar days, counting both endpoints.
    #[props(default)]
    min_range_span: Option<u32>,
    /// Maximum span in calendar days, counting both endpoints.
    #[props(default)]
    max_range_span: Option<u32>,
    /// Message shown when an otherwise valid range violates the span limits.
    #[props(default = "Choose a range within the allowed number of days.".to_owned())]
    invalid_span_message: String,
    #[props(default)] is_disabled: bool,
    /// Allow clearing a committed range. No action is shown for an incomplete draft.
    #[props(default = true)]
    has_clear: bool,
    /// Accessible clear button name. Defaults to `Clear {label}`.
    #[props(default)]
    clear_label: Option<String>,
    #[props(default)] width: Option<SizeValue>,
) -> Element {
    let error_id = use_hook(|| format!("astryx-date-range-error-{}", current_scope_id().0));
    let mut draft = use_signal(|| None::<Draft>);
    let mut start_node = use_signal(|| None::<MountedEvent>);
    use_effect(use_reactive((&value,), move |_| {
        draft.set(None);
    }));
    if [&min, &max]
        .into_iter()
        .flatten()
        .any(|date| !is_valid_date(date))
        || min
            .as_ref()
            .zip(max.as_ref())
            .is_some_and(|(min, max)| min > max)
        || min_range_span == Some(0)
        || max_range_span == Some(0)
        || min_range_span
            .zip(max_range_span)
            .is_some_and(|(min, max)| min > max)
        || value.as_ref().is_some_and(|range| {
            !is_valid_date(&range.start) || !is_valid_date(&range.end) || range.start > range.end
        })
    {
        return VNode::empty();
    }
    let style = width
        .map(|width| width.as_css().map(|width| format!("inline-size:{width}")))
        .transpose()?;
    let (start, end) = draft
        .read()
        .as_ref()
        .filter(|draft| draft.base == value)
        .map(|draft| (draft.start.clone(), draft.end.clone()))
        .or_else(|| {
            value
                .as_ref()
                .map(|range| (Some(range.start.clone()), Some(range.end.clone())))
        })
        .unwrap_or_default();
    let span_bounds = (min_range_span, max_range_span);
    let message = start
        .as_deref()
        .zip(end.as_deref())
        .and_then(|(start, end)| {
            range_error(start, end, min.as_deref(), max.as_deref(), span_bounds).map(|error| {
                match error {
                    RangeError::Endpoints => invalid_range_message.as_str(),
                    RangeError::Span => invalid_span_message.as_str(),
                }
            })
        });
    let described_by = message
        .filter(|text| !text.trim().is_empty())
        .map(|_| error_id.clone());
    let endpoint_status = message.is_some().then_some(FieldStatusInput {
        status_type: InputStatusType::Error,
        message: None,
        message_id: None,
    });
    let should_show_clear = has_clear && value.is_some() && !is_disabled;
    let clear_label = clear_label
        .filter(|text| !text.trim().is_empty())
        .unwrap_or_else(|| format!("Clear {label}"));
    let current_start = start.clone();
    let current_end = end.clone();
    let min_bound = min.clone();
    let max_bound = max.clone();
    let update = Callback::new(move |(start, end): (Option<String>, Option<String>)| {
        if is_disabled {
            return;
        }
        let next = start
            .as_deref()
            .zip(end.as_deref())
            .filter(|&(start, end)| {
                range_error(
                    start,
                    end,
                    min_bound.as_deref(),
                    max_bound.as_deref(),
                    span_bounds,
                )
                .is_none()
            })
            .map(|(start, end)| DateRange {
                start: start.to_owned(),
                end: end.to_owned(),
            });
        draft.set(Some(Draft {
            base: value.clone(),
            start,
            end,
        }));
        if let Some(next) = next {
            if value.as_ref() != Some(&next) {
                on_change.call(Some(next));
            }
        } else if value.is_some()
            && draft
                .peek()
                .as_ref()
                .is_some_and(|draft| draft.start.is_none() || draft.end.is_none())
        {
            on_change.call(None);
        }
    });
    rsx! {
        fieldset { class: "astryx-date-range-input", style, disabled: is_disabled,
            legend { "{label}" }
            div { class: "astryx-date-range-input__segments",
                DateInput { label: start_label, value: start, min: min.clone(), max: max.clone(), is_disabled,
                    status: endpoint_status.clone(), described_by: described_by.clone(),
                    onmounted: move |event| start_node.set(Some(event)),
                    on_change: move |start| update.call((start, current_end.clone())), }
                DateInput { label: end_label, value: end, min, max, is_disabled,
                    status: endpoint_status, described_by,
                    on_change: move |end| update.call((current_start.clone(), end)), }
                if should_show_clear {
                    InputClearButton { label: clear_label, onclick: move |_| {
                        let start = start_node.peek().clone();
                        draft.set(None);
                        on_change.call(None);
                        if let Some(start) = start {
                            spawn(async move { let _ = start.data().set_focus(true).await; });
                        }
                    } }
                }
            }
            div { id: error_id, role: "status", if let Some(message) = message { "{message}" } }
        }
    }
}

struct Draft {
    base: Option<DateRange>,
    start: Option<String>,
    end: Option<String>,
}

enum RangeError {
    Endpoints,
    Span,
}

fn range_error(
    start: &str,
    end: &str,
    min: Option<&str>,
    max: Option<&str>,
    (min_span, max_span): (Option<u32>, Option<u32>),
) -> Option<RangeError> {
    if start > end || min.is_some_and(|min| start < min) || max.is_some_and(|max| end > max) {
        return Some(RangeError::Endpoints);
    }
    if min_span.is_some() || max_span.is_some() {
        let Some((start, end)) = day_ordinal(start).zip(day_ordinal(end)) else {
            return Some(RangeError::Endpoints);
        };
        let span = end - start + 1;
        if min_span.is_some_and(|min| span < min) || max_span.is_some_and(|max| span > max) {
            return Some(RangeError::Span);
        }
    }
    None
}

// Count civil days without a time zone or daylight-saving transitions.
fn day_ordinal(date: &str) -> Option<u32> {
    let year = date.get(..4)?.parse::<u32>().ok()?;
    let month = date.get(5..7)?.parse::<usize>().ok()?;
    let day = date.get(8..10)?.parse::<u32>().ok()?;
    let completed = year.checked_sub(1)?;
    let before_month = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    Some(
        365 * completed + completed / 4 - completed / 100
            + completed / 400
            + before_month.get(month.checked_sub(1)?)?
            + u32::from(leap && month > 2)
            + day,
    )
}

#[cfg(test)]
mod tests {
    use super::{RangeError, range_error};

    #[test]
    fn span_counts_both_endpoints_across_leap_days_and_years() {
        let exactly_three = (Some(3), Some(3));
        for (start, end) in [("2024-02-28", "2024-03-01"), ("2025-12-30", "2026-01-01")] {
            assert!(range_error(start, end, None, None, exactly_three).is_none());
        }
        assert!(matches!(
            range_error("2100-02-28", "2100-03-01", None, None, exactly_three),
            Some(RangeError::Span)
        ));
        assert!(matches!(
            range_error("2024-02-28", "2024-03-02", None, None, exactly_three),
            Some(RangeError::Span)
        ));
    }
}
