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

#[test]
fn form_default_filters_conflicting_markers_before_optional_takes_precedence() {
    for (default_optionality, expected_optional, expected_required) in [
        (None, 1, 0),
        (Some(FormOptionality::Required), 1, 0),
        (Some(FormOptionality::Optional), 0, 1),
    ] {
        let label = rsx! { FieldLabel {
            label: "Email", input_id: "email", is_optional: true, is_required: true,
            optional_text: "Facultatif", required_text: "Obligatoire",
        } };
        let html = match default_optionality {
            Some(default) => render_element(rsx! { FormLayout { default_optionality: default,
                {label}
            } }),
            None => render_element(label),
        };
        assert_eq!(
            html.matches("Facultatif").count(),
            expected_optional,
            "{default_optionality:?}: {html}"
        );
        assert_eq!(
            html.matches("Obligatoire").count(),
            expected_required,
            "{default_optionality:?}: {html}"
        );
        assert!(
            !html.contains("Optional") && !html.contains("Required"),
            "{html}"
        );
    }
}
