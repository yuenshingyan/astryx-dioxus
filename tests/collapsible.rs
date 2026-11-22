use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::Collapsible;

#[test]
fn renders_closed_content_without_unmounting_it() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Collapsible { trigger: rsx! { "Details" }, is_open: false, is_disabled: true,
                input { value: "Retained" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let panel_id = html
        .split("aria-controls=\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("controls ID");
    assert!(html.contains(&format!("id=\"{panel_id}\"")), "{html}");
    for attribute in [
        "aria-expanded=\"false\"",
        "aria-disabled=\"true\"",
        "hidden",
        "Retained",
    ] {
        assert!(html.contains(attribute), "{html}");
    }
}
