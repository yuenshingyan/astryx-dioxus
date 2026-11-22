use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Item, ItemAlign, ItemTag};

#[test]
fn static_row_preserves_slot_order_and_native_root() {
    let html = render_element(rsx! {
        Item { id: "row", tag: ItemTag::Li, align: ItemAlign::FirstLine,
            marker: rsx! { span { "Marker" } }, start_content: rsx! { span { "Start" } },
            label: rsx! { strong { "Title" } }, description: Some("Details".into()),
            end_content: rsx! { button { "Action" } },
        }
    });
    assert!(
        html.starts_with("<li ") && html.contains("id=\"row\""),
        "{html}"
    );
    assert!(
        !html.contains("tabindex") && html.matches("<button").count() == 1,
        "{html}"
    );
    let positions =
        ["Marker", "Start", "Title", "Details", "Action"].map(|text| html.find(text).unwrap());
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]), "{html}");
}
