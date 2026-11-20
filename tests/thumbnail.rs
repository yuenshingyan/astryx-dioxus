use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::Thumbnail;

#[test]
fn decorative_image_and_separate_disabled_actions() {
    let mut dom = VirtualDom::new(|| {
        rsx! { Thumbnail {
            src: "/image.png", label: "Receipt", is_disabled: true, on_click: |_| {}, on_remove: |_| {},
        } }
    });
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    for expected in [
        "role=\"group\"",
        "aria-label=\"Receipt\"",
        "aria-label=\"Open Receipt\"",
        "aria-label=\"Remove Receipt\"",
        "role=\"presentation\"",
        "alt=\"\"",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert_eq!(html.matches("<button").count(), 2);
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 2);
}

#[test]
fn upload_keeps_named_image_and_hides_spinner_semantics() {
    let html = render_element(rsx! { Thumbnail {
        src: "/image.png", alt: "A receipt", label: "Receipt", is_loading: true,
    } });
    assert!(html.contains("aria-label=\"Receipt — A receipt\""));
    assert!(html.contains("alt=\"A receipt\"") && html.contains("astryx-thumbnail__upload"));
    assert!(!html.contains("role=\"progressbar\""));
}
