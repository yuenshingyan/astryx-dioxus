use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    SizeValue, SpacingStep, Stack, StackCrossAlignment, StackDirection, StackMainAlignment,
    StackRenderProps, StackWrap, VStack,
};

use self::showcase::VStackExamples;

#[path = "../examples/showcase/v_stack.rs"]
mod showcase;

#[test]
fn matches_stack_with_explicit_axis_and_precedence() {
    let actual = render_element(rsx! {
        VStack {
            v_align: StackMainAlignment::Between, justify: StackMainAlignment::End,
            h_align: StackCrossAlignment::Center, align: StackCrossAlignment::Stretch,
            gap: SpacingStep::Two, padding: SpacingStep::Four, padding_inline_start: SpacingStep::Zero,
            width: SizeValue::from(240), height: SizeValue::from("3rem"), wrap: StackWrap::Wrap,
            is_scrollable: true, class: "custom", style: "color:inherit", id: "row",
            button { "First" } a { href: "#row", "Second" }
        }
    });
    let expected = render_element(rsx! {
        Stack {
            direction: StackDirection::Vertical,
            justify: StackMainAlignment::Between, align: StackCrossAlignment::Center,
            gap: SpacingStep::Two, padding: SpacingStep::Four, padding_inline_start: SpacingStep::Zero,
            width: SizeValue::from(240), height: SizeValue::from("3rem"), wrap: StackWrap::Wrap,
            is_scrollable: true, class: "custom", style: "color:inherit", id: "row",
            button { "First" } a { href: "#row", "Second" }
        }
    });
    assert_eq!(actual, expected);
}

#[test]
fn default_root_preserves_child_order_without_inventing_semantics_or_modifiers() {
    let html = render_element(rsx! {
        VStack {
            button { r#type: "button", "First" }
            a { href: "#next", "Second" }
        }
    });
    assert!(html.starts_with("<div "), "{html}");
    assert!(html.contains("data-direction=\"vertical\""), "{html}");
    assert!(
        html.ends_with("<button type=\"button\">First</button><a href=\"#next\">Second</a></div>"),
        "{html}"
    );
    for absent in ["role=", "tabindex=", "aria-", "data-gap=", "data-wrap="] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn native_root_keeps_logical_padding_and_sizing_order() {
    fn app() -> Element {
        rsx! {
            VStack {
                as_element: Callback::new(|props: StackRenderProps| rsx! { nav { ..props.attributes, {props.children} } }),
                id: "topics", dir: "rtl", "aria-label": "Topics",
                padding: SpacingStep::Three, padding_inline: SpacingStep::Two,
                padding_inline_start: SpacingStep::Zero, padding_block_end: SpacingStep::Four,
                style: "width:1px;height:2px;max-width:3px;min-height:4px;color:inherit",
                width: SizeValue::from(240), height: SizeValue::from("7rem"),
                max_width: SizeValue::from("100%"), min_height: SizeValue::from(48),
                a { href: "#summary", "Summary" }
            }
        }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    assert!(html.starts_with("<nav "), "{html}");
    assert!(
        html.ends_with("<a href=\"#summary\">Summary</a></nav>"),
        "{html}"
    );
    for expected in [
        "id=\"topics\"",
        "dir=\"rtl\"",
        "aria-label=\"Topics\"",
        "--_stack-padding-inline-start:var(--spacing-0);",
        "--_stack-padding-inline-end:var(--spacing-2);",
        "--_stack-padding-block-start:var(--spacing-3);",
        "--_stack-padding-block-end:var(--spacing-4);",
        "width:1px;height:2px;max-width:3px;min-height:4px;color:inherit;width:240px;height:7rem;max-width:100%;min-height:48px;",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    for absent in [
        "role=",
        "tabindex=",
        "--container-padding-",
        "astryx-stack--scrollable",
    ] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn showcase_includes_pinned_card_playground_and_item_composition() {
    let html = render_element(rsx! { VStackExamples {} });
    let composition = |id| {
        let (before_id, after_id) = html.split_once(&format!("id=\"{id}\"")).unwrap();
        let (_, opening_before_id) = before_id.rsplit_once('<').unwrap();
        let (opening_after_id, body) = after_id.split_once('>').unwrap();
        assert!(format!("{opening_before_id}{opening_after_id}").contains("data-gap=\"2\""));
        body.split("<h3>").next().unwrap()
    };
    let playground = composition("v-stack-playground");
    let items = composition("v-stack-with-items");
    for (markup, labels) in [
        (playground, &["Item 1", "Item 2", "Item 3"][..]),
        (items, &["Item 1", "Item 2"][..]),
    ] {
        let mut tail = markup;
        for label in labels {
            (_, tail) = tail.split_once(label).unwrap();
        }
    }
    assert_eq!(
        playground
            .matches("--_card-padding:var(--spacing-3)")
            .count(),
        3,
        "{playground}"
    );
    assert_eq!(items.matches("class=\"astryx-item ").count(), 2, "{items}");
}

#[test]
fn aliases_and_native_root_pass_through_without_extra_semantics() {
    fn app() -> Element {
        rsx! {
            VStack {
                justify: StackMainAlignment::End, align: StackCrossAlignment::Start,
                as_element: Callback::new(|props: StackRenderProps| rsx! { section { ..props.attributes, {props.children} } }),
                "data-test": "caller", "Text"
            }
        }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    assert!(html.starts_with("<section"));
    for expected in [
        "astryx-stack--vertical",
        "astryx-stack--justify-end",
        "astryx-stack--align-start",
        "data-test=\"caller\"",
        "Text",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    for absent in ["<div", "role=", "tabindex=", "aria-"] {
        assert!(!html.contains(absent), "{html}");
    }
}
