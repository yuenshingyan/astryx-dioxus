use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{CheckboxIndicatorState, CheckboxInput};

#[test]
fn mixed_checkbox_renders_native_label_and_description_relationships() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            CheckboxInput { id: "alerts", label: "Alerts", description: "Some alerts enabled", value: CheckboxIndicatorState::Indeterminate }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "type=\"checkbox\"",
        "id=\"alerts\"",
        "for=\"alerts\"",
        "aria-checked=\"mixed\"",
        "aria-describedby=\"alerts-desc\"",
        "id=\"alerts-desc\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}
