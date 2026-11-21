use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{FieldLabel, FormLayout, FormOptionality};

#[test]
fn native_label_keeps_description_outside_its_accessible_name() {
    let html = render_element(rsx! { FieldLabel {
        label: "Email", input_id: "email", label_id: "email-label", description_id: "email-help",
        description: rsx! { "Work address only" }, is_required: true,
    } });
    assert!(html.contains("id=\"email-label\" for=\"email\""), "{html}");
    assert!(html.contains("Required"));
    assert!(html.find("</label>") < html.find("Work address only"));
    assert!(html.contains("id=\"email-help\""));
}

#[test]
fn group_labels_reflect_the_form_default_without_setting_native_required() {
    let html = render_element(
        rsx! { FormLayout { default_optionality: FormOptionality::Required,
            FieldLabel { label: "Preferences", input_id: "unused", label_id: "preferences", is_group_label: true, is_required: true }
            FieldLabel { label: "Contact", input_id: "contact", is_optional: true }
        } },
    );
    assert!(
        html.contains("id=\"preferences\">Preferences</span>"),
        "{html}"
    );
    assert!(!html.contains("unused") && !html.contains("Required") && !html.contains("required="));
    assert!(html.contains("Optional"));
}
