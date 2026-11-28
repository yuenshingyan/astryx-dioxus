use dioxus::prelude::*;

#[path = "../examples/showcase/stack_item.rs"]
mod showcase;

use astryx_dioxus::{
    StackItem, StackItemCrossAlignSelf, StackItemRoot, StackItemSize, StackRenderProps,
};
use showcase::StackItemExamples;

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

#[test]
fn overflow_semantics_are_deferred_without_losing_the_caller_label() {
    let html = render(|| {
        rsx! { StackItem { root: StackItemRoot::Span, is_scrollable: true, "aria-label": "Items" } }
    });
    assert!(html.starts_with("<span"), "{html}");
    assert!(html.contains("aria-label=\"Items\""), "{html}");
    assert!(
        !html.contains("role=") && !html.contains("tabindex="),
        "{html}"
    );
}

#[test]
fn showcase_renders_the_pinned_static_fill_sequences_and_all_alignment_choices() {
    let html = render(StackItemExamples);
    for (id, sizes, labels, width) in [
        (
            "showcase",
            &["static", "fill", "static"][..],
            &["Static Width", "Fills remaining space", "Static Width"][..],
            Some("500px"),
        ),
        (
            "fill",
            &["static", "fill"][..],
            &["Static", "Fills remaining space"][..],
            Some("400px"),
        ),
        (
            "source-example",
            &["static", "fill", "static"][..],
            &["Logo", "Content", "Actions"][..],
            None,
        ),
    ] {
        let (before_id, _) = html.split_once(&format!("id=\"stack-item-{id}\"")).unwrap();
        let tag_start = before_id.rfind('<').unwrap();
        let row = html[tag_start..].split_once("<h3>").unwrap().0;
        assert_eq!(row.matches("data-size=\"").count(), sizes.len(), "{row}");
        let mut remaining = row;
        for (size, label) in sizes.iter().zip(labels) {
            remaining = remaining
                .split_once(&format!("data-size=\"{size}\""))
                .unwrap()
                .1;
            remaining = remaining.split_once(label).unwrap().1;
        }
        if let Some(width) = width {
            assert!(row.contains(&format!("max-width:{width}")), "{row}");
        }
    }
    for alignment in ["start", "center", "end", "stretch"] {
        assert!(
            html.contains(&format!("astryx-stack-item--{alignment}")),
            "{alignment}"
        );
    }
}
