use dioxus::prelude::*;
use dioxus_ssr::{render, render_element};

use astryx_dioxus::{
    CheckboxIndicatorState, CheckboxList, CheckboxListItem, FieldStatusInput, FormLayout,
    FormOptionality, InputStatusType, ListDensity,
};

fn attribute<'a>(html: &'a str, name: &str) -> Option<&'a str> {
    html.split_once(&format!(" {name}=\""))?.1.split('"').next()
}

#[test]
fn group_references_its_label_and_both_descriptions() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            CheckboxList { label: "Notifications", description: "Choose channels", density: ListDensity::Compact,
                "aria-describedby": "external", status: FieldStatusInput { status_type: InputStatusType::Warning,
                    message: Some("Review charges".into()), message_id: Some("charges".into()), },
                li { "Email" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let labelled = attribute(&html, "aria-labelledby").unwrap();
    let described = attribute(&html, "aria-describedby").unwrap();
    let ids: Vec<_> = described.split_whitespace().collect();
    assert_eq!(ids.len(), 3, "{html}");
    assert_eq!((ids[0], ids[2]), ("external", "charges"));
    for id in [labelled, ids[1], ids[2]] {
        assert!(html.contains(&format!("id=\"{id}\"")), "{html}");
    }
    assert!(
        html.contains("role=\"group\"") && html.contains("data-density=\"compact\""),
        "{html}"
    );
}

#[test]
fn detached_status_uses_one_resolvable_reference_even_with_an_invalid_id() {
    for invalid_id in ["", " \t", "two tokens"] {
        let html = render_element(rsx! {
            p { id: "caller-name", "Channels" }
            p { id: "caller-description", "Contact settings" }
            CheckboxList { id: "notifications", label: "Notifications", is_label_hidden: true,
                aria_labelledby: "caller-name", aria_describedby: "caller-description",
                status: FieldStatusInput { status_type: InputStatusType::Warning,
                    message: Some("Review charges".into()), message_id: Some(invalid_id.into()), },
                CheckboxListItem { label: "Email", is_checked: CheckboxIndicatorState::Checked }
            }
        });
        let (before_role, after_role) = html.split_once(" role=\"group\"").unwrap();
        let (before_group, group_prefix) = before_role.rsplit_once('<').unwrap();
        let group = format!("{group_prefix}{}", after_role.split('>').next().unwrap());
        assert_eq!(attribute(&group, "id"), Some("notifications"), "{html}");
        assert!(!group.contains("tabindex="), "{html}");
        let labels = attribute(&group, "aria-labelledby").unwrap();
        let (label_id, caller_name) = labels.split_once(' ').unwrap();
        assert!(
            before_group.contains(&format!("id=\"{label_id}\"")),
            "{html}"
        );
        assert_eq!(caller_name, "caller-name", "{html}");
        let described_by = attribute(&group, "aria-describedby").unwrap();
        let (caller_description, status_id) = described_by.split_once(' ').unwrap();
        assert_eq!(caller_description, "caller-description", "{html}");
        assert_eq!(status_id, label_id.replace("-label", "-status"), "{html}");
        let (list, after_list) = html.split_once("</ul>").unwrap();
        assert!(
            list.contains("type=\"checkbox\"") && list.contains("data-checked=\"true\""),
            "{html}"
        );
        assert!(
            after_list.contains(&format!("id=\"{status_id}\"")),
            "{html}"
        );
        assert!(
            after_list.contains("data-variant=\"detached\"")
                && after_list.contains("Review charges"),
            "{html}"
        );
    }
}

#[test]
fn status_without_a_message_does_not_leave_a_description_reference() {
    for message in [None, Some(String::new())] {
        let html = render_element(rsx! {
            CheckboxList { label: "Notifications", status: FieldStatusInput {
                status_type: InputStatusType::Warning, message, message_id: Some("charges".into()),
            },
                CheckboxListItem { label: "Email" }
            }
        });
        assert!(!html.contains("aria-describedby="), "{html}");
        assert!(
            !html.contains("charges") && !html.contains("data-variant=\"detached\""),
            "{html}"
        );
    }
}

#[test]
fn form_optionality_marks_the_group_label_without_requiring_rows() {
    for (default_optionality, is_optional, has_marker) in [
        (FormOptionality::Required, false, false),
        (FormOptionality::Required, true, true),
        (FormOptionality::Optional, true, false),
    ] {
        let html = render_element(rsx! {
            FormLayout { default_optionality,
                CheckboxList { label: "Notifications", is_optional,
                    CheckboxListItem { label: "Email" }
                }
            }
        });
        assert_eq!(html.contains("field-label__marker"), has_marker, "{html}");
        assert!(html.contains("type=\"checkbox\""), "{html}");
        assert!(
            !html.contains(" required") && !html.contains(" aria-required"),
            "{html}"
        );
    }
}
