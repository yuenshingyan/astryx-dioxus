use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{FieldLabel, SizeValue};

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
    #[props(default)] html_name: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] is_required: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_optional: bool,
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
    let style = width
        .map(|size| size.as_css().map(|width| format!("inline-size:{width}")))
        .transpose()?;
    let description = description.filter(|text| !text.is_empty());
    let description_id = description.as_ref().map(|_| format!("{id}-description"));
    rsx! {
        div { class: "astryx-time-input", style,
            FieldLabel { label, input_id: id.clone(), is_label_hidden, is_disabled, is_required, is_optional,
                description: description.map(|text| rsx! { "{text}" }), description_id: description_id.clone(),
            }
            input { class: "astryx-time-input__control", id, r#type: "time", name: html_name,
                value: value.unwrap_or_default(), min, max, step: if has_seconds { "1" } else { "60" },
                required: is_required, disabled: is_disabled, "aria-describedby": description_id,
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
        }
    }
}

fn is_valid_time(value: &str, has_seconds: bool) -> bool {
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
