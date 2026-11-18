use astryx_dioxus::{
    ButtonElevation, ButtonGroup, ButtonSize, ButtonType, ButtonVariant, IconButton,
};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn icon_button_is_named_and_icon_only_without_a_visible_label() {
    let html = render(|| rsx! { IconButton { label: "Settings", icon: rsx! { span { "⚙" } } } });
    assert!(html.contains("aria-label=\"Settings\""), "{html}");
    assert!(html.contains("data-icon-only=\"true\""), "{html}");
    assert!(html.contains("⚙"), "{html}");
    assert!(!html.contains("astryx-button__label"), "{html}");
    assert!(!html.contains("astryx-button__end"), "{html}");
}

#[test]
fn button_states_form_fields_attributes_and_callbacks_are_forwarded() {
    let html = render(|| {
        rsx! {
            IconButton {
                label: "Save", icon: rsx! { svg {} }, variant: ButtonVariant::Primary,
                size: ButtonSize::Large, elevation: ButtonElevation::High, button_type: ButtonType::Submit,
                name: "action", value: "save", form: "editor", id: "save-icon", title: "Save",
                is_loading: true, is_interruptible: true, loading_label: "Saving", tooltip: "Save changes",
                onfocus: move |_: FocusEvent| {}, onpointerdown: move |_: PointerEvent| {},
                onkeyup: move |_: KeyboardEvent| {}, onmounted: move |_: MountedEvent| {},
            }
        }
    });
    for attribute in [
        "id=\"save-icon\"",
        "data-size=\"lg\"",
        "data-variant=\"primary\"",
        "data-elevation=\"high\"",
        "type=\"submit\"",
        "name=\"action\"",
        "value=\"save\"",
        "form=\"editor\"",
        "title=\"Save\"",
        "aria-busy=\"true\"",
    ] {
        assert!(html.contains(attribute), "missing {attribute}: {html}");
    }
    assert!(
        !html.contains("aria-disabled=\"true\""),
        "interruptible loading stays operable: {html}"
    );
    assert!(html.contains("Save changes"), "{html}");
}

#[test]
fn grouped_icon_links_preserve_flattening_and_disabled_native_fallback() {
    let html = render(|| {
        rsx! {
            ButtonGroup { label: "Actions", is_disabled: true,
                IconButton { label: "Navigate", icon: rsx! { svg {} }, href: "/next", elevation: ButtonElevation::High }
            }
        }
    });
    assert!(!html.contains("href=\"/next\""), "{html}");
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 2, "{html}");
    assert!(html.contains("data-astryx-grouped=\"true\""), "{html}");
    assert_eq!(html.matches("data-elevation=\"none\"").count(), 2, "{html}");
}
