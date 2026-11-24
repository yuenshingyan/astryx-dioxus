use dioxus::prelude::*;

use crate::date_input::is_valid_date;
use crate::{DateInput, SizeValue};

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
    #[props(default)] is_disabled: bool,
    #[props(default)] width: Option<SizeValue>,
) -> Element {
    let mut draft = use_signal(|| None::<Draft>);
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
    let has_error = start
        .as_deref()
        .zip(end.as_deref())
        .is_some_and(|(start, end)| !is_allowed(start, end, min.as_deref(), max.as_deref()));
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
                is_allowed(start, end, min_bound.as_deref(), max_bound.as_deref())
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
                    on_change: move |start| update.call((start, current_end.clone())), }
                DateInput { label: end_label, value: end, min, max, is_disabled,
                    on_change: move |end| update.call((current_start.clone(), end)), }
            }
            div { role: "status", if has_error { "{invalid_range_message}" } }
        }
    }
}

struct Draft {
    base: Option<DateRange>,
    start: Option<String>,
    end: Option<String>,
}

fn is_allowed(start: &str, end: &str, min: Option<&str>, max: Option<&str>) -> bool {
    start <= end && min.is_none_or(|min| start >= min) && max.is_none_or(|max| end <= max)
}
