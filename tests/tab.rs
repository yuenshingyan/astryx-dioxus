use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{Tab, TabMode};

#[test]
fn panel_mode_ignores_href_and_associates_its_panel() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Tab { label: "Inbox", value: "inbox", is_selected: true, href: "/elsewhere",
                mode: TabMode::Panel { panel_id: "inbox-panel".into() } }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for fragment in [
        "<button",
        "role=\"tab\"",
        "aria-selected=\"true\"",
        "aria-controls=\"inbox-panel\"",
    ] {
        assert!(html.contains(fragment), "{html}");
    }
    assert!(
        !html.contains("href=") && !html.contains("aria-current"),
        "{html}"
    );
}
