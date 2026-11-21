use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{Button, ElementSize, Toolbar, ToolbarOrientation};

#[test]
fn toolbar_names_three_slots_and_passes_size_to_buttons() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Toolbar { label: "Document", id: "tools", size: ElementSize::Small, orientation: ToolbarOrientation::Vertical,
                start_content: rsx! { Button { label: "Edit" } }, center_content: rsx! { "Center" }, end_content: rsx! { "End" },
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "role=\"toolbar\"",
        "aria-label=\"Document\"",
        "aria-orientation=\"vertical\"",
        "id=\"tools\"",
        "data-center=\"true\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert_eq!(html.matches("data-size=\"sm\"").count(), 2, "{html}");
}
