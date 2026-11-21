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
