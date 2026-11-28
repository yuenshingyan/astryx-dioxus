use dioxus::prelude::*;
use dioxus_ssr::{render, render_element};

use astryx_dioxus::{
    DrawerHeader, DrawerHeaderEdgeCompensation, InternationalizationProvider, Layout,
    MessagesByLocale, SpacingStep,
};

#[test]
fn close_is_optional_and_title_is_a_level_two_heading() {
    let html = render_element(rsx! {
        DrawerHeader { id: "details", title: rsx! { "Details" }, subtitle: rsx! { a { href: "#region", "Region" } },
            start_content: rsx! { span { "Start" } }, end_content: rsx! { span { "Healthy" } },
        }
    });
    let mut remaining = html.as_str();
    for marker in [
        "id=\"details\"",
        "astryx-drawer-header-start-content",
        ">Start</span>",
        "astryx-drawer-header-title-block",
        "<h2 ",
        ">Details</h2>",
        "<span class=\"astryx-text ",
        "data-color=\"secondary\"",
        "<a href=\"#region\">Region</a>",
        "astryx-drawer-header-end-content",
        ">Healthy</span>",
    ] {
        remaining = remaining
            .split_once(marker)
            .unwrap_or_else(|| panic!("missing or out of order: {marker}: {html}"))
            .1;
    }
    for forbidden in [
        "<button",
        "data-compensation",
        "aria-label",
        "tabindex",
        "autofocus",
    ] {
        assert!(!html.contains(forbidden), "unexpected {forbidden}: {html}");
    }
    let empty = render_element(rsx! {
        DrawerHeader { title: rsx! { "Details" }, subtitle: rsx! { "" }, start_content: rsx! {}, end_content: rsx! { "" } }
    });
    for forbidden in [
        "<span class=\"astryx-text ",
        "astryx-drawer-header-start-content",
        "astryx-drawer-header-end-content",
    ] {
        assert!(
            !empty.contains(forbidden),
            "unexpected {forbidden}: {empty}"
        );
    }
}

#[test]
fn close_uses_a_named_native_button_and_matching_tooltip() {
    for (catalog, label, expected) in [
        (None, "", "Close"),
        (Some("Fermer"), "", "Fermer"),
        (Some("Fermer"), " \t", "Fermer"),
        (Some("Fermer"), "Close", "Close"),
        (Some("Fermer"), "Dismiss", "Dismiss"),
        (Some(" \t"), "", "Close"),
    ] {
        let messages: MessagesByLocale = catalog
            .map(|message| {
                (
                    "fr".into(),
                    [("@astryx.dialog.close".into(), message.into())].into(),
                )
            })
            .into_iter()
            .collect();
        let mut dom = VirtualDom::new_with_props(
            |(catalog, label, messages): (Option<&str>, &str, MessagesByLocale)| {
                if catalog.is_some() {
                    rsx! { InternationalizationProvider { locale: "fr-CA", messages,
                        DrawerHeader { title: rsx! { "Details" }, close_label: label, on_open_change: |_| {},
                            end_content: rsx! { button { r#type: "button", "Edit" } },
                        }
                    } }
                } else {
                    rsx! { DrawerHeader { title: rsx! { "Details" }, close_label: label, on_open_change: |_| {},
                        end_content: rsx! { button { r#type: "button", "Edit" } },
                    } }
                }
            },
            (catalog, label, messages),
        );
        dom.rebuild_in_place();
        let html = render(&dom);
        let (caller_action, trailing) = html
            .split_once(">Edit</button>")
            .expect("independent caller button");
        assert!(caller_action.contains("<button "), "{html}");
        assert!(!caller_action.contains("aria-label"), "{html}");
        let (_, button) = trailing
            .split_once("<button ")
            .expect("native close button after caller action");
        let (attrs, rest) = button.split_once('>').expect("button attributes");
        for expected in [
            format!("aria-label=\"{expected}\""),
            "type=\"button\"".to_owned(),
            "data-variant=\"ghost\"".to_owned(),
            "data-icon-only=\"true\"".to_owned(),
        ] {
            assert!(attrs.contains(&expected), "missing {expected}: {html}");
        }
        assert!(!attrs.contains("aria-describedby"), "{html}");
        let (glyph, tooltip) = rest.split_once("</button>").expect("button end");
        assert!(
            glyph.contains("astryx-drawer-header-close-icon")
                && glyph.contains("aria-hidden=\"true\""),
            "{html}"
        );
        assert!(
            tooltip.contains("role=\"tooltip\"") && tooltip.contains(&format!(">{expected}</div>")),
            "{html}"
        );
    }
}

#[test]
fn layout_divider_and_end_compensation_precedence_are_independent() {
    for (compensation, expected) in [
        (None, None),
        (Some(DrawerHeaderEdgeCompensation::Inline), Some("inline")),
        (Some(DrawerHeaderEdgeCompensation::Block), Some("block")),
        (Some(DrawerHeaderEdgeCompensation::All), Some("all")),
    ] {
        for has_close in [false, true] {
            let expected = expected.or(has_close.then_some("all"));
            let mut dom = VirtualDom::new_with_props(
                |(compensation, has_close): (Option<DrawerHeaderEdgeCompensation>, bool)| {
                    let close = has_close.then(|| EventHandler::new(|_: bool| {}));
                    rsx! {
                        Layout { default_has_dividers: true, padding: SpacingStep::Four,
                            header: rsx! {
                                DrawerHeader { title: rsx! { "Details" }, end_content: rsx! { "0" },
                                    on_open_change: close, end_content_edge_compensation: compensation,
                                }
                            },
                            DrawerHeader { title: rsx! { "Override" }, has_divider: false }
                        }
                    }
                },
                (compensation, has_close),
            );
            dom.rebuild_in_place();
            let html = render(&dom);
            assert_eq!(html.matches("data-divider=\"true\"").count(), 1, "{html}");
            let (header, rest) = html
                .split_once("class=\"astryx-layout-header__inner\"")
                .expect("one LayoutHeader inner wrapper per header");
            assert!(header.contains("data-divider=\"true\""), "{html}");
            let (_, end_slot) = rest
                .split_once("class=\"astryx-drawer-header-end-content\"")
                .expect("end slot");
            let (attrs, rest) = end_slot.split_once('>').expect("end slot attributes");
            assert!(rest.starts_with('0'), "{html}");
            if let Some(expected) = expected {
                assert!(
                    attrs.contains(&format!("data-compensation=\"{expected}\"")),
                    "{html}"
                );
            } else {
                assert!(!attrs.contains("data-compensation"), "{html}");
            }
        }
    }
}
