use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{TextInput, TextInputType};

#[test]
fn disabled_search_keeps_a_label_but_has_no_form_name() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            TextInput { id: "query", label: "Search", value: "Example", input_type: TextInputType::Search,
                is_label_hidden: true, description: "Search the catalog.", html_name: "query", is_disabled: true }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    for fragment in [
        "for=\"query\"",
        "type=\"search\"",
        "readonly",
        "aria-disabled=\"true\"",
        "aria-describedby=\"query-desc\"",
    ] {
        assert!(html.contains(fragment), "{html}");
    }
    assert!(!html.contains("name=\"query\""), "{html}");
}
