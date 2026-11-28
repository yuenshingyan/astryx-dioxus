use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    Field, FieldStatusInput, FieldStatusPlacement, FormLayout, FormLayoutDirection,
    FormOptionality, InputStatusType,
};

#[test]
fn field_composes_label_description_and_detached_status() {
    let html = render_element(rsx! {
        Field { label: "Email", input_id: "email", description: "Work address", width: Some("320px".into()),
            status_variant: FieldStatusPlacement::Detached,
            status: FieldStatusInput { status_type: InputStatusType::Error, message: Some("Invalid address".into()), message_id: None },
            input { id: "email", "aria-describedby": "email-desc email-status" }
        }
    });
    for expected in [
        "for=\"email\"",
        "id=\"email-desc\"",
        "id=\"email-status\"",
        "data-variant=\"detached\"",
        "width:320px",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}

#[test]
fn horizontal_field_keeps_group_relations_and_explicit_ids_in_the_control_column() {
    let html = render_element(rsx! {
        FormLayout { direction: FormLayoutDirection::HorizontalLabels,
            Field { label: "Notifications", input_id: "preferences", is_group_label: true,
                label_id: "preferences-name", description: "Choose a frequency", description_id: "preferences-help",
                status_variant: FieldStatusPlacement::Detached,
                status: FieldStatusInput { status_type: InputStatusType::Warning, message: Some("May take a minute".into()), message_id: Some("preferences-warning".into()) },
                class: "caller-class", style: "color:inherit", "data-purpose": "preferences", "data-layout": "caller-layout",
                div { id: "preferences", role: "radiogroup", "aria-labelledby": "preferences-name",
                    "aria-describedby": "preferences-help preferences-warning",
                }
            }
        }
    });
    let (label_column, control_column) = html.split_once("astryx-field__control-status").unwrap();
    assert!(label_column.contains("astryx-field caller-class"), "{html}");
    assert!(label_column.contains("style=\"color:inherit\""), "{html}");
    assert!(
        label_column.contains("data-purpose=\"preferences\""),
        "{html}"
    );
    assert!(
        label_column.contains("data-layout=\"horizontal-labels\""),
        "{html}"
    );
    assert!(!html.contains("caller-layout"), "{html}");
    assert!(
        label_column.contains("id=\"preferences-name\">Notifications</span>"),
        "{html}"
    );
    assert!(
        !html.contains("<label") && !html.contains(" for="),
        "{html}"
    );
    assert!(!label_column.contains("Choose a frequency"), "{html}");
    for expected in [
        "astryx-text--block",
        "data-type=\"supporting\"",
        "id=\"preferences-help\"",
        "aria-labelledby=\"preferences-name\"",
        "aria-describedby=\"preferences-help preferences-warning\"",
        "data-type=\"warning\"",
        "id=\"preferences-warning\"",
    ] {
        assert!(
            control_column.contains(expected),
            "missing {expected}: {html}"
        );
    }
    assert!(
        !html.contains("preferences-desc") && !html.contains("preferences-status"),
        "{html}"
    );
    assert_eq!(html.matches("id=\"preferences-help\"").count(), 1, "{html}");
    assert_eq!(
        html.matches("id=\"preferences-warning\"").count(),
        1,
        "{html}"
    );
    assert!(
        control_column.find("Choose a frequency") < control_column.find("role=\"radiogroup\""),
        "{html}"
    );
    assert!(
        control_column.find("role=\"radiogroup\"") < control_column.find("May take a minute"),
        "{html}"
    );
}

#[test]
fn field_optionality_is_label_only_even_when_form_defaults_suppress_the_marker() {
    for (default_optionality, expected) in [
        (None, "Optional"),
        (Some(FormOptionality::Required), "Optional"),
        (Some(FormOptionality::Optional), "Required"),
    ] {
        let field = rsx! {
            Field { label: "Display name", input_id: "display-name", is_required: true, is_optional: true, is_disabled: true,
                input { id: "display-name", "aria-label": "Public name", "aria-required": "false" }
            }
        };
        let html = match default_optionality {
            Some(default) => {
                render_element(rsx! { FormLayout { default_optionality: default, {field} } })
            }
            None => render_element(field),
        };
        let (label, control) = html.split_once("<input").unwrap();
        assert!(label.contains("data-disabled=\"true\""), "{html}");
        assert!(label.contains(&format!("{expected}</span>")), "{html}");
        assert_eq!(
            label.matches("astryx-field-label__marker").count(),
            1,
            "{html}"
        );
        assert!(control.contains("aria-label=\"Public name\""), "{html}");
        assert!(control.contains("aria-required=\"false\""), "{html}");
        assert!(
            !control.contains(" required") && !control.contains(" disabled"),
            "{html}"
        );
    }
}

#[test]
fn message_severity_does_not_set_aria_invalid_on_the_control() {
    for (status_type, severity) in [
        (InputStatusType::Error, "error"),
        (InputStatusType::Warning, "warning"),
        (InputStatusType::Success, "success"),
    ] {
        let html = render_element(rsx! {
            Field { label: "Address", input_id: "address",
                status: FieldStatusInput { status_type, message: Some("Check this address".into()), message_id: None },
                input { id: "address", "aria-describedby": "address-status" }
            }
        });
        assert!(
            html.contains(&format!("data-type=\"{severity}\"")),
            "{html}"
        );
        assert!(html.contains("data-variant=\"attached\""), "{html}");
        assert!(
            !html.contains("aria-invalid") && !html.contains("aria-live"),
            "{html}"
        );
        assert_eq!(html.matches("id=\"address-status\"").count(), 1, "{html}");
        assert!(html.find("</label>") < html.find("<input"), "{html}");
        assert!(
            html.find("<input") < html.find("Check this address"),
            "{html}"
        );
    }
}

#[test]
fn empty_description_or_status_has_no_target_even_with_an_explicit_id() {
    for status in [
        None,
        Some(FieldStatusInput {
            status_type: InputStatusType::Success,
            message: None,
            message_id: Some("feedback".into()),
        }),
        Some(FieldStatusInput {
            status_type: InputStatusType::Success,
            message: Some(String::new()),
            message_id: Some("feedback".into()),
        }),
    ] {
        let html = render_element(rsx! {
            Field { label: "Search", input_id: "search", description: "", description_id: "help",
                status,
                input { id: "search" }
            }
        });
        assert!(!html.contains("id=\"help\""), "{html}");
        assert!(!html.contains("id=\"feedback\""), "{html}");
        assert!(!html.contains("astryx-field-status"), "{html}");
        assert!(
            !html.contains("aria-invalid") && !html.contains("aria-describedby"),
            "{html}"
        );
    }
}

#[test]
fn horizontal_labels_move_description_into_the_control_column() {
    let html = render_element(rsx! {
        FormLayout { direction: FormLayoutDirection::HorizontalLabels,
            Field { label: "Email", input_id: "email", is_label_hidden: true, description: "Work address", width: Some("320px".into()),
                input { id: "email", "aria-describedby": "email-desc" }
            }
        }
    });
    assert!(
        html.contains("data-layout=\"horizontal-labels\"") && !html.contains("width:320px"),
        "{html}"
    );
    let control = html
        .split_once("astryx-field__control-status")
        .expect("control column")
        .1;
    assert!(
        control.contains("id=\"email-desc\"") && control.contains("clip:rect(0,0,0,0)"),
        "{html}"
    );
}
