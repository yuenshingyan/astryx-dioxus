use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::BreadcrumbItem;

#[test]
fn current_page_is_a_nonlink_and_attributes_reach_the_item() {
    let html = render_element(rsx! { ol {
        BreadcrumbItem { href: "/settings", "Settings" }
        BreadcrumbItem { href: "/profile", is_current: true, id: "profile", "Profile" }
    } });
    assert_eq!(html.matches("<a ").count(), 1, "{html}");
    assert!(html.contains("href=\"/settings\"") && !html.contains("href=\"/profile\""));
    assert!(html.contains("id=\"profile\"") && html.contains("aria-current=\"page\">Profile"));
    assert_eq!(html.matches("aria-hidden=\"true\"").count(), 2);
}

#[test]
fn unsafe_destination_is_not_emitted() {
    let html = render_element(
        rsx! { BreadcrumbItem { href: "javascript:void(0)", is_current: false, "Invalid" } },
    );
    assert!(
        !html.contains("href=") && html.contains("aria-disabled=\"true\""),
        "{html}"
    );
}
