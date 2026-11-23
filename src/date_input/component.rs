use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{FieldLabel, SizeValue};

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
    #[props(default)] html_name: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] is_required: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_optional: bool,
    #[props(default)] is_label_hidden: bool,
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
    let style = width
        .map(|size| size.as_css().map(|width| format!("inline-size:{width}")))
        .transpose()?;
    let description = description.filter(|text| !text.is_empty());
    let description_id = description.as_ref().map(|_| format!("{id}-description"));
    rsx! {
        div { class: "astryx-date-input", style,
            FieldLabel { label, input_id: id.clone(), is_label_hidden, is_disabled, is_required, is_optional,
                description: description.map(|text| rsx! { "{text}" }), description_id: description_id.clone(),
            }
            input { class: "astryx-date-input__control", id, r#type: "date", name: html_name,
                value: value.unwrap_or_default(), min, max,
                required: is_required, disabled: is_disabled, "aria-describedby": description_id,
                onchange: move |event| {
                    if !is_disabled && let Some(handler) = on_change {
                        let date = event.value();
                        if date.is_empty() { handler.call(None); }
                        else if is_valid_date(&date) { handler.call(Some(date)); }
                    }
                },
            }
        }
    }
}

fn is_valid_date(value: &str) -> bool {
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
