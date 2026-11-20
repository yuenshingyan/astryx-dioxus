use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    SizeValue, SpacingStep, Stack, StackCrossAlignment, StackDirection, StackMainAlignment,
    StackRenderProps, StackWrap, VStack,
};

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
