use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::InputGroupText;

#[test]
fn plain_text_isolated_without_changing_cell_direction() {
    let prefix = "https://";
    let html = render_element(rsx! { InputGroupText { id: "prefix", dir: "rtl", "{prefix}" } });
    assert!(
        html.contains("data-astryx-input-group-item=\"\"") && html.contains("dir=\"rtl\""),
        "{html}"
    );
    assert!(html.contains("<span dir=\"ltr\">https://</span>"), "{html}");
}

#[test]
fn rich_and_rtl_content_keep_their_direction() {
    let html =
        render_element(rsx! { InputGroupText { strong { "USD" } } InputGroupText { "السعر" } });
    assert!(
        html.contains("<strong>USD</strong>") && html.contains("السعر") && !html.contains("dir="),
        "{html}"
    );
}
