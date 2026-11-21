use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{SideNavCollapseButton, SideNavControlledCollapsible};

#[test]
fn no_state_source_renders_no_button() {
    let mut dom = VirtualDom::new(|| rsx! { SideNavCollapseButton {} });
    dom.rebuild_in_place();
    assert!(!render(&dom).contains("<button"));
}

#[test]
fn controlled_state_renders_custom_named_icon_only_button() {
    fn app() -> Element {
        let config = SideNavControlledCollapsible {
            is_collapsed: true,
            on_collapsed_change: EventHandler::new(|_| {}),
        };
        rsx! { SideNavCollapseButton { collapsible: config, label: "Toggle navigation", span { "Custom glyph" } } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "aria-label=\"Toggle navigation\"",
        "data-variant=\"ghost\"",
        "data-icon-only=\"true\"",
        "Custom glyph",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("__chevron"), "{html}");
}
