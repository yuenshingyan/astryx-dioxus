use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::TopNavHeading;

#[test]
fn logo_only_link_has_its_own_name_and_no_text_track() {
    let html = render_element(
        rsx! { TopNavHeading { logo: rsx! { span { "Logo" } }, logo_label: "Home", heading_href: "/", subheading: "Not shown" } },
    );
    assert!(
        html.starts_with("<a ")
            && html.contains("aria-label=\"Home\"")
            && html.contains("href=\"/\""),
        "{html}"
    );
    assert!(
        !html.contains("Not shown") && !html.contains("__text"),
        "{html}"
    );
}

#[test]
fn text_and_logo_links_keep_separate_destinations() {
    let html = render_element(
        rsx! { TopNavHeading { heading: "Product", logo: rsx! { span { "Logo" } }, heading_href: "/product", superheading: "Suite", superheading_href: "/suite" } },
    );
    assert!(html.starts_with("<div"), "{html}");
    assert_eq!(html.matches("href=\"/product\"").count(), 2, "{html}");
    assert_eq!(html.matches("href=\"/suite\"").count(), 1, "{html}");
    assert!(html.contains("aria-label=\"Product\""), "{html}");
}

#[test]
fn trailing_control_is_not_inside_link() {
    let html = render_element(
        rsx! { TopNavHeading { heading: "Product", heading_href: "/product", header_end_content: rsx! { button { "Settings" } } } },
    );
    assert!(
        html.starts_with("<div")
            && html.contains("</a><span class=\"astryx-top-nav-heading__end\"><button"),
        "{html}"
    );
}
