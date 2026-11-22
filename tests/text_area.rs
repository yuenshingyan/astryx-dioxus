use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::TextArea;

#[test]
fn textarea_renders_initial_text_and_native_limit() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            TextArea { label: "Notes", id: "notes", value: "First\nSecond", html_max_length: 20, is_read_only: true }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(
        html.contains("for=\"notes\"") && html.contains("id=\"notes\""),
        "{html}"
    );
    assert!(
        html.contains("readonly") && html.contains("maxlength=20"),
        "{html}"
    );
    assert!(html.contains(">First\nSecond</textarea>"), "{html}");
}
