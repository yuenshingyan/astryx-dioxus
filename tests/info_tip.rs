use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::InfoTip;

#[test]
fn closed_button_references_help_without_being_expanded() {
    let mut dom = VirtualDom::new(
        || rsx! { InfoTip { content: "Only admins can edit this", label: "Edit permissions" } },
    );
    dom.rebuild_in_place();
    let html = render(&dom);
    let id = html
        .split("aria-describedby=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    assert!(html.contains(&format!("id=\"{id}\"")));
    for expected in [
        "type=\"button\"",
        "aria-label=\"Edit permissions\"",
        "aria-expanded=\"false\"",
        "role=\"tooltip\"",
        "Only admins can edit this",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("aria-controls"));
}
