use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{
    Button, ElementSize, FieldStatusInput, InputGroup, InputStatusType, SizeProvider,
};

#[test]
fn group_owns_name_and_state_and_inherits_size() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            SizeProvider { size: ElementSize::Large,
                InputGroup { label: "Search", id: "search-row", description: "Workspace only", is_label_hidden: true,
                    is_disabled: true, "aria-label": "Not the group name",
                    status: FieldStatusInput { status_type: InputStatusType::Warning, message: Some("Incomplete".into()), message_id: Some("notice".into()) },
                    Button { label: "Search" }
                    InputGroup { label: "More", Button { label: "More" } }
                }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "id=\"search-row\"",
        "role=\"group\"",
        "aria-labelledby=",
        "aria-describedby=",
        " notice\"",
        "id=\"notice\"",
        "data-status=\"warning\"",
        "data-label-hidden=\"true\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 4, "{html}");
    assert_eq!(html.matches("data-size=\"lg\"").count(), 4, "{html}");
    assert!(!html.contains("Not the group name"));
}

#[test]
fn group_describes_the_row_with_description_then_detached_status() {
    for message_id in [None, Some(""), Some("not one-id"), Some("custom-status")] {
        let mut dom = VirtualDom::new_with_props(
            |message_id: Option<&str>| {
                rsx! {
                    InputGroup { label: "Search", description: "Workspace only",
                        status: FieldStatusInput {
                            status_type: InputStatusType::Warning,
                            message: Some("Incomplete".into()),
                            message_id: message_id.map(str::to_owned),
                        },
                        Button { label: "Search" }
                    }
                }
            },
            message_id,
        );
        dom.rebuild_in_place();
        let html = render(&dom);
        let row_start = html.find("role=\"group\"").expect("group row");
        let (before_row, _) = html[..row_start].rsplit_once('<').expect("row opening tag");
        let (row_attrs, after_row) = html[before_row.len()..]
            .split_once('>')
            .expect("end of row opening tag");
        let (_, refs) = row_attrs
            .split_once("aria-describedby=\"")
            .expect("row description");
        let (refs, _) = refs.split_once('"').expect("end of description attribute");
        let ids: Vec<_> = refs.split_ascii_whitespace().collect();
        let [description_id, status_id] = ids.as_slice() else {
            panic!("expected one description ID and one status ID: {html}");
        };
        assert!(
            before_row.contains(&format!("id=\"{description_id}\"")),
            "{html}"
        );
        assert!(before_row.contains("Workspace only"), "{html}");
        let status_tag = after_row
            .split('<')
            .filter_map(|part| part.split_once('>').map(|(tag, _)| tag))
            .find(|tag| tag.contains("data-variant=\"detached\""))
            .expect("detached status after the row");
        assert!(
            status_tag.contains(&format!("id=\"{status_id}\"")),
            "{html}"
        );
        assert!(after_row.contains("Incomplete"), "{html}");
        if message_id == Some("custom-status") {
            assert_eq!(*status_id, "custom-status");
        }
    }
}
