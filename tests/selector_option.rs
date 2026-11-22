use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{ItemLayout, SelectorOption};

#[test]
fn delegates_to_one_passive_item_root_with_caller_content() {
    let html = render_element(rsx! {
        SelectorOption { label: rsx! { strong { "Editor" } }, description: Some("Can publish".into()),
            icon: rsx! { span { "Icon" } }, end_content: rsx! { span { "Default" } },
            layout: ItemLayout::Inline, class: "caller", style: "padding:1px",
        }
    });
    assert!(
        html.starts_with("<div class=\"astryx-item astryx-selector-option caller\""),
        "{html}"
    );
    assert!(
        html.contains("data-layout=\"inline\"") && html.contains("style=\"padding:1px\""),
        "{html}"
    );
    let slots = ["Icon", "<strong>Editor</strong>", "Can publish", "Default"]
        .map(|text| html.find(text).unwrap());
    assert!(slots.windows(2).all(|pair| pair[0] < pair[1]), "{html}");
    assert!(
        !html.contains("tabindex") && !html.contains("role="),
        "{html}"
    );
}
