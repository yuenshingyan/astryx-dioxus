use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{LinkProvider, LinkRenderProps, SideNav, SideNavHeading};

#[test]
fn static_and_whole_link_use_native_roots() {
    let html = render_element(
        rsx! { SideNavHeading { heading: "Analytics", superheading: "Acme", subheading: "Production" } },
    );
    assert!(html.starts_with("<div") && !html.contains("<a "), "{html}");
    assert!(
        html.contains("<span class=\"astryx-side-nav-heading__title\">Analytics</span>")
            && !html.contains("role=\"heading\""),
        "{html}"
    );
    let html = render_element(rsx! {
        SideNav { label: "Analytics navigation",
            header: rsx! {
                SideNavHeading { id: "heading", heading: "Analytics", heading_href: "/analytics",
                    icon: rsx! { svg { "aria-hidden": "true" } },
                }
            },
            a { href: "/reports", "Reports" }
        }
    });
    let header = html
        .split_once("class=\"astryx-side-nav__scrollable\"")
        .unwrap()
        .0;
    assert!(
        header.contains("class=\"astryx-side-nav__top\"><a ")
            && header.contains("href=\"/analytics\"")
            && header.contains("id=\"heading\"")
            && header.contains("<svg")
            && header.contains("Analytics</span></span></a>"),
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
    let html = render_element(rsx! {
        SideNavHeading { heading: "Analytics", heading_href: "/analytics", icon: rsx! { "Mark" },
            superheading: "Acme", superheading_href: "/acme",
            subheading: "Production", subheading_href: "/production",
            header_end_content: rsx! { button { "Settings" } },
        }
    });
    assert!(html.starts_with("<div"), "{html}");
    let destinations = ["/analytics", "/acme", "/analytics", "/production"];
    assert_eq!(html.matches("</a>").count(), destinations.len(), "{html}");
    for (segment, destination) in html.split("</a>").zip(destinations) {
        assert_eq!(segment.matches("<a ").count(), 1, "{html}");
        assert!(
            segment.contains(&format!("href=\"{destination}\"")),
            "{html}"
        );
        assert!(!segment.contains("<button"), "{html}");
    }
    assert!(html.contains("aria-label=\"Analytics\">Mark</a>"), "{html}");
}

#[test]
fn empty_destinations_do_not_create_or_split_links() {
    let html = render_element(rsx! {
        SideNavHeading { heading: "Analytics", heading_href: "",
            superheading: "Acme", superheading_href: "", subheading: "Production", subheading_href: "",
        }
    });
    assert!(html.starts_with("<div") && !html.contains("<a "), "{html}");
    let html = render_element(rsx! {
        SideNavHeading { heading: "Analytics", heading_href: "/analytics",
            superheading: "Acme", superheading_href: "", subheading: "Production", subheading_href: "",
            header_end_content: rsx! { if false { button { "Settings" } } },
        }
    });
    assert!(html.starts_with("<a ") && html.ends_with("</a>"), "{html}");
    assert_eq!(html.matches("<a ").count(), 1, "{html}");
    assert!(
        html.contains("href=\"/analytics\"") && !html.contains("href=\"\""),
        "{html}"
    );
}

#[test]
fn only_whole_and_icon_links_use_the_heading_renderer_override() {
    let html = render_element(rsx! { RendererCases {} });
    assert_eq!(
        html.matches("data-router=\"explicit\"").count(),
        2,
        "{html}"
    );
    assert_eq!(
        html.matches("data-router=\"provider\"").count(),
        3,
        "{html}"
    );
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 2, "{html}");
    assert!(!html.contains("javascript:"), "{html}");
}

#[component]
fn RendererCases() -> Element {
    let provider = |props: LinkRenderProps| {
        rsx! { a { href: props.href, "data-router": "provider", ..props.attributes, {props.children} } }
    };
    let explicit = |props: LinkRenderProps| {
        rsx! { a { href: props.href, "data-router": "explicit", ..props.attributes, {props.children} } }
    };
    rsx! {
        LinkProvider { component: provider,
            SideNavHeading { heading: "Whole", heading_href: "/whole", render_link: explicit }
            SideNavHeading { heading: "Split", heading_href: "/product", render_link: explicit,
                icon: rsx! { "Mark" }, superheading: "Suite", superheading_href: "/suite",
                subheading: "Account", subheading_href: "/account",
            }
            SideNavHeading { heading: "Blocked whole", heading_href: "javascript:blocked()", render_link: explicit }
            SideNavHeading { heading: "Static title", superheading: "Blocked line", superheading_href: "javascript:blocked()" }
        }
    }
}
