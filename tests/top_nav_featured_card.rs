use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::TopNavMegaMenuFeaturedCard;

#[test]
fn decorative_image_and_incomplete_link() {
    let html = render_element(rsx! {
        TopNavMegaMenuFeaturedCard { id: "featured", title: "News", image: "/scene.png", link_label: "Read",
            span { "Extra" }
        }
    });
    for expected in [
        "id=\"featured\"",
        "alt=\"\"",
        "role=\"presentation\"",
        "aria-hidden=\"true\"",
        "Extra",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("<a "), "{html}");
}

#[test]
fn informative_image_and_complete_link() {
    let html = render_element(rsx! {
        TopNavMegaMenuFeaturedCard { title: "News", image: "/scene.png", image_alt: "City skyline", link_label: "Read", link_href: "/news" }
    });
    assert!(
        html.contains("alt=\"City skyline\"") && !html.contains("presentation"),
        "{html}"
    );
    assert!(
        html.contains("href=\"/news\"") && html.contains("aria-hidden=\"true\">→"),
        "{html}"
    );
}
