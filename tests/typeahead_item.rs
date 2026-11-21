use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{TypeaheadItem, TypeaheadItemContent};

#[test]
fn default_layout_forwards_attributes_without_option_semantics() {
    let html = render_element(rsx! {
        TypeaheadItem { item: TypeaheadItemContent { label: "Alice".into(), element: None },
            description: "Engineer", is_disabled: true, id: "alice", class: "custom",
            icon: rsx! { span { "Artwork" } },
        }
    });
    assert!(html.contains("astryx-typeahead-item custom"));
    assert!(html.contains("id=\"alice\"") && html.contains("data-disabled=\"true\""));
    assert!(html.contains("<span>Artwork</span><div class=\"astryx-typeahead-item__content\">"));
    assert!(html.contains(">Engineer</span>"));
    assert!(
        !html.contains("role=") && !html.contains("aria-disabled=") && !html.contains("tabindex=")
    );
}

#[test]
fn replacement_owns_the_complete_rendered_root() {
    let html = render_element(rsx! {
        TypeaheadItem { id: "default-root", item: TypeaheadItemContent {
            label: "Default".into(), element: Some(rsx! { strong { "Custom result" } }),
        } }
    });
    assert_eq!(html, "<strong>Custom result</strong>");
}
