use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{List, ListDensity, ListItem, ListMarkerStyle};

#[test]
fn inherits_list_settings_and_keeps_actions_caller_owned() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            List { density: ListDensity::Compact, list_style: ListMarkerStyle::Decimal, has_dividers: true,
                ListItem { label: "Install", id: "install", end_content: rsx! { button { "Details" } } }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let (_, row) = html.split_once("<li ").unwrap();
    for expected in [
        "id=\"install\"",
        "data-density=\"compact\"",
        "data-kind=\"decimal\"",
        "aria-hidden=\"true\"",
        "astryx-list-item--divided",
        "Details",
    ] {
        assert!(row.contains(expected), "missing {expected}: {html}");
    }
    assert!(!row.contains("tabindex"));
}
