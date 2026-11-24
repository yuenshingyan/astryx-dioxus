use std::collections::HashSet;

use dioxus::core::current_scope_id;
use dioxus::prelude::*;

use crate::{Field, SizeValue};

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
    #[props(default)] html_name: Option<String>,
    #[props(default)] width: Option<SizeValue>,
    #[props(default)] is_disabled: bool,
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
    let described_by = description_id.clone();
    let allowed = options
        .iter()
        .filter(|choice| !choice.is_disabled)
        .map(|choice| choice.value.clone())
        .collect::<Vec<_>>();
    rsx! {
        Field { label, input_id: id.clone(), description, description_id, width, is_label_hidden, is_required, is_disabled,
            select { id, class: "astryx-selector__control", name: html_name, required: is_required, disabled: is_disabled,
                "aria-describedby": described_by, "data-astryx-selector-value": value.clone(),
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
        }
    }
}
