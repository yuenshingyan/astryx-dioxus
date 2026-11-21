use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::DrawerHeader;

#[test]
fn close_is_optional_and_title_is_a_level_two_heading() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            DrawerHeader { id: "details", title: rsx! { "Details" }, subtitle: rsx! { "Region" },
                start_content: rsx! { span { "Start" } }, end_content: rsx! { span { "Healthy" } },
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(
        html.contains("id=\"details\"") && html.contains("<h2 "),
        "{html}"
    );
    for text in ["Start", "Details", "Region", "Healthy"] {
        assert!(html.contains(text), "missing {text}: {html}");
    }
    assert!(
        !html.contains("<button") && !html.contains("data-compensation"),
        "{html}"
    );
}

#[test]
fn close_uses_ghost_button_and_default_edge_compensation() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            DrawerHeader { title: rsx! { "Details" }, close_label: "Dismiss", on_open_change: |_| {} }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "aria-label=\"Dismiss\"",
        "data-variant=\"ghost\"",
        "data-compensation=\"all\"",
        "astryx-drawer-header-close-icon",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}
