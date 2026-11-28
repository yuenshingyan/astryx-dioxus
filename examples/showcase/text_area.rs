use dioxus::prelude::*;

use astryx_dioxus::{FieldStatusInput, FieldStatusPlacement, InputStatusType, TextArea};

#[component]
pub(super) fn TextAreaExamples() -> Element {
    let mut description = use_signal(|| "Initial description".to_owned());
    let mut last_event = use_signal(|| "");
    let mut is_loading = use_signal(|| false);
    rsx! {
        section { id: "text-area-examples",
            h2 { "TextArea" }
            h3 { "Showcase" }
            TextArea { id: "description-area", label: "Description", value: description(), rows: 3,
                placeholder: "Enter a description...", width: Some("320px".into()),
                autocomplete: "off", has_spell_check: false, is_loading: is_loading(),
                onfocus: move |_| last_event.set("Focused"),
                onblur: move |_| last_event.set("Blurred"),
                onpaste: move |_| last_event.set("Paste requested"),
                on_change: move |value| description.set(value),
            }
            button { id: "text-area-toggle-loading", r#type: "button", onclick: move |_| is_loading.toggle(), "Toggle simulated loading" }
            output { id: "description-preview", "{description}" }
            p { id: "text-area-last-event", "{last_event}" }
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
