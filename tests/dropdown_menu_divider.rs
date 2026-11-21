use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{BreadcrumbMenuDivider, ContextMenuDivider, DropdownMenuDivider};

#[test]
fn aliases_share_the_same_unfocusable_separator() {
    let html = render_element(rsx! { DropdownMenuDivider {} });
    assert_eq!(html, render_element(rsx! { ContextMenuDivider {} }));
    assert_eq!(html, render_element(rsx! { BreadcrumbMenuDivider {} }));
    for expected in [
        "astryx-divider",
        "astryx-dropdown-menu-divider",
        "role=\"separator\"",
        "aria-orientation=\"horizontal\"",
    ] {
        assert!(html.contains(expected), "{html}");
    }
}

#[test]
fn host_attributes_pass_through_without_replacing_owned_semantics() {
    let html = render_element(rsx! { DropdownMenuDivider {
        id: "actions-divider", class: "custom", style: "margin-block:8px",
        "data-testid": "separator", role: "button", "aria-orientation": "vertical", tabindex: "0",
        contenteditable: "true",
    } });
    assert!(html.contains("id=\"actions-divider\"") && html.contains("data-testid=\"separator\""));
    assert!(html.contains("custom") && html.contains("margin-block:8px"));
    assert!(
        html.contains("role=\"separator\"") && html.contains("aria-orientation=\"horizontal\"")
    );
    for forbidden in [
        "role=\"button\"",
        "aria-orientation=\"vertical\"",
        "tabindex=",
        "contenteditable=",
    ] {
        assert!(!html.contains(forbidden), "{html}");
    }
}
