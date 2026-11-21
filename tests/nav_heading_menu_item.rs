use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::NavHeadingMenuItem;

#[test]
fn standalone_link_retains_destination_and_menu_semantics() {
    let html = render_element(rsx! {
        NavHeadingMenuItem { label: rsx! { "Dashboard" }, href: "/dashboard", role: "button", tabindex: "0" }
    });
    assert!(html.starts_with("<a "), "{html}");
    for expected in [
        "href=\"/dashboard\"",
        "role=\"menuitem\"",
        "tabindex=\"-1\"",
        "draggable=\"false\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("role=\"button\"") && !html.contains("tabindex=\"0\""));
}

#[test]
fn disabled_links_keep_the_row_but_drop_the_destination() {
    let html = render_element(rsx! {
        NavHeadingMenuItem { label: rsx! { "Settings" }, href: "/settings", is_disabled: true }
    });
    assert!(
        html.starts_with("<a ") && html.contains("aria-disabled=\"true\""),
        "{html}"
    );
    assert!(!html.contains("href="), "{html}");
}
