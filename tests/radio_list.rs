use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{
    FieldStatusInput, FormLayout, FormOptionality, InputStatusType, RadioList, RadioListOrientation,
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
