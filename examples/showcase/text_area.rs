use dioxus::prelude::*;

use astryx_dioxus::{FieldStatusInput, FieldStatusPlacement, InputStatusType, TextArea};

#[component]
pub(super) fn TextAreaExamples() -> Element {
    let mut description = use_signal(|| "Initial description".to_owned());
    rsx! {
        section { id: "text-area-examples",
            h2 { "TextArea" }
            h3 { "Showcase" }
            TextArea { id: "description-area", label: "Description", value: description(), rows: 3,
                placeholder: "Enter a description...", width: Some("320px".into()),
                on_change: move |value| description.set(value),
            }
            output { id: "description-preview", "{description}" }
            h3 { "States" }
            form { id: "text-area-form",
                TextArea { id: "disabled-area", label: "Disabled field", value: "Unavailable", is_disabled: true, html_name: "disabled" }
                TextArea { id: "read-only-area", label: "Read-only field", value: "Reference notes", is_read_only: true, html_name: "reference" }
            }
            h3 { "Validation" }
            TextArea { id: "validation-area", label: "Error message", value: "", description: "Explain the issue.", is_required: true,
                status_variant: FieldStatusPlacement::Detached,
                status: FieldStatusInput { status_type: InputStatusType::Error, message: Some("Enter a description.".into()), message_id: None },
            }
        }
    }
}
