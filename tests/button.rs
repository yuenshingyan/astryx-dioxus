use astryx_dioxus::{Button, ButtonVariant};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn primary_button_has_safe_defaults_and_children() {
    let html = render(|| rsx! { Button { "Save" } });
    assert!(html.contains("type=\"button\""), "{html}");
    assert!(html.contains("astryx-button--primary"), "{html}");
    assert!(html.contains("Save"), "{html}");
    assert!(!html.contains("disabled"), "{html}");
}

#[test]
fn secondary_button_supports_disabled_and_accessibility_attributes() {
    let html = render(|| {
        rsx! {
            Button {
                variant: ButtonVariant::Secondary,
                disabled: true,
                class: "custom-button",
                "aria-label": "Save changes",
                "Save"
            }
        }
    });
    assert!(html.contains("astryx-button--secondary"), "{html}");
    assert!(html.contains("disabled"), "{html}");
    assert!(html.contains("custom-button"), "{html}");
    assert!(html.contains("aria-label=\"Save changes\""), "{html}");
}
