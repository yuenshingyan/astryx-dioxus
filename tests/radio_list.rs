use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{
    FieldStatusInput, FormLayout, FormOptionality, InputStatusType, RadioList, RadioListItem,
    RadioListOrientation,
};

#[test]
fn group_links_name_and_descriptions_without_imposing_native_validation() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            FormLayout { default_optionality: FormOptionality::Required,
                RadioList { label: "Delivery", orientation: RadioListOrientation::Horizontal, description: "Choose one option",
                    "aria-labelledby": "extra-name", "aria-describedby": "extra-description",
                    status: FieldStatusInput { status_type: InputStatusType::Error, message: Some("Choose a method".into()), message_id: Some("delivery-error".into()) },
                    input { r#type: "radio", name: "delivery" }
                }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let labelled_by = html
        .split("aria-labelledby=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    let mut ids = labelled_by.split_whitespace();
    assert!(
        html.contains(&format!("id=\"{}\"", ids.next().unwrap())),
        "{html}"
    );
    assert_eq!(ids.next(), Some("extra-name"));
    for expected in [
        "role=\"radiogroup\"",
        "data-orientation=\"horizontal\"",
        "aria-required=\"true\"",
        "aria-invalid=\"true\"",
        "aria-describedby=\"extra-description ",
        " delivery-error\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains(" required"), "{html}");
}

#[test]
fn group_disabled_state_takes_precedence_over_caller_aria_disabled() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            RadioList { label: "Default", html_name: "default-option",
                RadioListItem { label: "Email", value: "email" }
            }
            RadioList { label: "Caller true", html_name: "aria-only", value: "email", "aria-disabled": true,
                RadioListItem { label: "Email", value: "email" }
            }
            RadioList { label: "Caller false", "aria-disabled": "false",
                input { r#type: "radio", name: "caller-option" }
            }
            RadioList { label: "Disabled", html_name: "blocked-option", is_disabled: true, "aria-disabled": "false",
                RadioListItem { label: "Email", value: "email" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let groups: Vec<_> = html.split("role=\"radiogroup\"").skip(1).collect();
    assert_eq!(groups.len(), 4, "{html}");
    for (group, expected) in groups
        .iter()
        .zip([None, Some("true"), Some("false"), Some("true")])
    {
        let opening_tag = group.split('>').next().unwrap();
        let mut values = opening_tag
            .split(" aria-disabled=\"")
            .skip(1)
            .map(|value| value.split('"').next().unwrap());
        assert_eq!(values.next(), expected, "{opening_tag}");
        assert_eq!(values.next(), None, "{opening_tag}");
    }

    for (group, name, is_disabled) in [
        (groups[0], "default-option", false),
        (groups[1], "aria-only", false),
        (groups[2], "caller-option", false),
        (groups[3], "blocked-option", true),
    ] {
        let (_, input) = group.split_once("<input ").unwrap();
        let (input, _) = input.split_once('>').unwrap();
        assert!(input.contains("type=\"radio\""), "{input}");
        assert!(input.contains(&format!("name=\"{name}\"")), "{input}");
        assert_eq!(input.contains(" disabled"), is_disabled, "{input}");
    }
}
