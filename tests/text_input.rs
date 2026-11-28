use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{
    FieldStatusInput, FieldStatusPlacement, InputStatusType, TextInput, TextInputType,
};

#[test]
fn disabled_search_keeps_a_label_but_has_no_form_name() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            TextInput { id: "query", label: "Search", value: "Example", input_type: TextInputType::Search,
                is_label_hidden: true, description: "Search the catalog.", html_name: "query", is_disabled: true }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for fragment in [
        "for=\"query\"",
        "type=\"search\"",
        "readonly",
        "aria-disabled=\"true\"",
        "aria-describedby=\"query-desc\"",
    ] {
        assert!(html.contains(fragment), "{html}");
    }
    assert!(!html.contains("name=\"query\""), "{html}");
}

#[test]
fn status_glyph_follows_status_placement() {
    let glyph = "astryx-text-input__status-icon";
    for (placement, severity, message) in [
        (FieldStatusPlacement::Attached, None, None),
        (
            FieldStatusPlacement::Attached,
            Some(InputStatusType::Warning),
            None,
        ),
        (
            FieldStatusPlacement::Attached,
            Some(InputStatusType::Error),
            Some("Display name is taken."),
        ),
        (
            FieldStatusPlacement::Detached,
            Some(InputStatusType::Error),
            Some("Display name is taken."),
        ),
    ] {
        let status = severity.map(|status_type| FieldStatusInput {
            status_type,
            message: message.map(str::to_owned),
            message_id: None,
        });
        let mut dom = VirtualDom::new_with_props(
            |(status, status_variant)| {
                rsx! {
                    TextInput { id: "name", label: "Display name", value: "Sam", status, status_variant,
                        has_clear: true, is_loading: true }
                }
            },
            (status, placement),
        );
        dom.rebuild_in_place();
        let html = render(&dom);
        let has_control_glyph = placement == FieldStatusPlacement::Attached && severity.is_some();
        assert_eq!(html.contains(glyph), has_control_glyph, "{html}");
        if has_control_glyph {
            let (before, after) = html.split_once(glyph).expect("trailing status glyph");
            assert!(before.contains("astryx-input-clear-button"), "{html}");
            let (attributes, _) = after.split_once('>').expect("glyph opening tag");
            assert!(attributes.contains("aria-hidden=\"true\""), "{html}");
        }
        assert_eq!(
            html.contains("astryx-field-status-icon"),
            placement == FieldStatusPlacement::Detached,
            "{html}"
        );
        assert_eq!(
            html.contains("aria-invalid=\"true\""),
            severity == Some(InputStatusType::Error),
            "{html}"
        );
        if let Some(message) = message {
            assert!(html.contains("aria-describedby=\"name-status\""), "{html}");
            assert!(html.contains("id=\"name-status\""), "{html}");
            assert!(html.contains(message), "{html}");
        }
    }
}
