use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    Field, FieldStatusInput, FieldStatusPlacement, FormLayout, FormLayoutDirection, InputStatusType,
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
