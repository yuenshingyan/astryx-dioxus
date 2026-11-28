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

#[test]
fn direct_empty_text_omits_optional_slot_wrappers() {
    let empty = String::new();
    let html = render_element(rsx! { EmptyState {
        title: "Nothing found", is_compact: true,
        icon: rsx! { "{empty}" }, actions: rsx! { "{empty}" },
    } });
    assert!(!html.contains("aria-hidden="), "{html}");
    assert!(!html.contains("astryx-empty-state__actions"), "{html}");
}

#[test]
fn nonempty_slot_fragments_keep_wrappers_even_when_children_render_empty() {
    let empty = String::new();
    let html = render_element(rsx! { EmptyState {
        title: "Nothing found", heading_level: HeadingLevel::Two,
        icon: rsx! { for _ in 0..2 { if false { span {} } } },
        actions: rsx! { for _ in 0..1 { "{empty}" } },
    } });
    let icon = html
        .find("aria-hidden=\"true\"")
        .expect("decorative icon wrapper");
    let heading = html.find("<h2").expect("native heading");
    let actions = html
        .find("astryx-empty-state__actions")
        .expect("actions wrapper");
    assert!(icon < heading && heading < actions, "{html}");
    assert!(
        !html.contains("aria-live=") && !html.contains("role=\"status\""),
        "{html}"
    );
}
