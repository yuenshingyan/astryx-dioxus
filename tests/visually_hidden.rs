use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{VisuallyHidden, VisuallyHiddenRenderProps};

#[path = "../examples/showcase/visually_hidden.rs"]
mod showcase;

use showcase::VisuallyHiddenExamples;

#[test]
fn default_span_clips_without_implicit_accessibility_or_focus_attributes() {
    let html = render_element(rsx! { VisuallyHidden { "Build state" } });
    assert!(html.starts_with("<span "), "{html}");
    assert!(html.ends_with("Build state</span>"), "{html}");
    for expected in [
        "position:absolute",
        "width:1px",
        "height:1px",
        "margin:-1px",
        "padding:0",
        "overflow:hidden",
        "clip:rect(0,0,0,0)",
        "white-space:nowrap",
        "border-style:none",
        "inset-block-start:0",
        "inset-inline-start:0",
        "pointer-events:none",
        "user-select:none",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    for absent in [
        "role=",
        "aria-",
        " hidden",
        "inert",
        "tabindex",
        "display:",
        "visibility:",
    ] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn fixed_clip_and_native_attributes_survive_ssr() {
    let html = render_element(rsx! {
        VisuallyHidden {
            id: "message", role: "status", aria_live: "polite", aria_atomic: "true", "data-test": "kept",
            style: "display:none", class: "spoof", "className": "raw-spoof", "xstyle": "compat",
            display: "none", position: "static", visibility: "hidden", pointer_events: "auto",
            "Update"
        }
    });
    assert!(html.starts_with("<span"), "{html}");
    for expected in [
        "id=\"message\"",
        "role=\"status\"",
        "aria-live=\"polite\"",
        "aria-atomic=\"true\"",
        "data-test=\"kept\"",
        "clip:rect(0,0,0,0)",
        "Update",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    for absent in [
        "display:",
        "position:static",
        "visibility:",
        "pointer-events:auto",
        "spoof",
        "compat",
        "xstyle",
    ] {
        assert!(!html.contains(absent), "{html}");
    }
    assert_eq!(html.matches(" style=").count(), 1, "{html}");
}

#[test]
fn callers_can_explicitly_hide_content_from_assistive_technology() {
    let html = render_element(
        rsx! { VisuallyHidden { hidden: true, inert: true, aria_hidden: "true", "Removed" } },
    );
    for attribute in [" hidden=", " inert=", " aria-hidden=\"true\""] {
        assert!(html.contains(attribute), "{html}");
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

#[test]
fn showcase_renders_a_hidden_label_and_separate_polite_result_count() {
    let html = render_element(rsx! { VisuallyHiddenExamples {} });
    let (before_button_id, after_button_id) =
        html.split_once("id=\"vh-filter\"").expect("filter button");
    let (_, button_attrs) = before_button_id
        .rsplit_once("<button")
        .expect("native button root");
    let (button, _) = after_button_id
        .split_once("</button>")
        .expect("end of filter button");
    assert!(button.contains("aria-hidden=\"true\""), "{button}");
    assert!(button.contains("clip:rect(0,0,0,0)"), "{button}");
    assert!(
        button.ends_with("Show passing builds only</span>"),
        "{button}"
    );
    assert!(
        !button_attrs.contains("aria-label") && !button.contains("aria-label"),
        "{button_attrs}{button}"
    );
    let (before_region_id, after_region_id) =
        html.split_once("id=\"vh-results\"").expect("result count");
    let (_, region_attrs) = before_region_id.rsplit_once("<div").expect("block root");
    let (region, _) = after_region_id
        .split_once("</div>")
        .expect("end of block root");
    let region = format!("{region_attrs}{region}");
    assert!(
        region.contains("aria-live=\"polite\"") && region.ends_with(">2 build results"),
        "{region}"
    );
}
