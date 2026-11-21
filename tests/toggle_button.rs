use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::ToggleButton;

#[test]
fn pressed_state_owns_semantics_and_swaps_icon() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ToggleButton { label: "Favorite", is_icon_only: true, is_pressed: true, "aria-pressed": "false",
                icon: rsx! { span { "Outline icon" } }, pressed_icon: rsx! { span { "Solid icon" } }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for expected in [
        "aria-pressed=\"true\"",
        "aria-label=\"Favorite\"",
        "data-is-pressed=\"true\"",
        "Solid icon",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("Outline icon") && !html.contains("aria-pressed=\"false\""));
}
