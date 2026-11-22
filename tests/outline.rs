use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{Outline, OutlineItem};

#[test]
fn renders_named_navigation_and_encodes_fragment_ids() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Outline { active_id: "Café#part", items: vec![OutlineItem {
                id: "Café#part".into(), label: "Details".into(), level: 3,
            }] }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for fragment in [
        "<nav",
        "aria-label=\"Table of contents\"",
        "href=\"#Caf%C3%A9%23part\"",
        "aria-current=\"location\"",
        "Details",
    ] {
        assert!(html.contains(fragment), "{html}");
    }
}
