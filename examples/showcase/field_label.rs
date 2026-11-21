use dioxus::prelude::*;

use astryx_dioxus::{FieldLabel, FormLayout, FormOptionality};

#[component]
fn ExampleLabels(prefix: &'static str) -> Element {
    let email = format!("{prefix}-email");
    let phone = format!("{prefix}-phone");
    let description_id = format!("{prefix}-phone-description");
    rsx! {
        div {
            FieldLabel { label: "Email address", input_id: email.clone(), is_required: true }
            input { id: email, r#type: "email", required: true }
        }
        div {
            FieldLabel { label: "Phone number", input_id: phone.clone(), is_optional: true,
                description: rsx! { "Include country code for international numbers" }, description_id: description_id.clone(),
            }
            input { id: phone, r#type: "tel", "aria-describedby": description_id }
        }
    }
}

#[component]
pub(super) fn FieldLabelExamples() -> Element {
    rsx! {
        section { id: "field-label-examples",
            h2 { "FieldLabel" }
            h3 { "Basic" }
            FormLayout { ExampleLabels { prefix: "field-label-basic" } }
            h3 { "Showcase (without InfoTip)" }
            FormLayout {
                ExampleLabels { prefix: "field-label-showcase" }
                div {
                    FieldLabel { label: "API key", input_id: "field-label-api-key", label_icon: rsx! {
                        svg { width: "16", height: "16", view_box: "0 0 24 24", stroke: "currentColor", fill: "none", "aria-hidden": "true",
                            path { d: "M6 10h12v10H6zM8 10V7a4 4 0 0 1 8 0v3" }
                        }
                    } }
                    input { id: "field-label-api-key" }
                }
                div {
                    FieldLabel { label: "Disabled field", input_id: "field-label-disabled", is_disabled: true }
                    input { id: "field-label-disabled", disabled: true }
                }
            }
            h3 { "Label arrangements" }
            FormLayout { default_optionality: FormOptionality::Required,
                div {
                    FieldLabel { label: "Search", input_id: "field-label-search", is_label_hidden: true, is_required: true,
                        description: rsx! { "Search the catalog" }, description_id: "field-label-search-description",
                    }
                    input { id: "field-label-search", r#type: "search", "aria-describedby": "field-label-search-description" }
                }
                div { role: "group", "aria-labelledby": "field-label-notifications",
                    FieldLabel { label: "Notifications", input_id: "", label_id: "field-label-notifications", is_group_label: true, is_optional: true }
                    label { input { r#type: "checkbox" } "Email alerts" }
                }
                div {
                    FieldLabel { label: "Digest", input_id: "field-label-digest",
                        description_id: "field-label-digest-description", description: rsx! { "Receive a weekly digest. " a { href: "#field-label-examples", "Learn more" } },
                    }
                    input { id: "field-label-digest", r#type: "checkbox", "aria-describedby": "field-label-digest-description" }
                }
            }
        }
    }
}
