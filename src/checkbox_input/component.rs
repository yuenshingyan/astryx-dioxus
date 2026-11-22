use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{
    CheckboxIndicator, CheckboxIndicatorState, FieldLabel, FieldStatus, FieldStatusInput,
    FieldStatusVariant, IndicatorSize, InputStatusType,
};

/// Controlled native checkbox with a decorative indicator and field label.
#[component]
pub fn CheckboxInput(
    label: String,
    value: CheckboxIndicatorState,
    #[props(default)] on_change: Option<EventHandler<bool>>,
    #[props(default)] id: Option<String>,
    #[props(default)] html_name: Option<String>,
    #[props(default)] description: Option<String>,
    /// ID references for additional descriptions owned by the caller.
    #[props(default)]
    described_by: Option<String>,
    /// ID references for a visible label owned by a composite control.
    #[props(default)]
    labelled_by: Option<String>,
    #[props(default)] status: Option<FieldStatusInput>,
    #[props(default)] is_disabled: bool,
    #[props(default)] is_read_only: bool,
    #[props(default)] is_required: bool,
    #[props(default)] is_label_hidden: bool,
    #[props(default)] size: IndicatorSize,
) -> Element {
    let owner = use_hook(|| format!("astryx-checkbox-input-{}", current_scope_id().0));
    let id = id.unwrap_or_else(|| owner.clone());
    let label_input_id = id.clone();
    let is_checked = value == CheckboxIndicatorState::Checked;
    let is_mixed = value == CheckboxIndicatorState::Indeterminate;
    let is_error = status
        .as_ref()
        .is_some_and(|status| status.status_type == InputStatusType::Error);
    let description = description.filter(|message| !message.is_empty());
    let description_id = description.as_ref().map(|_| format!("{id}-desc"));
    let status = status.and_then(|status| {
        Some((
            status.status_type,
            status.message.filter(|message| !message.is_empty())?,
            status.message_id.unwrap_or_else(|| format!("{id}-status")),
        ))
    });
    let described_by = [
        described_by.as_deref(),
        description_id.as_deref(),
        status.as_ref().map(|(_, _, id)| id.as_str()),
    ]
    .into_iter()
    .flatten()
    .flat_map(str::split_whitespace)
    .collect::<Vec<_>>()
    .join(" ");
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    rsx! {
        div { class: "astryx-checkbox-input",
            div { class: "astryx-checkbox-input__row", "data-label-hidden": is_label_hidden.then_some("true"),
                span { class: "astryx-checkbox-input__control", "data-size": if size == IndicatorSize::Small { "sm" } else { "md" },
                    input { r#type: "checkbox", id: id.clone(), name: (!is_disabled).then_some(html_name).flatten(),
                        form: is_disabled.then_some(String::new()), required: is_required, checked: is_checked,
                        "data-astryx-checkbox-owner": owner.clone(), "data-checked": is_checked.to_string(), "data-mixed": is_mixed.to_string(),
                        "aria-checked": is_mixed.then_some("mixed"), "aria-disabled": is_disabled.then_some("true"),
                        "aria-readonly": is_read_only.then_some("true"),
                        "aria-invalid": (is_error || is_required).then(|| is_error.to_string()),
                        "aria-describedby": (!described_by.is_empty()).then_some(described_by), "aria-labelledby": labelled_by,
                        onmounted: move |_| {
                            if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                            let eval = document::eval(include_str!("sync_checkbox.js"));
                            let _ = eval.send(owner.clone());
                            controller.set(Some(eval));
                        },
                        onclick: move |event| { if is_disabled || is_read_only { event.prevent_default(); } },
                        onchange: move |event| {
                            if !is_disabled && !is_read_only && let Some(handler) = on_change { handler.call(event.checked()); }
                        },
                    }
                    CheckboxIndicator { state: value, size, is_disabled }
                }
                FieldLabel { label, input_id: label_input_id, is_label_hidden, is_disabled, is_required,
                    description: description.map(|message| rsx! { "{message}" }), description_id,
                }
            }
            if let Some((severity, message, id)) = status {
                FieldStatus { r#type: severity, message, id, variant: FieldStatusVariant::DETACHED }
            }
        }
    }
}
