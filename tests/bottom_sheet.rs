use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::BottomSheet;

fn sheet(is_open: bool) -> Element {
    rsx! {
        BottomSheet { is_open, label: "Filters",
            button { "Apply" }
        }
    }
}

#[test]
fn initially_open_standalone_sheet_renders_closed_on_the_server() {
    let mut closed = VirtualDom::new(|| sheet(false));
    closed.rebuild_in_place();
    let mut requested_open = VirtualDom::new(|| sheet(true));
    requested_open.rebuild_in_place();

    let html = render(&requested_open);
    let (host, _) = html.split_once('>').expect("native sheet host");
    assert!(host.starts_with("<dialog "), "{html}");
    assert!(host.contains("aria-label=\"Filters\""), "{html}");
    assert!(host.contains("data-open=\"false\""), "{html}");
    assert!(
        !host.contains(" open") && !host.contains("aria-modal"),
        "{html}"
    );
    assert_eq!(html, render(&closed));
}
