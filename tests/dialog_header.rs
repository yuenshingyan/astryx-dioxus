use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::DialogHeader;

#[test]
fn title_has_native_heading_semantics_and_focus_target() {
    let html = render_element(rsx! {
        DialogHeader { id: "header", title: rsx! { em { "Invite teammates" } }, title_id: "title",
            subtitle: rsx! { "Workspace" }, start_content: rsx! { "Start" }, end_content: rsx! { "End" },
        }
    });
    for expected in [
        "id=\"header\"",
        "<h2 ",
        "id=\"title\"",
        "tabindex=\"-1\"",
        "data-autofocus=\"dialog-title\"",
        "<em>Invite teammates</em>",
        "Workspace",
        "Start",
        "End",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("<button"), "{html}");
}

#[test]
fn inline_preview_keeps_programmatic_focus_without_requesting_autofocus() {
    let html = render_element(rsx! { DialogHeader { title: rsx! { "Details" }, is_inline: true } });
    assert!(
        html.contains("tabindex=\"-1\"") && !html.contains("data-autofocus"),
        "{html}"
    );
}
