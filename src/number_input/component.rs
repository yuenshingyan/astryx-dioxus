use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{FieldLabel, SizeValue};

/// Native numeric field with browser-owned editing and constraint validation.
/// The change callback reports a committed number, or `None` for an empty DOM value.
#[component]
pub fn NumberInput(
    label: String,
    value: Option<f64>,
    #[props(default)] on_change: Option<EventHandler<Option<f64>>>,
    #[props(default)] min: Option<f64>,
    #[props(default)] max: Option<f64>,
    #[props(default)] step: Option<f64>,
    #[props(default)] placeholder: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] html_name: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] is_required: bool,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_read_only: bool,
    #[props(default)] is_label_hidden: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-number-input-{}", current_scope_id().0));
    if [value, min, max, step]
        .into_iter()
        .flatten()
        .any(|n| !n.is_finite())
        || min.zip(max).is_some_and(|(min, max)| min > max)
        || step.is_some_and(|step| step <= 0.0)
    {
        return VNode::empty();
    }
    let style = width
        .map(|size| size.as_css().map(|width| format!("inline-size:{width}")))
        .transpose()?;
    let description = description.filter(|text| !text.is_empty());
    let description_id = description.as_ref().map(|_| format!("{id}-description"));
    rsx! {
        div { class: "astryx-number-input", style,
            FieldLabel { label, input_id: id.clone(), is_label_hidden, is_disabled, is_required,
                description: description.map(|text| rsx! { "{text}" }), description_id: description_id.clone(),
            }
            input { class: "astryx-number-input__control", id, r#type: "number", name: html_name,
                value: value.map_or_else(String::new, |value| value.to_string()),
                min: min.map(|min| min.to_string()), max: max.map(|max| max.to_string()),
                step: step.map_or_else(|| "any".to_owned(), |step| step.to_string()),
                placeholder, required: is_required, readonly: is_read_only, disabled: is_disabled,
                "aria-describedby": description_id,
                onchange: move |event| {
                    if !is_disabled && !is_read_only && let Some(handler) = on_change {
                        let raw = event.value();
                        if raw.is_empty() { handler.call(None); }
                        else if let Ok(number) = raw.parse::<f64>() && number.is_finite() {
                            handler.call(Some(number));
                        }
                    }
                },
            }
        }
    }
}
