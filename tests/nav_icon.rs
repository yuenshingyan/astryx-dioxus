use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::NavIcon;

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
    for unexpected in ["role=", "tabindex=", "aria-label=", "aria-hidden="] {
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
