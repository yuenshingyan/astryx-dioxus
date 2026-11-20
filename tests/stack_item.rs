use dioxus::prelude::*;

use astryx_dioxus::{
    StackItem, StackItemCrossAlignSelf, StackItemRoot, StackItemSize, StackRenderProps,
};

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn defaults_to_one_nonsemantic_static_item() {
    let html = render(|| rsx! { StackItem { button { "First" } a { href: "#second", "Second" } } });
    assert_eq!(html.matches("<div").count(), 1);
    assert!(html.contains("astryx-stack-item"));
    assert!(html.find("First") < html.find("Second"));
    for absent in [
        "role=",
        "aria-",
        "tabindex=",
        "data-size=",
        "data-astryx-item-scroll",
    ] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn custom_root_preserves_caller_semantics_and_styles_even_when_scrollable() {
    let html = render(|| {
        rsx! { StackItem {
            root: StackItemRoot::Custom(Callback::new(|props: StackRenderProps| rsx! { article { ..props.attributes, {props.children} } })),
            size: StackItemSize::Fill, cross_align_self: StackItemCrossAlignSelf::End, is_scrollable: true,
            class: "caller", style: "overflow:auto", id: "root", "aria-label": "Messages", "Content"
        } }
    });
    assert!(html.starts_with("<article"), "{html}");
    for expected in [
        "data-size=\"fill\"",
        "astryx-stack-item--end",
        "astryx-stack-item--scrollable",
        "caller",
        "style=\"overflow:auto\"",
        "id=\"root\"",
        "aria-label=\"Messages\"",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert!(!html.contains("data-astryx-item-scroll"));
}

#[test]
fn caller_role_wins_on_scrollable_span() {
    let html = render(|| {
        rsx! { StackItem { root: StackItemRoot::Span, is_scrollable: true,
            role: "region", "Items"
        } }
    });
    assert!(html.starts_with("<span"), "{html}");
    assert!(html.contains("role=\"region\""));
    assert!(!html.contains("aria-label") && !html.contains("data-astryx-item-scroll"));
}
