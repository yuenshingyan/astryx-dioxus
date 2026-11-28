use std::collections::HashSet;

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{
    Field, FieldStatusInput, FieldStatusPlacement, InputStatusType, SelectorChoice, SizeValue,
};

/// A controlled multiple-choice field backed by a native listbox.
#[component]
pub fn MultiSelector(
    label: String,
    options: Vec<SelectorChoice>,
    value: Vec<String>,
    on_change: EventHandler<Vec<String>>,
    #[props(default)] html_name: Option<String>,
    /// Help text associated with the listbox.
    #[props(default)]
    description: Option<String>,
    /// Caller-owned validation feedback, independent of native validity.
    #[props(default)]
    status: Option<FieldStatusInput>,
    /// Placement of the validation message.
    #[props(default)]
    status_variant: FieldStatusPlacement,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] is_required: bool,
    #[props(default)] is_disabled: bool,
) -> Element {
    let id = use_hook(|| format!("astryx-multi-selector-{}", current_scope_id().0));
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
    let previous = value.clone();
    let report = use_callback(move |next: Vec<String>| {
        if is_disabled {
            return;
        }
        // Preserve the caller's selection order, then append newly chosen values.
        let mut ordered: Vec<_> = previous
            .iter()
            .filter(|choice| next.contains(choice))
            .cloned()
            .collect();
        for choice in next {
            if !ordered.contains(&choice) {
                ordered.push(choice);
            }
        }
        on_change.call(ordered);
    });
    rsx! {
        Field { label, input_id: id.clone(), width, is_required, is_disabled, description, description_id, status, status_variant,
            select { id, class: "astryx-selector__control astryx-multi-selector__control", name: html_name,
                multiple: true, required: is_required, disabled: is_disabled,
                "aria-describedby": (!described_by.is_empty()).then_some(described_by),
                "aria-invalid": (severity == Some(InputStatusType::Error)).then_some("true"),
                "data-status": severity.map(InputStatusType::as_str),
                onmounted: move |_| {
                    if let Some(eval) = *controller.peek() { let _ = eval.send(()); }
                    let mut eval = document::eval(include_str!("sync_multi_selector.js"));
                    let _ = eval.send(mount_id.clone());
                    controller.set(Some(eval));
                    spawn(async move {
                        while let Ok(next) = eval.recv::<Vec<String>>().await { report.call(next); }
                    });
                },
                for choice in options {
                    option { key: "{choice.value}", value: choice.value.clone(), disabled: choice.is_disabled,
                        selected: value.contains(&choice.value),
                        "data-astryx-selected": value.contains(&choice.value).to_string(), "{choice.label}" }
                }
            }
        }
    }
}
