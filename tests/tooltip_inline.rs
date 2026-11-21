use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::Tooltip;

#[test]
fn descriptions_are_linked_and_redundant_help_adds_no_tab_stop() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Tooltip { text: "Privacy policy", content: "Data is encrypted" }
            Tooltip { text: "Duplicate", content: "Duplicate" }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert_eq!(html.matches("tabindex=\"0\"").count(), 1, "{html}");
    assert_eq!(html.matches("role=\"tooltip\"").count(), 1, "{html}");
    let description = html
        .split("aria-describedby=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    assert!(
        html.contains(&format!("id=\"{description}\"")) && html.contains("Data is encrypted"),
        "{html}"
    );
}
