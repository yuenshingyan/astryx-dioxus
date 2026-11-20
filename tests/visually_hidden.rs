use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{VisuallyHidden, VisuallyHiddenRenderProps};

#[test]
fn fixed_clip_and_native_attributes_survive_ssr() {
    let html = render_element(rsx! {
        VisuallyHidden { id: "message", "aria-live": "polite", "data-test": "kept", style: "display:none", class: "spoof", "Update" }
    });
    assert!(html.starts_with("<span"), "{html}");
    for expected in [
        "id=\"message\"",
        "aria-live=\"polite\"",
        "data-test=\"kept\"",
        "clip:rect(0,0,0,0)",
        "Update",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    for absent in ["display:none", "spoof", "aria-hidden", "tabindex"] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn caller_renderer_keeps_heading_semantics_without_a_wrapper() {
    fn app() -> Element {
        rsx! {
            VisuallyHidden { as_element: Callback::new(|root: VisuallyHiddenRenderProps| rsx! { h2 { ..root.attributes, {root.children} } }), "Build state" }
        }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    assert!(
        html.starts_with("<h2")
            && html.contains("clip:rect(0,0,0,0)")
            && html.ends_with("Build state</h2>"),
        "{html}"
    );
    assert!(!html.contains("<span"), "{html}");
}
