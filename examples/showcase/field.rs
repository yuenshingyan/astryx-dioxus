use dioxus::prelude::*;

use astryx_dioxus::{
    Field, FieldStatusInput, FieldStatusPlacement, FormLayout, FormLayoutDirection, InputStatusType,
};

fn status(status_type: InputStatusType, message: &str) -> FieldStatusInput {
    FieldStatusInput {
        status_type,
        message: Some(message.into()),
        message_id: None,
    }
}

const CONTROL: &str = "background:var(--color-background-surface);color:var(--color-text-primary);border:1px solid var(--color-border-emphasized);border-radius:var(--radius-element);padding:4px 8px;min-width:0;font:inherit";

#[component]
pub(super) fn FieldExamples() -> Element {
    let mut email = use_signal(String::new);
    let has_error = !email().is_empty() && !email().contains('@');
    rsx! {
        section { id: "field-examples", "data-astryx-theme": "neutral",
            h2 { "Field" }
            h3 { "Showcase" }
            Field { label: "Email", input_id: "field-email", width: Some("320px".into()), description: "We will never share your email.", is_required: true,
                status: has_error.then(|| status(InputStatusType::Error, "Enter a valid email address.")),
                input { id: "field-email", style: CONTROL, r#type: "email", placeholder: "you@example.com", required: true,
                    "aria-invalid": has_error.then_some("true"),
                    "aria-describedby": if has_error { "field-email-desc field-email-status" } else { "field-email-desc" },
                    oninput: move |event| email.set(event.value()),
                }
            }
            h3 { "With description" }
            FormLayout { direction: FormLayoutDirection::HorizontalLabels,
                Field { label: "Email", input_id: "field-contact", description: "We'll send a confirmation link to this address",
                    input { id: "field-contact", style: CONTROL, "aria-describedby": "field-contact-desc", placeholder: "you@example.com" }
                }
                Field { label: "Password", input_id: "field-password", description: "At least 8 characters with one uppercase letter",
                    label_tooltip: "Use a password you do not use for another account.", label_tooltip_name: "About password requirements",
                    input { id: "field-password", style: CONTROL, r#type: "password", "aria-describedby": "field-password-desc", placeholder: "Create a password" }
                }
            }
            h3 { "Required and optional" }
            FormLayout {
                Field { label: "Username", input_id: "field-username", is_required: true,
                    input { id: "field-username", style: CONTROL, required: true, placeholder: "Enter your username" }
                }
                Field { label: "Backup email", input_id: "field-backup", is_optional: true,
                    input { id: "field-backup", style: CONTROL, r#type: "email", placeholder: "you@example.com" }
                }
            }
            h3 { "Wrap a custom control" }
            Field { label: "Volume", input_id: "field-volume", width: Some("320px".into()),
                description: "Adjust the playback volume.", description_id: "field-volume-help",
                status_variant: FieldStatusPlacement::Detached,
                status: FieldStatusInput {
                    status_type: InputStatusType::Success,
                    message: Some("Volume can be adjusted.".into()),
                    message_id: Some("field-volume-feedback".into()),
                },
                input { id: "field-volume", r#type: "range", min: "0", max: "100",
                    "aria-describedby": "field-volume-help field-volume-feedback",
                }
            }
            h3 { "Status variants" }
            FormLayout {
                for (id, label, description, severity, message, placement) in [
                    ("field-work-email", "Email", "Enter your work email", InputStatusType::Error, "This email address is already in use", FieldStatusPlacement::Attached),
                    ("field-handle", "Username", "Choose a unique username", InputStatusType::Warning, "This username is similar to an existing one", FieldStatusPlacement::Detached),
                    ("field-key", "API Key", "Paste your API key", InputStatusType::Success, "API key is valid and active", FieldStatusPlacement::Attached),
                ] {
                    Field { key: "{id}", label, input_id: id, description, status: status(severity, message), status_variant: placement,
                        input { id, style: CONTROL, "aria-describedby": "{id}-desc {id}-status" }
                    }
                }
            }
        }
    }
}
