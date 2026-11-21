use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::SideNavHeading;

#[test]
fn static_and_whole_link_use_native_roots() {
    let html = render_element(
        rsx! { SideNavHeading { heading: "Analytics", superheading: "Acme", subheading: "Production" } },
    );
    assert!(html.starts_with("<div") && !html.contains("<a "), "{html}");
    let html = render_element(
        rsx! { SideNavHeading { id: "heading", heading: "Analytics", heading_href: "/analytics" } },
    );
    assert!(
        html.starts_with("<a ")
            && html.contains("href=\"/analytics\"")
            && html.contains("id=\"heading\""),
        "{html}"
    );
}

#[test]
fn trailing_action_is_not_inside_the_link() {
    let html = render_element(
        rsx! { SideNavHeading { heading: "Analytics", heading_href: "/analytics",
            header_end_content: rsx! { button { "Settings" } }
        } },
    );
    assert!(
        html.starts_with("<div")
            && html.contains("</a><span class=\"astryx-side-nav-heading__end\"><button"),
        "{html}"
    );
}

#[test]
fn separate_destinations_do_not_nest_links() {
    let html = render_element(
        rsx! { SideNavHeading { heading: "Analytics", heading_href: "/analytics", superheading: "Acme", superheading_href: "/acme" } },
    );
    assert!(html.starts_with("<div"), "{html}");
    assert_eq!(html.matches("<a ").count(), 2, "{html}");
    assert!(html.find("</a>") < html.find("href=\"/analytics\""));
}
