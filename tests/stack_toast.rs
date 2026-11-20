use dioxus::prelude::*;
use dioxus_ssr::render_element;

#[path = "../src/toast/pattern.rs"]
mod toast;

use toast::{DemoToastViewport, use_demo_toasts};

#[component]
fn EmptyDemo() -> Element {
    let toasts = use_demo_toasts();
    rsx! { DemoToastViewport { toasts, label: "Notifications", dismiss_label: "Dismiss notification" } }
}

#[test]
fn empty_private_viewport_has_no_landmark_or_live_card() {
    let html = render_element(rsx! { EmptyDemo {} });
    assert!(html.contains("popover=\"manual\""));
    assert!(html.contains("aria-label=\"Notifications\""));
    assert!(!html.contains("role=\"region\""));
    assert!(!html.contains("role=\"status\""));
    assert!(!html.contains("class=\"astryx-toast\""));
}

#[component]
fn VisibleDemo() -> Element {
    let toasts = use_demo_toasts();
    use_hook(move || {
        toasts.show("Caller supplied body");
    });
    rsx! { DemoToastViewport { toasts, label: "Messages", dismiss_label: "Close message" } }
}

#[test]
fn private_default_card_uses_caller_text_and_existing_button() {
    let html = render_element(rsx! { VisibleDemo {} });
    assert!(html.contains("role=\"region\""));
    assert!(html.contains("aria-label=\"Messages\""));
    assert!(html.contains("class=\"astryx-toast\" data-type=\"info\""));
    assert!(html.contains("Caller supplied body"));
    assert!(html.contains("aria-label=\"Close message\""));
    assert!(html.contains("data-variant=\"ghost\""));
    assert!(html.contains("data-size=\"sm\""));
    assert_eq!(html.matches("data-toast-id=").count(), 1);
}
