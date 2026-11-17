use astryx_dioxus::{Button, ButtonElevation, ButtonSize, ButtonType, ButtonVariant};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

fn root_tag(html: &str) -> &str {
    html.split_once('>').expect("rendered root tag").0
}

#[test]
fn defaults_match_upstream_and_render_label() {
    let html = render(|| rsx! { Button { label: "Save" } });
    let root = root_tag(&html);
    assert!(root.starts_with("<button"), "{html}");
    [
        "type=\"button\"",
        "data-variant=\"secondary\"",
        "data-size=\"md\"",
        "data-elevation=\"none\"",
    ]
    .iter()
    .for_each(|attribute| assert!(root.contains(attribute), "{html}"));
    assert!(html.contains(">Save</span>"), "{html}");
    assert!(!root.contains("aria-disabled"), "{html}");
    assert!(!root.contains("aria-busy"), "{html}");
    assert!(!root.contains("aria-label"), "{html}");
    assert!(!html.contains("astryx-button__icon"), "{html}");
    assert!(
        html.contains("role=\"status\" aria-live=\"polite\"")
            || html.contains("aria-live=\"polite\" role=\"status\""),
        "{html}"
    );
}

#[test]
fn variants_sizes_and_elevation_reach_theme_attributes() {
    let html = render(|| {
        rsx! {
            Button { label: "Delete", variant: ButtonVariant::Destructive, size: ButtonSize::Large, elevation: ButtonElevation::High }
            Button { label: "Primary", variant: ButtonVariant::Primary, size: ButtonSize::Small, elevation: ButtonElevation::Low }
            Button { label: "Ghost", variant: ButtonVariant::Ghost, elevation: ButtonElevation::Medium }
        }
    });
    [
        "data-variant=\"destructive\"",
        "data-size=\"lg\"",
        "data-elevation=\"high\"",
        "data-variant=\"primary\"",
        "data-size=\"sm\"",
        "data-elevation=\"low\"",
        "data-variant=\"ghost\"",
        "data-elevation=\"med\"",
        "data-astryx-edge-comp=\"\"",
    ]
    .iter()
    .for_each(|attribute| assert!(html.contains(attribute), "{html}"));
}

#[test]
fn disabled_button_remains_focusable_and_forwards_global_attributes() {
    let html = render(|| {
        rsx! {
            Button {
                label: "Save", is_disabled: true, class: "custom-button", id: "save",
                name: "action", value: "save", "aria-label": "Save changes", "data-test": "button"
            }
        }
    });
    let root = root_tag(&html);
    assert!(root.contains("aria-disabled=\"true\""), "{html}");
    assert!(root.contains("aria-label=\"Save changes\""), "{html}");
    assert!(root.contains("custom-button"), "{html}");
    assert!(root.contains("id=\"save\""), "{html}");
    assert!(root.contains("data-test=\"button\""), "{html}");
    assert!(!root.contains(" disabled="), "{html}");
    assert!(!root.contains("tabindex=\"-1\""), "{html}");
    assert!(!root.contains(" name="), "{html}");
}

#[test]
fn loading_preserves_label_and_exposes_busy_without_native_disabled() {
    let html =
        render(|| rsx! { Button { label: "Save", is_loading: true, loading_label: "Loading" } });
    let root = root_tag(&html);
    assert!(root.contains("aria-busy=\"true\""), "{html}");
    assert!(root.contains("aria-disabled=\"true\""), "{html}");
    assert!(root.contains("aria-label=\"Save\""), "{html}");
    assert!(!root.contains(" disabled="), "{html}");
    assert!(html.contains("astryx-button__spinner"), "{html}");
    assert!(
        html.contains("astryx-button__content\" aria-hidden=\"true\""),
        "{html}"
    );
    assert!(html.contains(">Save</span>"), "{html}");
    // First-render loading announces only after the mounted live region exists.
    assert!(!html.contains(">Loading<"), "{html}");
}

#[test]
fn interruptible_loading_remains_operable_but_disabled_wins() {
    let html =
        render(|| rsx! { Button { label: "Refresh", is_loading: true, is_interruptible: true } });
    assert!(root_tag(&html).contains("aria-busy=\"true\""), "{html}");
    assert!(!root_tag(&html).contains("aria-disabled"), "{html}");
    assert!(
        root_tag(&html).contains("data-loading-delayed=\"true\""),
        "{html}"
    );
    let disabled = render(
        || rsx! { Button { label: "Refresh", is_loading: true, is_interruptible: true, is_disabled: true } },
    );
    assert!(
        root_tag(&disabled).contains("aria-disabled=\"true\""),
        "{disabled}"
    );
    assert!(
        root_tag(&disabled).contains("data-disabled=\"true\""),
        "{disabled}"
    );
}

#[test]
fn link_mode_keeps_native_navigation_attributes_and_width() {
    let html = render(|| {
        rsx! {
            Button { label: "Docs", href: "https://example.com", target: "_blank", rel: "noopener noreferrer", width: "100%", style: "margin: 1px;" }
        }
    });
    let root = root_tag(&html);
    assert!(root.starts_with("<a "), "{html}");
    [
        "href=\"https://example.com\"",
        "target=\"_blank\"",
        "rel=\"noopener noreferrer\"",
        "width: 100%; margin: 1px;",
    ]
    .iter()
    .for_each(|attribute| assert!(root.contains(attribute), "{html}"));
    assert!(!root.contains("type="), "{html}");
}

#[test]
fn disabled_link_becomes_button_but_busy_link_stays_anchor() {
    let disabled = render(|| rsx! { Button { label: "Docs", href: "/docs", is_disabled: true } });
    assert!(root_tag(&disabled).starts_with("<button"), "{disabled}");
    assert!(!root_tag(&disabled).contains("href="), "{disabled}");
    let busy = render(|| rsx! { Button { label: "Docs", href: "/docs", is_loading: true } });
    assert!(root_tag(&busy).starts_with("<a "), "{busy}");
    assert!(root_tag(&busy).contains("aria-disabled=\"true\""), "{busy}");
}

#[test]
fn blocked_form_buttons_are_safe_before_mount() {
    let submit = render(
        || rsx! { Button { label: "Save", button_type: ButtonType::Submit, is_disabled: true } },
    );
    let reset = render(
        || rsx! { Button { label: "Reset", button_type: ButtonType::Reset, is_disabled: true } },
    );
    [submit, reset].iter().for_each(|html| {
        assert!(root_tag(html).contains("type=\"button\""), "{html}");
    });
    let busy = render(
        || rsx! { Button { label: "Save", button_type: ButtonType::Submit, is_loading: true } },
    );
    assert!(root_tag(&busy).contains("type=\"button\""), "{busy}");
}

#[test]
fn form_attributes_reach_operable_button() {
    let html = render(
        || rsx! { Button { label: "Save", button_type: ButtonType::Submit, name: "action", value: "save", form: "editor" } },
    );
    [
        "type=\"submit\"",
        "name=\"action\"",
        "value=\"save\"",
        "form=\"editor\"",
    ]
    .iter()
    .for_each(|attribute| assert!(root_tag(&html).contains(attribute), "{html}"));
}

#[test]
fn slots_render_in_order_and_children_replace_visible_label() {
    let html = render(|| {
        rsx! {
            Button { label: "Accessible label", icon: rsx! { span { "leading" } }, end_content: rsx! { span { "trailing" } }, strong { "Visible content" } }
        }
    });
    assert!(
        root_tag(&html).contains("aria-label=\"Accessible label\""),
        "{html}"
    );
    assert!(
        html.find("leading").unwrap() < html.find("Visible content").unwrap(),
        "{html}"
    );
    assert!(
        html.find("Visible content").unwrap() < html.find("trailing").unwrap(),
        "{html}"
    );
}

#[test]
fn icon_only_omits_visible_label_children_and_end_content() {
    let html = render(|| {
        rsx! {
            Button { label: "Add item", is_icon_only: true, icon: rsx! { span { "+" } }, end_content: rsx! { span { "trailing" } }, "custom" }
        }
    });
    assert!(
        root_tag(&html).contains("aria-label=\"Add item\""),
        "{html}"
    );
    assert!(
        root_tag(&html).contains("data-icon-only=\"true\""),
        "{html}"
    );
    assert!(!html.contains("astryx-button__label"), "{html}");
    assert!(!html.contains("trailing"), "{html}");
    assert!(!html.contains("custom"), "{html}");
}

#[test]
fn caller_aria_state_cannot_override_component_state() {
    let html = render(|| {
        rsx! {
            Button { label: "Save", is_disabled: true, is_loading: true, "aria-busy": "false", "aria-disabled": "false", "aria-label": "Wrong name" }
        }
    });
    let root = root_tag(&html);
    assert_eq!(root.matches("aria-busy=").count(), 1, "{html}");
    assert_eq!(root.matches("aria-disabled=").count(), 1, "{html}");
    assert_eq!(root.matches("aria-label=").count(), 1, "{html}");
    assert!(root.contains("aria-busy=\"true\""), "{html}");
    assert!(root.contains("aria-disabled=\"true\""), "{html}");
    assert!(root.contains("aria-label=\"Save\""), "{html}");
}

#[test]
fn unsafe_destination_is_inert_and_does_not_reach_dom() {
    let html = render(|| rsx! { Button { label: "Unsafe link", href: "javascript:alert(1)" } });
    let root = root_tag(&html);
    assert!(root.starts_with("<button"), "{html}");
    assert!(root.contains("aria-disabled=\"true\""), "{html}");
    assert!(!html.contains("javascript:"), "{html}");
}

#[test]
fn forwarded_theme_attributes_cannot_disagree_with_component_props() {
    let html = render(|| {
        rsx! {
            Button { label: "Save", "data-variant": "destructive", "data-size": "lg", "data-disabled": "true" }
        }
    });
    let root = root_tag(&html);
    assert_eq!(root.matches("data-variant=").count(), 1, "{html}");
    assert!(root.contains("data-variant=\"secondary\""), "{html}");
    assert!(root.contains("data-size=\"md\""), "{html}");
    assert!(!root.contains("data-disabled="), "{html}");
}

#[test]
fn caller_aria_state_is_preserved_when_component_is_idle() {
    let html =
        render(|| rsx! { Button { label: "Save", "aria-busy": "true", "aria-disabled": "true" } });
    assert!(root_tag(&html).contains("aria-busy=\"true\""), "{html}");
    assert!(root_tag(&html).contains("aria-disabled=\"true\""), "{html}");
    assert!(!root_tag(&html).contains("data-loading="), "{html}");
}
