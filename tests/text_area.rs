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

#[test]
fn textarea_keeps_icon_and_spinner_outside_the_native_control() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            TextArea {
                label: "Notes",
                value: "Native text",
                start_icon: rsx! { span { "Compose icon" } },
                is_loading: true,
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let (_, frame) = html.split_once("<div class=\"astryx-text-area\"").unwrap();
    let (before, control) = frame.split_once("<textarea").unwrap();
    let (control, after) = control.split_once("</textarea>").unwrap();
    assert!(
        before.contains("class=\"astryx-text-area__icon\""),
        "{html}"
    );
    assert!(control.ends_with(">Native text"), "{html}");
    assert!(
        after.starts_with("<span class=\"astryx-text-area__loading\""),
        "{html}"
    );
}
