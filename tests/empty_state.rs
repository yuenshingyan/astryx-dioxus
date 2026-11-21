use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{EmptyState, HeadingLevel};

#[test]
fn heading_level_and_optional_content_have_native_semantics() {
    let html = render_element(rsx! { EmptyState {
        id: "notice", title: "Nothing found", description: "Try another filter", heading_level: HeadingLevel::Six,
        is_compact: true, icon: rsx! { span { "Artwork" } }, actions: rsx! { button { "Retry" } },
    } });
    for expected in [
        "id=\"notice\"",
        "<h6",
        "Nothing found",
        "Try another filter",
        "data-variant=\"compact\"",
        "aria-hidden=\"true\"",
        "<button",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert!(!html.contains("aria-live=") && !html.contains("role=\"status\""));
}

#[test]
fn empty_slots_do_not_create_wrappers() {
    let html = render_element(rsx! { EmptyState {
        title: "Nothing found", description: "", icon: rsx! { if false { span {} } }, actions: rsx! {},
    } });
    assert!(html.contains("<h3"));
    for absent in [
        "aria-hidden=",
        "astryx-empty-state-description",
        "astryx-empty-state__actions",
    ] {
        assert!(!html.contains(absent), "{html}");
    }
}
