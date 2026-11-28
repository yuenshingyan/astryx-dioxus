use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::FileInput;

#[test]
fn loading_keeps_the_native_file_input_available_to_the_form() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            form {
                FileInput { label: "Attachment", html_name: "attachment", is_required: true, is_loading: true }
            }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    let input = html
        .split('<')
        .find(|tag| tag.starts_with("input "))
        .expect("expected a native file input");

    for fragment in [
        "type=\"file\"",
        "name=\"attachment\"",
        " required",
        "aria-busy=\"true\"",
    ] {
        assert!(input.contains(fragment), "{input}");
    }
    for fragment in [" disabled", " tabindex=", " form="] {
        assert!(!input.contains(fragment), "{input}");
    }
}
