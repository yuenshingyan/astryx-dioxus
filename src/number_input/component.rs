use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SizeValue};

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
    /// Caller-owned validation status, independent of native constraints.
    #[props(default)]
    status: Option<FieldStatusInput>,
    /// Display the status attached to the input or below it as a separate message.
    #[props(default)]
    status_variant: FieldStatusPlacement,
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
    let description_id = description
        .as_ref()
        .filter(|text| !text.is_empty())
        .map(|_| format!("{id}-description"));
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
    rsx! {
        Field { class: "astryx-number-input", label, input_id: id.clone(), is_label_hidden, is_disabled, is_required,
            width, description, description_id, status, status_variant,
            input { class: "astryx-number-input__control", id, r#type: "number", name: html_name,
                value: value.map_or_else(String::new, |value| value.to_string()),
                min: min.map(|min| min.to_string()), max: max.map(|max| max.to_string()),
                step: step.map_or_else(|| "any".to_owned(), |step| step.to_string()),
                placeholder, required: is_required, readonly: is_read_only, disabled: is_disabled,
                "aria-describedby": (!described_by.is_empty()).then_some(described_by),
                "aria-invalid": (severity == Some(InputStatusType::Error)).then_some("true"),
                "data-status": severity.map(InputStatusType::as_str),
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
