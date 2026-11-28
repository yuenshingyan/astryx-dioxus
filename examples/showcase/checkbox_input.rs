use dioxus::prelude::*;

use astryx_dioxus::{CheckboxIndicatorState, CheckboxInput, FieldStatusInput, InputStatusType};

fn state(checked: bool) -> CheckboxIndicatorState {
    if checked {
        CheckboxIndicatorState::Checked
    } else {
        CheckboxIndicatorState::Unchecked
    }
}

#[component]
pub(super) fn CheckboxInputExamples() -> Element {
    let mut choices = use_signal(|| [true, false, false, false]);
    let mut enabled = use_signal(|| false);
    let mut is_loading = use_signal(|| false);
    let mut backup = use_signal(|| false);
    let count = choices.read().iter().filter(|&&checked| checked).count();
    let all = match count {
        0 => CheckboxIndicatorState::Unchecked,
        4 => CheckboxIndicatorState::Checked,
        _ => CheckboxIndicatorState::Indeterminate,
    };
    rsx! {
        section { id: "checkbox-input-examples",
            h2 { "CheckboxInput" }
            h3 { "States" }
            CheckboxInput { label: "Checked", value: CheckboxIndicatorState::Checked, description: "This checkbox is currently on." }
            CheckboxInput { id: "checkbox-unchecked", label: "Unchecked", value: CheckboxIndicatorState::Unchecked, description: "This checkbox is currently off." }
            form { id: "checkbox-preview-form",
                CheckboxInput { id: "checkbox-disabled", label: "Disabled", value: CheckboxIndicatorState::Unchecked,
                    is_disabled: true, is_required: true, html_name: "disabled-choice", description: "This checkbox cannot be changed." }
            }
            CheckboxInput { label: "Indeterminate", value: CheckboxIndicatorState::Indeterminate, description: "This checkbox represents a partial selection." }
            h3 { "Showcase" }
            CheckboxInput { label: "Enable alerts", value: state(enabled()), on_change: move |checked| enabled.set(checked) }
            h3 { "Loading" }
            CheckboxInput { id: "checkbox-loading", label: "Include backups", value: state(backup()),
                is_loading: is_loading(), on_change: move |checked| backup.set(checked),
            }
            button { id: "checkbox-toggle-loading", r#type: "button", onclick: move |_| is_loading.toggle(), "Toggle simulated loading" }
            h3 { "Indeterminate" }
            CheckboxInput { id: "checkbox-select-all", label: "Select all notifications", value: all,
                description: format!("{count} of 4 enabled"), on_change: move |checked| choices.set([checked; 4]),
            }
            for (index, label) in ["Email notifications", "Push notifications", "SMS alerts", "Slack messages"].into_iter().enumerate() {
                CheckboxInput { key: "{label}", label, value: state(choices.read()[index]), on_change: move |checked| choices.write()[index] = checked }
            }
            h3 { "Status" }
            for (label, status_type, message) in [("Error", InputStatusType::Error, "Accept this field to continue."), ("Warning", InputStatusType::Warning, "Review the implications of this setting."), ("Success", InputStatusType::Success, "This setting has been verified.")] {
                CheckboxInput { key: "{label}", label, value: CheckboxIndicatorState::Unchecked,
                    status: FieldStatusInput { status_type, message: Some(message.to_owned()), message_id: None },
                }
            }
        }
    }
}
