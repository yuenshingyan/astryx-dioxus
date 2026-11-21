use dioxus::prelude::*;

use astryx_dioxus::{FormLayout, FormLayoutDirection};

#[component]
fn NativeField(label: &'static str, #[props(default = "text")] kind: &'static str) -> Element {
    rsx! { label { style: "display:grid;gap:var(--spacing-1);min-width:0",
        "{label}"
        input { r#type: kind, style: "min-width:0" }
    } }
}

#[component]
fn AddressFields() -> Element {
    rsx! {
        FormLayout {
            FormLayout { direction: FormLayoutDirection::Horizontal,
                NativeField { label: "First Name" }
                NativeField { label: "Last Name" }
            }
            NativeField { label: "Email", kind: "email" }
            FormLayout { direction: FormLayoutDirection::Horizontal,
                NativeField { label: "City" }
                NativeField { label: "State" }
                NativeField { label: "ZIP" }
            }
        }
    }
}

#[component]
pub(super) fn FormLayoutExamples() -> Element {
    rsx! {
        section { id: "form-layout-examples", style: "max-width:400px",
            h2 { "FormLayout" }
            h3 { "Showcase" }
            AddressFields {}
            h3 { "Horizontal" }
            FormLayout { id: "form-layout-horizontal", direction: FormLayoutDirection::Horizontal,
                NativeField { label: "First Name" }
                NativeField { label: "Last Name" }
            }
            h3 { "Horizontal labels" }
            FormLayout { id: "form-layout-horizontal-labels", direction: FormLayoutDirection::HorizontalLabels,
                label { r#for: "form-layout-display-name", "Display Name" }
                input { id: "form-layout-display-name" }
                label { r#for: "form-layout-email", "Email" }
                input { id: "form-layout-email", r#type: "email" }
                label { r#for: "form-layout-timezone", "Timezone" }
                select { id: "form-layout-timezone", option { "Pacific Time" } option { "Eastern Time" } option { "UTC" } }
                label { r#for: "form-layout-bio", "Bio" }
                textarea { id: "form-layout-bio", rows: "3" }
            }
            h3 { "Nested" }
            AddressFields {}
            h3 { "Mixed controls" }
            FormLayout {
                NativeField { label: "Full Name" }
                label { "Role " select { option { "Viewer" } option { "Editor" } option { "Admin" } } }
                fieldset { legend { "Notifications" }
                    for label in ["Email", "SMS", "Push"] {
                        label { key: "{label}", input { r#type: "checkbox", initial_checked: label != "SMS" }, "{label}" }
                    }
                }
            }
        }
    }
}
