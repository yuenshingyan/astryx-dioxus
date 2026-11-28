use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    CheckIndicator, CheckIndicatorState, IconEntry, IconSize, IndicatorSize, Spinner, SpinnerShade,
    SpinnerSize, register_icons,
};

#[test]
fn unchecked_has_no_box_and_checked_uses_the_icon_root() {
    let html = render_element(rsx! {
        CheckIndicator { state: CheckIndicatorState::Unchecked, id: "absent" }
        CheckIndicator { state: CheckIndicatorState::Checked, id: "chosen", class: "host-mark", style: "opacity:.75", "aria-hidden": "false" }
    });
    assert!(!html.contains("absent"));
    assert!(html.starts_with("<span"), "{html}");
    let root = html.split_once('>').unwrap().0;
    for expected in [
        "astryx-icon host-mark",
        "id=\"chosen\"",
        "aria-hidden=\"true\"",
        "data-size=\"sm\"",
        "width:var(--_icon-width, 1rem);height:var(--_icon-height, 1rem)",
        "color:var(--color-accent);opacity:.75",
    ] {
        assert!(root.contains(expected), "{html}");
    }
    assert!(html.contains("<svg"));
    assert!(html.contains("d=\"M5 13l4 4L19 7\""), "{html}");
    assert!(html.contains("stroke-width=\"1.5\""), "{html}");
    assert!(!html.contains("aria-hidden=\"false\""));
}

#[test]
fn replacement_in_either_state_preserves_the_box_and_native_attributes() {
    let html = render_element(rsx! {
        CheckIndicator { state: CheckIndicatorState::Unchecked, is_disabled: true, id: "busy", class: "host-slot", style: "opacity:.5", "aria-hidden": "false", replacement: rsx! { "0" } }
        CheckIndicator { state: CheckIndicatorState::Checked, id: "empty", replacement: rsx! {} }
    });
    for expected in [
        "id=\"busy\"",
        "class=\"host-slot\"",
        "id=\"empty\"",
        "width:1rem;height:1rem",
        "color:var(--color-icon-disabled);opacity:.5",
        ">0</span>",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert_eq!(html.matches("aria-hidden=\"true\"").count(), 2);
    assert!(!html.contains("aria-hidden=\"false\""));
    assert!(!html.contains("<svg"));
}

#[test]
fn busy_visual_leaves_state_and_name_on_the_host() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            button { "aria-pressed": "true", "aria-busy": "true",
                "Choose option"
                CheckIndicator {
                    state: CheckIndicatorState::Checked,
                    size: IndicatorSize::Small,
                    replacement: rsx! { Spinner { size: SpinnerSize::Small, shade: SpinnerShade::Inherit, is_decorative: true } },
                }
            }
        }
    });
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    let (host, content) = html.split_once('>').unwrap();
    assert!(host.starts_with("<button "), "{html}");
    assert!(host.contains("aria-pressed=\"true\""), "{html}");
    assert!(host.contains("aria-busy=\"true\""), "{html}");
    assert!(content.starts_with("Choose option<span "), "{html}");
    let slot = content.split_once('>').unwrap().0;
    assert!(slot.contains("aria-hidden=\"true\""), "{html}");
    assert!(slot.contains("color:var(--color-accent)"), "{html}");
    assert!(content.contains("astryx-spinner"), "{html}");
    for unexpected in [" tabindex=", " role=", " aria-label="] {
        assert!(!slot.contains(unexpected), "{html}");
    }
    assert!(!content.contains("astryx-icon"), "{html}");
}

#[test]
fn both_sizes_request_small_registry_artwork_and_keep_disabled_paint_visual_only() {
    register_icons([(
        "check".into(),
        IconEntry::Renderer(Rc::new(|request| {
            assert_eq!(request.size, IconSize::Small);
            assert_eq!(request.variant, None);
            rsx! { span { "custom check" } }
        })),
    )])
    .unwrap();
    let html = render_element(rsx! {
        CheckIndicator { state: CheckIndicatorState::Checked, size: IndicatorSize::Small, is_disabled: true }
        CheckIndicator { state: CheckIndicatorState::Checked, size: IndicatorSize::Medium, is_disabled: true }
        CheckIndicator { state: CheckIndicatorState::Unchecked, is_disabled: true }
    });
    for expected in [
        "custom check",
        "data-size=\"sm\"",
        "color:var(--color-icon-disabled)",
    ] {
        assert_eq!(html.matches(expected).count(), 2, "{html}");
    }
    for unexpected in [
        "<svg",
        " aria-disabled=",
        " tabindex=",
        " role=",
        " aria-label=",
    ] {
        assert!(!html.contains(unexpected), "{html}");
    }
}
