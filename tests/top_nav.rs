use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::TopNav;

#[test]
fn explicit_start_wins_and_empty_slots_are_omitted() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            TopNav { start_content: rsx! { "Primary" }, "Ignored alias" }
            TopNav { "Fallback start" }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert!(html.contains("aria-label=\"Top navigation\""), "{html}");
    assert!(
        html.contains("Primary") && html.contains("Fallback start"),
        "{html}"
    );
    assert!(
        !html.contains("Ignored alias") && !html.contains("astryx-top-nav__end"),
        "{html}"
    );
}
