use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{
    HStack, SizeValue, SpacingStep, Stack, StackCrossAlignment, StackDirection, StackMainAlignment,
    StackRenderProps, StackWrap,
};

#[test]
fn matches_stack_with_explicit_axis_and_precedence() {
    let actual = render_element(rsx! {
        HStack {
            h_align: StackMainAlignment::Between, justify: StackMainAlignment::End,
            v_align: StackCrossAlignment::Center, align: StackCrossAlignment::Stretch,
            gap: SpacingStep::Two, padding: SpacingStep::Four, padding_inline_start: SpacingStep::Zero,
            width: SizeValue::from(240), height: SizeValue::from("3rem"), wrap: StackWrap::Wrap,
            is_scrollable: true, class: "custom", style: "color:inherit", id: "row",
            button { "First" } a { href: "#row", "Second" }
        }
    });
    let expected = render_element(rsx! {
        Stack {
            direction: StackDirection::Horizontal,
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
            HStack {
                justify: StackMainAlignment::End, align: StackCrossAlignment::Start,
                as_element: |props: StackRenderProps| rsx! { nav { ..props.attributes, {props.children} } },
                "aria-label": "Example pages", "data-test": "caller",
                a { href: "#overview", "Overview" }
                a { href: "#patterns", "Patterns" }
            }
        }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    assert!(
        html.starts_with("<nav ") && html.ends_with("</nav>"),
        "{html}"
    );
    for expected in [
        "astryx-stack--horizontal",
        "astryx-stack--justify-end",
        "astryx-stack--align-start",
        "data-test=\"caller\"",
        "aria-label=\"Example pages\"",
        "><a href=\"#overview\">Overview</a><a href=\"#patterns\">Patterns</a></nav>",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    for absent in ["<div", "role=", "tabindex="] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn default_root_is_a_plain_div_with_no_implicit_gap_or_wrap() {
    let html = render_element(rsx! { HStack { span { "First" } span { "Second" } } });
    assert!(
        html.starts_with("<div ") && html.ends_with("</div>"),
        "{html}"
    );
    assert!(html.contains("data-direction=\"horizontal\""), "{html}");
    assert!(
        html.contains("><span>First</span><span>Second</span></div>"),
        "{html}"
    );
    for absent in ["role=", "tabindex=", "aria-", "data-gap=", "data-wrap="] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn explicit_dimensions_follow_conflicting_caller_styles() {
    let html = render_element(rsx! {
        HStack {
            class: "caller-row",
            style: "color:red;width:10px;height:20px;max-width:30px;min-height:5px",
            width: SizeValue::from(300), height: SizeValue::from("3rem"),
            max_width: SizeValue::from("100%"), min_height: SizeValue::from(0),
            "Content"
        }
    });
    assert!(
        html.contains("astryx-stack astryx-stack--horizontal caller-row"),
        "{html}"
    );
    assert!(
        html.contains(concat!(
            "color:red;width:10px;height:20px;max-width:30px;min-height:5px;",
            "width:300px;height:3rem;max-width:100%;min-height:0px;"
        )),
        "{html}"
    );
}
