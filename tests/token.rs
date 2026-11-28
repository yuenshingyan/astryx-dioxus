use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{InteractiveRole, InteractiveRoleProvider, Token, TokenColor, TokenSize};

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

#[test]
fn disabled_href_yields_to_the_inherited_button_role() {
    let html = render_element(rsx! {
        InteractiveRoleProvider { role: InteractiveRole::Button,
            Token { label: "Enabled destination", href: "#enabled" }
            Token { label: "Disabled trigger", href: "#disabled", is_disabled: true }
        }
    });
    assert_eq!(html.matches("<a ").count(), 1, "{html}");
    assert!(html.contains("href=\"#enabled\""), "{html}");
    assert!(!html.contains("href=\"#disabled\""), "{html}");
    assert!(html.contains("data-disabled=\"disabled\""), "{html}");

    let (_, button) = html.split_once("<button ").expect(&html);
    let (attributes, content) = button.split_once('>').expect(&html);
    assert!(attributes.contains("type=\"button\""), "{html}");
    assert!(attributes.contains("aria-disabled=\"true\""), "{html}");
    assert!(
        !attributes.split_whitespace().any(|attr| {
            attr == "disabled" || attr.starts_with("disabled=") || attr == "tabindex=\"-1\""
        }),
        "{html}"
    );
    assert!(
        content
            .split_once("</button>")
            .expect(&html)
            .0
            .contains("Disabled trigger"),
        "{html}"
    );
}
