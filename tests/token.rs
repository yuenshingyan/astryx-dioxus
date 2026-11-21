use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Token, TokenColor, TokenSize};

#[test]
fn static_token_preserves_label_and_theme_identifiers() {
    let color = TokenColor::new("custom-blue").unwrap();
    let html = render_element(rsx! { Token { label: "مرحبا", color, size: TokenSize::Large } });
    for value in ["data-color=\"custom-blue\"", "data-size=\"lg\"", "مرحبا"] {
        assert!(html.contains(value), "{html}");
    }
    assert!(!html.contains("<button") && !html.contains("<a ") && !html.contains("dir="));
    assert!(TokenColor::new("bad color").is_err());
}

#[test]
fn disabled_link_and_remove_remain_separate_focusable_controls() {
    let mut dom = VirtualDom::new(
        || rsx! { Token { label: "Destination", href: "#destination", is_disabled: true, on_remove: |_| {} } },
    );
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    assert!(
        html.contains("</a>") && html.find("</a>") < html.find("<button"),
        "{html}"
    );
    assert!(html.contains("role=\"link\"") && html.contains("tabindex=\"0\""));
    assert!(!html.contains("href="));
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 2);
}
