use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{CheckboxList, FieldStatusInput, InputStatusType, ListDensity};

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
    let labelled = html
        .split("aria-labelledby=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    let described = html
        .split("aria-describedby=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
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
