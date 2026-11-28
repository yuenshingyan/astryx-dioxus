use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::Drawer;

#[test]
fn native_focus_fallback_does_not_claim_the_caller_marker() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Drawer { is_open: true, on_open_change: |_| {}, label: "Details",
                button { "Review" }
                button { "data-autofocus": "true", "Edit" }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let (root, content) = html.split_once('>').expect("Drawer renders a native host");
    assert!(root.starts_with("<dialog "), "{html}");
    assert!(root.contains("aria-label=\"Details\""), "{html}");
    assert!(
        !root.contains(" open") && !root.contains("aria-modal"),
        "{html}"
    );

    let (fallback, children) = content
        .split_once('>')
        .expect("Drawer renders a content container");
    assert!(
        fallback.starts_with("<div ") && fallback.contains(" tabindex=-1 "),
        "{html}"
    );
    assert!(fallback.contains(" autofocus"), "{html}");
    assert!(!fallback.contains("data-autofocus"), "{html}");
    assert!(
        children.contains("data-autofocus=\"true\">Edit</button>"),
        "{html}"
    );
    assert_eq!(html.matches("data-autofocus").count(), 1, "{html}");
}
