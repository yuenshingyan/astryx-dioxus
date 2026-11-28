use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{LinkProvider, LinkRenderProps, TopNav, TopNavHeading};

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
    let html = render_element(rsx! {
        TopNavHeading {
            heading: "Product", logo: rsx! { span { "Logo" } }, logo_label: "  ", heading_href: "/product",
            superheading: "Suite", superheading_href: "/suite", subheading: "Account", subheading_href: "/account",
            header_end_content: rsx! { button { "Settings" } },
        }
    });
    assert!(html.starts_with("<div"), "{html}");
    assert_eq!(html.matches("href=\"/product\"").count(), 2, "{html}");
    assert_eq!(html.matches("href=\"/suite\"").count(), 1, "{html}");
    assert_eq!(html.matches("href=\"/account\"").count(), 1, "{html}");
    assert!(html.contains("aria-label=\"Product\""), "{html}");
    assert!(html.rfind("</a>") < html.find("<button"), "{html}");
}

#[test]
fn only_visible_destinations_can_split_the_whole_link() {
    let html = render_element(rsx! {
        TopNavHeading {
            heading: "Product", heading_href: "#product", logo: rsx! { img { src: "/logo.svg", alt: "" } },
            superheading: "Suite", superheading_href: "", subheading_href: "#unused",
        }
    });
    assert!(html.starts_with("<a ") && html.ends_with("</a>"), "{html}");
    assert_eq!(html.matches("<a ").count(), 1, "{html}");
    assert_eq!(html.matches("href=\"#product\"").count(), 1, "{html}");
    assert!(
        html.contains("<img") && html.contains("Suite") && html.contains("Product"),
        "{html}"
    );
}

#[test]
fn empty_href_leaves_identity_as_text_not_a_document_heading() {
    let html = render_element(rsx! {
        TopNav { label: "Workspace navigation", heading: rsx! {
            TopNavHeading { heading: "Product", heading_href: "", superheading: "Suite", subheading: "Account" }
        } }
    });
    assert!(
        html.starts_with("<nav ") && html.contains("aria-label=\"Workspace navigation\""),
        "{html}"
    );
    assert!(
        html.contains("Product") && html.contains("Suite") && html.contains("Account"),
        "{html}"
    );
    assert!(
        !html.contains("<a ")
            && !html.contains("<button")
            && !html.contains("<h")
            && !html.contains("role=\"heading\""),
        "{html}"
    );
}

#[test]
fn logo_only_with_blank_label_can_use_image_alt() {
    let html = render_element(rsx! {
        TopNavHeading { logo: rsx! { img { src: "/logo.svg", alt: "Product home" } }, logo_label: " ", heading_href: "/" }
    });
    assert!(
        html.starts_with("<a ")
            && html.contains("alt=\"Product home\"")
            && !html.contains("aria-label"),
        "{html}"
    );
}

fn provider_link(props: LinkRenderProps) -> Element {
    rsx! { a { href: props.href, "data-renderer": "provider", ..props.attributes, {props.children} } }
}

fn explicit_link(props: LinkRenderProps) -> Element {
    rsx! { a { href: props.href, "data-renderer": "explicit", ..props.attributes, {props.children} } }
}

#[test]
fn explicit_renderer_overrides_provider_on_logo_and_whole_link_but_not_inline_links() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            LinkProvider { component: provider_link,
                TopNavHeading { heading: "Whole", heading_href: "/whole", render_link: explicit_link, class: "shell", style: "width:123px", id: "whole" }
                TopNavHeading { heading: "Product", heading_href: "/product", superheading: "Suite", superheading_href: "/suite",
                    logo: rsx! { img { src: "/logo.svg", alt: "" } }, logo_label: "Home", render_link: explicit_link,
                }
                TopNavHeading { heading: "Blocked", heading_href: "javascript:blocked", render_link: explicit_link }
            }
        }
    });
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    assert_eq!(
        html.matches("data-renderer=\"explicit\"").count(),
        2,
        "{html}"
    );
    assert_eq!(
        html.matches("data-renderer=\"provider\"").count(),
        2,
        "{html}"
    );
    assert!(
        html.contains("href=\"/whole\" data-renderer=\"explicit\"")
            && html.contains("id=\"whole\"")
            && html.contains("astryx-top-nav-heading shell")
            && html.contains("style=\"width:123px\""),
        "{html}"
    );
    assert!(
        html.contains("href=\"/product\" data-renderer=\"explicit\"")
            && html.contains("aria-label=\"Home\""),
        "{html}"
    );
    assert!(
        html.contains("aria-disabled=\"true\"") && !html.contains("javascript:"),
        "{html}"
    );
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
