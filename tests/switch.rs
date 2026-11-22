use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::Switch;

#[test]
fn disabled_switch_keeps_native_checked_state_without_a_form_name() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Switch { id: "alerts", label: "Alerts", description: "Send updates.", value: true, html_name: "alerts", is_disabled: true }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for fragment in [
        "role=\"switch\"",
        "for=\"alerts\"",
        "aria-disabled=\"true\"",
        "aria-describedby=\"alerts-desc\"",
        " checked",
    ] {
        assert!(html.contains(fragment), "{html}");
    }
    assert!(!html.contains("name=\"alerts\""), "{html}");
}
