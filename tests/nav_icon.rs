use dioxus::core::{AttributeValue, Mutation};
use dioxus::events::onmounted;
use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{NavIcon, TopNavHeading};

#[test]
fn root_forwards_props_without_adding_semantics() {
    let html = render_element(rsx! {
        NavIcon { icon: rsx! { span { role: "img", "aria-label": "Acme", "A" } },
            class: "custom", style: "width:40px", id: "mark", "data-testid": "product-mark",
        }
    });
    let (root, children) = html.split_once('>').expect("root span");
    assert!(root.starts_with("<span "), "{html}");
    for expected in [
        "class=\"astryx-nav-icon custom\"",
        "style=\"width:40px\"",
        "id=\"mark\"",
        "data-testid=\"product-mark\"",
    ] {
        assert!(root.contains(expected), "missing {expected}: {html}");
    }
    for unexpected in [" role=", " tabindex=", " aria-"] {
        assert!(
            !root.contains(unexpected),
            "unexpected {unexpected}: {html}"
        );
    }
    assert!(
        children.contains("role=\"img\" aria-label=\"Acme\""),
        "{html}"
    );
}

#[test]
fn root_receives_the_mount_listener_not_the_artwork() {
    let mut dom = VirtualDom::new(|| {
        rsx! { NavIcon { id: "mark", attributes: vec![onmounted(|_| {})], icon: rsx! { span { "A" } } } }
    });
    let mutations = dom.rebuild_to_vec();
    let root = mutations.edits.iter().find_map(|edit| match edit {
        Mutation::SetAttribute {
            name: "id",
            value: AttributeValue::Text(value),
            id,
            ..
        } if value == "mark" => Some(*id),
        _ => None,
    });
    let root = root.expect("NavIcon root ID mutation");
    let listeners: Vec<_> = mutations
        .edits
        .iter()
        .filter_map(|edit| match edit {
            Mutation::NewEventListener { name, id } if name == "mounted" => Some(*id),
            _ => None,
        })
        .collect();
    assert_eq!(listeners, [root]);
}

#[test]
fn decorative_logo_preserves_the_heading_link_and_its_current_state() {
    let html = render_element(rsx! {
        TopNavHeading { heading: "Workspace", heading_href: "/workspace", "aria-current": "page",
            logo: rsx! { NavIcon { icon: rsx! { span { "aria-hidden": "true", "W" } } } },
        }
    });
    let (link, content) = html.split_once('>').expect("heading link");
    assert!(link.starts_with("<a ") && html.ends_with("</a>"), "{html}");
    assert!(link.contains("href=\"/workspace\""), "{html}");
    assert!(link.contains("aria-current=\"page\""), "{html}");
    assert!(!link.contains("aria-label="), "{html}");
    assert!(content.contains(">Workspace</span>"), "{html}");
    assert!(
        content
            .contains("<span class=\"astryx-nav-icon\"><span aria-hidden=\"true\">W</span></span>"),
        "{html}"
    );
    for unexpected in [
        "<a ",
        "<button",
        " tabindex=",
        " aria-current=",
        " aria-disabled=",
    ] {
        assert!(
            !content.contains(unexpected),
            "unexpected {unexpected}: {html}"
        );
    }
}

#[test]
fn logo_only_link_preserves_meaningful_artwork_without_labeling_the_wrapper() {
    let html = render_element(rsx! {
        TopNavHeading { heading_href: "/workspace",
            logo: rsx! { NavIcon { icon: rsx! { img { alt: "Workspace home", src: "/logo.svg" } } } },
        }
    });
    let (link, content) = html.split_once('>').expect("logo-only link");
    assert!(link.starts_with("<a ") && html.ends_with("</a>"), "{html}");
    assert!(link.contains("href=\"/workspace\""), "{html}");
    assert!(
        !html.contains("aria-label=") && !html.contains("aria-hidden="),
        "{html}"
    );
    assert!(
        content.contains("<span class=\"astryx-nav-icon\"><img "),
        "{html}"
    );
    assert!(content.contains("alt=\"Workspace home\""), "{html}");
}
