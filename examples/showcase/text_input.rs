use dioxus::prelude::*;

use astryx_dioxus::{
    FieldStatusInput, FieldStatusPlacement, InputStatusType, TextInput, TextInputType,
};

#[component]
pub(super) fn TextInputExamples() -> Element {
    let mut name = use_signal(String::new);
    let mut search = use_signal(String::new);
    let mut filter = use_signal(|| "Design systems".to_owned());
    rsx! {
        section { id: "text-input-examples",
            h2 { "TextInput" }
            h3 { "Showcase" }
            TextInput { id: "name-input", label: "Name", value: name(), placeholder: "Enter your name", width: Some("320px".into()), on_change: move |value| name.set(value) }
            output { id: "name-input-preview", "{name}" }
            h3 { "Types" }
            TextInput { label: "Password field", value: "", input_type: TextInputType::Password, autocomplete: "new-password" }
            TextInput { id: "email-input", label: "Email field", value: "example", input_type: TextInputType::Email, description: "Use a work email.",
                status: FieldStatusInput { status_type: InputStatusType::Error, message: Some("Enter a valid email address.".into()), message_id: None },
                status_variant: FieldStatusPlacement::Detached,
            }
            h3 { "Search" }
            TextInput { id: "project-search", label: "Search field", value: search(), input_type: TextInputType::Search,
                placeholder: "Search projects…", has_clear: true, on_change: move |value| search.set(value) }
            TextInput { id: "project-filter", label: "Search field with value", value: filter(), input_type: TextInputType::Search,
                placeholder: "Filter…", has_clear: true, on_change: move |value| filter.set(value) }
            h3 { "States" }
            form { id: "text-input-form",
                TextInput { id: "disabled-text-input", label: "Disabled field", value: "Unavailable", is_disabled: true, has_clear: true, html_name: "disabled" }
                TextInput { label: "Read-only field", value: "Reference", is_read_only: true, has_clear: true, html_name: "reference" }
            }
        }
    }
}
