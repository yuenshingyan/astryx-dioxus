use dioxus::prelude::*;

use astryx_dioxus::{DialogHeader, DialogHeaderEdgeCompensation, Link};

fn render<F: FnOnce() -> Element + Clone + 'static>(app: F) -> String {
    let mut dom = VirtualDom::new_with_props(|app: F| app(), app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn title_has_native_heading_semantics_and_focus_target() {
    let html = render(|| {
        rsx! {
            DialogHeader { id: "header", title: rsx! { em { "Invite teammates" } }, title_id: "title",
                subtitle: rsx! { Link { href: "/sharing-policy", "Sharing policy" } },
                start_content: rsx! { "Start" }, end_content: rsx! { "End" },
            }
        }
    });
    for expected in [
        "id=\"header\"",
        "<h2 ",
        "id=\"title\"",
        "tabindex=\"-1\"",
        "data-autofocus=\"dialog-title\"",
        "<em>Invite teammates</em>",
        "href=\"/sharing-policy\"",
        "Sharing policy",
        "Start",
        "End",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("<button"), "{html}");
}

#[test]
fn inline_preview_keeps_programmatic_focus_without_requesting_autofocus() {
    let html = render(|| rsx! { DialogHeader { title: rsx! { "Details" }, is_inline: true } });
    assert!(
        html.contains("tabindex=\"-1\"") && !html.contains("data-autofocus"),
        "{html}"
    );
}

#[test]
fn close_has_a_nonblank_name_and_does_not_submit_a_form() {
    let html = render(|| {
        rsx! {
            form {
                DialogHeader { title: rsx! { "Default" }, on_open_change: |_| {} }
                DialogHeader { title: rsx! { "Blank" }, close_label: " \t ", on_open_change: |_| {} }
                DialogHeader { title: rsx! { "Localized" }, close_label: "Fermer", on_open_change: |_| {} }
            }
        }
    });
    let buttons: Vec<_> = html.split("<button ").skip(1).collect();
    assert_eq!(buttons.len(), 3, "{html}");
    for (button, label) in buttons.into_iter().zip(["Close", "Close", "Fermer"]) {
        let (attributes, contents) = button.split_once('>').unwrap();
        assert!(attributes.contains("type=\"button\""), "{attributes}");
        assert!(
            attributes.contains(&format!("aria-label=\"{label}\"")),
            "{attributes}"
        );
        let icon = contents
            .split_once("astryx-dialog-header-close-icon")
            .unwrap()
            .1;
        assert!(
            icon.split('>')
                .next()
                .unwrap()
                .contains("aria-hidden=\"true\""),
            "{contents}"
        );
    }
}

#[test]
fn trailing_slot_selects_compensation_without_reordering_actions() {
    for (choice, explicit) in [
        (None, None),
        (Some(DialogHeaderEdgeCompensation::Inline), Some("inline")),
        (Some(DialogHeaderEdgeCompensation::Block), Some("block")),
        (Some(DialogHeaderEdgeCompensation::All), Some("all")),
    ] {
        for has_close in [false, true] {
            for has_end in [false, true] {
                let html = render(move || {
                    rsx! {
                        DialogHeader {
                            title: rsx! { "Details" },
                            end_content: Some(if has_end { rsx! { span { id: "end-action", "Help" } } } else { rsx! {} }),
                            on_open_change: has_close.then(|| EventHandler::new(|_: bool| {})),
                            end_content_edge_compensation: choice,
                        }
                    }
                });
                let slot = html.split_once("class=\"astryx-dialog-header-end-content\"");
                assert_eq!(slot.is_some(), has_end || has_close, "{html}");
                let Some((_, slot)) = slot else { continue };
                let (attributes, contents) = slot.split_once('>').unwrap();
                let expected = explicit.or(has_close.then_some("all"));
                let actual = attributes
                    .split_once("data-compensation=\"")
                    .and_then(|(_, value)| value.split_once('"').map(|(value, _)| value));
                assert_eq!(actual, expected, "{html}");
                let end = contents.find("id=\"end-action\"");
                let close = contents.find("aria-label=\"Close\"");
                assert_eq!(end.is_some(), has_end, "{html}");
                assert_eq!(close.is_some(), has_close, "{html}");
                if let (Some(end), Some(close)) = (end, close) {
                    assert!(end < close, "{html}");
                }
            }
        }
    }
}
