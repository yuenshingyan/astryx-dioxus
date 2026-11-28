use dioxus::prelude::*;

use astryx_dioxus::{
    CheckboxIndicatorState, CheckboxList, CheckboxListItem, FormLayout, FormLayoutDirection,
    Selector, SelectorChoice, TextArea, TextInput,
};

#[component]
fn ExampleTextField(label: &'static str, initial: &'static str) -> Element {
    let mut value = use_signal(|| initial.to_owned());
    rsx! { TextInput { label, value: value(), on_change: move |next| value.set(next) } }
}

fn choices(options: &[(&str, &str)]) -> Vec<SelectorChoice> {
    options
        .iter()
        .map(|&(label, value)| SelectorChoice {
            label: label.into(),
            value: value.into(),
            is_disabled: false,
        })
        .collect()
}

#[component]
fn AddressFields(id: &'static str) -> Element {
    rsx! {
        FormLayout { id,
            FormLayout { direction: FormLayoutDirection::Horizontal,
                ExampleTextField { label: "First Name", initial: "Priya" }
                ExampleTextField { label: "Last Name", initial: "Sharma" }
            }
            ExampleTextField { label: "Email", initial: "priya.sharma@example.com" }
            FormLayout { direction: FormLayoutDirection::Horizontal,
                ExampleTextField { label: "City", initial: "San Francisco" }
                ExampleTextField { label: "State", initial: "CA" }
                ExampleTextField { label: "ZIP", initial: "94105" }
            }
        }
    }
}

#[component]
fn SettingsFields() -> Element {
    let mut timezone = use_signal(|| "America/Los_Angeles".to_owned());
    let mut bio = use_signal(String::new);
    rsx! {
        FormLayout { id: "form-layout-horizontal-labels", direction: FormLayoutDirection::HorizontalLabels,
            ExampleTextField { label: "Display Name", initial: "Jane Doe" }
            ExampleTextField { label: "Email", initial: "jane@example.com" }
            Selector {
                label: "Timezone", value: Some(timezone()), on_change: move |next| timezone.set(next),
                options: choices(&[("Pacific Time", "America/Los_Angeles"), ("Eastern Time", "America/New_York"), ("UTC", "UTC")]),
            }
            TextArea { label: "Bio", value: bio(), rows: 3, on_change: move |next| bio.set(next) }
        }
    }
}

#[component]
fn MixedFields() -> Element {
    let mut role = use_signal(|| "editor".to_owned());
    let mut notifications = use_signal(|| [true, false, true]);
    rsx! {
        FormLayout { id: "form-layout-mixed-controls",
            ExampleTextField { label: "Full Name", initial: "Maya Torres" }
            Selector {
                label: "Role", value: Some(role()), on_change: move |next| role.set(next),
                options: choices(&[("Viewer", "viewer"), ("Editor", "editor"), ("Admin", "admin")]),
            }
            CheckboxList { label: "Notifications",
                for (index, label) in ["Email", "SMS", "Push"].into_iter().enumerate() {
                    CheckboxListItem {
                        key: "{label}", label,
                        is_checked: if notifications()[index] { CheckboxIndicatorState::Checked } else { CheckboxIndicatorState::Unchecked },
                        on_check: move |checked| notifications.write()[index] = checked,
                    }
                }
            }
        }
    }
}

#[component]
pub(super) fn FormLayoutExamples() -> Element {
    rsx! {
        section { id: "form-layout-examples",
            h2 { "FormLayout" }
            AddressFields { id: "form-layout-showcase" }
            h3 { "FormLayout — Horizontal" }
            FormLayout {
                id: "form-layout-horizontal", direction: FormLayoutDirection::Horizontal,
                style: "width:100%;max-width:400px",
                ExampleTextField { label: "First Name", initial: "Jordan" }
                ExampleTextField { label: "Last Name", initial: "Rivera" }
            }
            h3 { "FormLayout — Settings Form" }
            SettingsFields {}
            h3 { "FormLayout — Mixed Controls" }
            MixedFields {}
            h3 { "FormLayout — Nested Address Form" }
            AddressFields { id: "form-layout-nested" }
        }
    }
}
