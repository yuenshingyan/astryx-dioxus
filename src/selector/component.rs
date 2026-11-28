use std::collections::HashSet;

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{
    Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SizeValue, Spinner, SpinnerSize,
};

/// A plain-text option for a native selector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectorChoice {
    /// Unique nonempty value submitted by the form.
    pub value: String,
    /// Visible option label.
    pub label: String,
    /// Whether the option is unavailable for selection.
    pub is_disabled: bool,
}

/// A controlled single-choice field backed by a native select element.
#[component]
pub fn Selector(
    label: String,
    options: Vec<SelectorChoice>,
    value: Option<String>,
    on_change: EventHandler<String>,
    #[props(default = "Select an option".to_owned())] placeholder: String,
    #[props(default)] description: Option<String>,
    /// Caller-supplied feedback, independent of native required validation.
    #[props(default)]
    status: Option<FieldStatusInput>,
    /// Placement of the validation message.
    #[props(default)]
    status_variant: FieldStatusPlacement,
    #[props(default)] html_name: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] is_disabled: bool,
    /// Show progress for caller-owned work without disabling existing options.
    #[props(default)]
    is_loading: bool,
    #[props(default)] is_required: bool,
    #[props(default)] is_label_hidden: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-selector-{}", current_scope_id().0));
    let mount_id = id.clone();
    let mut controller = use_signal(|| None::<document::Eval>);
    use_drop(move || {
        if let Some(eval) = *controller.peek() {
            let _ = eval.send(());
        }
    });
    let mut seen = HashSet::new();
    let options: Vec<_> = options
        .into_iter()
        .filter(|choice| !choice.value.is_empty() && seen.insert(choice.value.clone()))
        .collect();
    let value = value
        .filter(|value| options.iter().any(|choice| &choice.value == value))
        .unwrap_or_default();
    let description_id = description
        .as_ref()
        .filter(|text| !text.is_empty())
        .map(|_| format!("{id}-desc"));
    let severity = status.as_ref().map(|status| status.status_type);
    let status_id = status
        .as_ref()
        .filter(|status| status.message.as_ref().is_some_and(|text| !text.is_empty()))
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
    let allowed = options
        .iter()
        .filter(|choice| !choice.is_disabled)
        .map(|choice| choice.value.clone())
        .collect::<Vec<_>>();
    rsx! {
        Field { label, input_id: id.clone(), description, description_id, width, is_label_hidden, is_required, is_disabled, status, status_variant,
            div { class: "astryx-selector__entry", "data-loading": is_loading.then_some("true"),
            select { id, class: "astryx-selector__control", name: html_name, required: is_required, disabled: is_disabled,
                "aria-describedby": (!described_by.is_empty()).then_some(described_by),
                "aria-busy": is_loading.then_some("true"),
                "aria-invalid": (severity == Some(InputStatusType::Error)).then_some("true"),
                "data-status": severity.map(InputStatusType::as_str), "data-astryx-selector-value": value.clone(),
                onmounted: move |_| {
                    if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                    let eval = document::eval(include_str!("sync_selector.js"));
                    let _ = eval.send(mount_id.clone());
                    controller.set(Some(eval));
                },
                onchange: move |event| {
                    let next = event.value();
                    if !is_disabled && allowed.contains(&next) { on_change.call(next); }
                },
                option { value: "", disabled: true, selected: value.is_empty(), "{placeholder}" }
                for choice in options {
                    option { key: "{choice.value}", selected: choice.value == value, value: choice.value,
                        disabled: choice.is_disabled, "{choice.label}" }
                }
            }
            if is_loading { Spinner { size: SpinnerSize::Small, is_decorative: true } }
            }
        }
    }
}
