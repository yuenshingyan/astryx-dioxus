use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::MetadataListItem;

#[test]
fn inline_term_and_value_are_direct_siblings() {
    let html = render_element(rsx! { dl {
        MetadataListItem { label: "Owner", id: "term", test_id: "owner",
            icon: rsx! { span { "Icon" } }, a { href: "#", "Alice" }
        }
    } });
    assert!(
        html.starts_with("<dl><dt") && html.ends_with("</dd></dl>"),
        "{html}"
    );
    assert!(
        html.contains("data-testid=\"owner-label\" id=\"term\""),
        "{html}"
    );
    assert!(
        html.contains("</dt><dd") && html.contains("data-testid=\"owner-value\"><a"),
        "{html}"
    );
    assert!(html.contains("astryx-metadata-list-item__icon"));
}
