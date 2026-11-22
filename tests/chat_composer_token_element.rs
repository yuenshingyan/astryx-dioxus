use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{BadgeVariant, ChatComposerToken, ChatComposerTokenElement};

#[test]
fn preserves_serialized_value_and_owns_editability() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ChatComposerTokenElement {
                token: ChatComposerToken::Badge { value: "@ada#2".into(), label: "Ada".into(), variant: BadgeVariant::BLUE, icon: None },
                class: "example-token", style: "margin-inline:2px", id: "person-token", contenteditable: "true",
                "data-astryx-token-value": "wrong",
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for fragment in [
        "data-astryx-token-value=\"@ada#2\"",
        "contenteditable=\"false\"",
        "example-token",
        "margin-inline:2px",
        "person-token",
        "data-variant=\"blue\"",
        "Ada",
    ] {
        assert!(html.contains(fragment), "{html}");
    }
    assert!(
        !html.contains("wrong") && !html.contains("role=") && !html.contains("tabindex"),
        "{html}"
    );
}
