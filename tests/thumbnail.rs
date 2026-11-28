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
fn caller_name_does_not_replace_group_role_or_action_names() {
    let mut dom = VirtualDom::new(|| {
        rsx! { Thumbnail {
            label: "Receipt", role: "img", "aria-label": "Attachment", on_click: |_| {}, on_remove: |_| {},
        } }
    });
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    let (root, _) = html.split_once('>').expect("thumbnail opening tag");
    assert!(root.contains("role=\"group\""), "{html}");
    assert_eq!(root.matches(" role=").count(), 1, "{html}");
    assert!(root.contains("aria-label=\"Attachment\""), "{html}");
    assert!(html.contains("aria-label=\"Open Receipt\""), "{html}");
    assert!(html.contains("aria-label=\"Remove Receipt\""), "{html}");
}

#[test]
fn caller_labelledby_reaches_the_group() {
    let html = render_element(rsx! { Thumbnail {
        alt: "A receipt", role: "presentation", "aria-labelledby": "attachment-caption",
    } });
    let (root, _) = html.split_once('>').expect("thumbnail opening tag");
    assert!(root.contains("role=\"group\""), "{html}");
    assert_eq!(root.matches(" role=").count(), 1, "{html}");
    assert!(
        root.contains("aria-labelledby=\"attachment-caption\""),
        "{html}"
    );
    assert!(!root.contains("aria-label="), "{html}");
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
