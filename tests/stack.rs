use astryx_dioxus::{
    SizeValue, SpacingStep, Stack, StackAlignment, StackCrossAlignment, StackDirection,
    StackMainAlignment, StackRenderProps, StackWrap,
};
use dioxus::events::{onclick, onmounted};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn default_is_one_vertical_div_with_browser_alignment_gap_and_wrap_defaults() {
    let html = render(|| rsx! { Stack { "Caller content" } });
    assert_eq!(html.matches("<div").count(), 1);
    for expected in [
        "astryx-stack ",
        "astryx-stack--vertical",
        "data-direction=\"vertical\"",
        "Caller content",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    for absent in [
        "role=",
        "tabindex=",
        "aria-",
        "data-gap=",
        "data-wrap=",
        "--justify-",
        "--align-",
        "--padding-",
        "container-padding",
        "min-width",
        "min-height",
        "overflow",
    ] {
        assert!(!html.contains(absent), "{absent}: {html}");
    }
}

#[test]
fn every_axis_alignment_and_alias_resolves_for_both_directions() {
    let html = render(|| {
        rsx! {
            for direction in [StackDirection::Horizontal, StackDirection::Vertical] {
                for main in [StackMainAlignment::Start, StackMainAlignment::Center, StackMainAlignment::End, StackMainAlignment::Between, StackMainAlignment::Around, StackMainAlignment::Evenly] {
                    for cross in [StackCrossAlignment::Start, StackCrossAlignment::Center, StackCrossAlignment::End, StackCrossAlignment::Stretch] {
                        Stack { direction, justify: main, align: cross }
                    }
                }
            }
        }
    });
    assert_eq!(html.matches("<div").count(), 48);
    for name in ["start", "center", "end", "between", "around", "evenly"] {
        assert_eq!(
            html.matches(&format!("astryx-stack--justify-{name}"))
                .count(),
            8
        );
    }
    for name in ["start", "center", "end", "stretch"] {
        assert_eq!(
            html.matches(&format!("astryx-stack--align-{name}")).count(),
            12
        );
    }
    let explicit = render(|| {
        rsx! {
            Stack { direction: StackDirection::Horizontal, h_align: StackAlignment::End, v_align: StackAlignment::Start, justify: StackMainAlignment::Center, align: StackCrossAlignment::Stretch }
            Stack { h_align: StackAlignment::Start, v_align: StackAlignment::End, justify: StackMainAlignment::Center, align: StackCrossAlignment::Stretch }
        }
    });
    assert_eq!(explicit.matches("--justify-end").count(), 2);
    assert_eq!(explicit.matches("--align-start").count(), 2);
    assert!(!explicit.contains("--justify-center"));
    assert!(!explicit.contains("--align-stretch"));
}

#[test]
fn inapplicable_physical_values_are_ignored_without_alias_fallback() {
    let html = render(|| {
        rsx! {
            Stack { direction: StackDirection::Horizontal, h_align: StackAlignment::Stretch, v_align: StackAlignment::Between, justify: StackMainAlignment::End, align: StackCrossAlignment::Center }
            Stack { h_align: StackAlignment::Evenly, v_align: StackAlignment::Stretch, justify: StackMainAlignment::End, align: StackCrossAlignment::Center }
        }
    });
    assert!(!html.contains("--justify-"));
    assert!(!html.contains("--align-"));
}

#[test]
fn all_spacing_steps_and_wrap_values_reflect_without_invented_defaults() {
    let html = render(|| {
        rsx! {
            for gap in [SpacingStep::Zero, SpacingStep::Half, SpacingStep::One, SpacingStep::OneAndHalf, SpacingStep::Two, SpacingStep::Three, SpacingStep::Four, SpacingStep::Five, SpacingStep::Six, SpacingStep::Eight, SpacingStep::Ten] {
                Stack { gap, padding: gap }
            }
            for wrap in [StackWrap::Nowrap, StackWrap::Wrap, StackWrap::WrapReverse] { Stack { wrap } }
        }
    });
    for step in ["0", "0.5", "1", "1.5", "2", "3", "4", "5", "6", "8", "10"] {
        assert!(html.contains(&format!("data-gap=\"{step}\"")));
    }
    for token in ["0", "0-5", "1", "1-5", "2", "3", "4", "5", "6", "8", "10"] {
        assert_eq!(html.matches(&format!("var(--spacing-{token})")).count(), 5);
    }
    for wrap in ["nowrap", "wrap", "wrap-reverse"] {
        assert!(html.contains(&format!("data-wrap=\"{wrap}\"")));
    }
}

#[test]
fn padding_resolves_edge_then_axis_then_uniform_including_zero() {
    let html = render(|| {
        rsx! { Stack {
            padding: SpacingStep::Ten,
            padding_inline: SpacingStep::Four, padding_block: SpacingStep::Three,
            padding_inline_start: SpacingStep::Zero, padding_block_end: SpacingStep::Half,
        } }
    });
    for (edge, token) in [
        ("inline-start", "0"),
        ("inline-end", "4"),
        ("block-start", "3"),
        ("block-end", "0-5"),
    ] {
        assert!(
            html.contains(&format!("--_stack-padding-{edge}:var(--spacing-{token});")),
            "{html}"
        );
    }
    assert!(!html.contains("--spacing-10"));
    assert!(!html.contains("container-padding"));
}

#[test]
fn dimensions_follow_caller_style_and_scrolling_is_independent_of_height() {
    let html = render(|| {
        rsx! { Stack {
            style: "width:1px;height:2px;max-width:3px;min-height:4px",
            width: SizeValue::from(300), height: SizeValue::from("50vh"),
            max_width: SizeValue::from("calc(100% - 2rem)"), min_height: SizeValue::from(24.5),
        } }
    });
    assert!(html.contains("width:1px;height:2px;max-width:3px;min-height:4px;width:300px;height:50vh;max-width:calc(100% - 2rem);min-height:24.5px;"), "{html}");
    assert!(!html.contains("--scrollable"));
    let scroll = render(|| rsx! { Stack { is_scrollable: true } });
    assert!(scroll.contains("--scrollable"));
    assert!(!scroll.contains("role="));
    for invalid in [
        SizeValue::from(-1),
        SizeValue::from(f64::NAN),
        SizeValue::from("1px;display:none"),
    ] {
        assert!(invalid.validate().is_err());
    }
}

#[test]
fn polymorphic_root_receives_native_attributes_events_and_unwrapped_children() {
    let html = render(|| {
        rsx! { Stack {
            as_element: |props: StackRenderProps| rsx! { nav { ..props.attributes, {props.children} } },
            id: "native", role: "navigation", "aria-label": "Caller navigation", dir: "rtl",
            "data-caller": "yes", class: "caller", style: "margin:7px",
            attributes: vec![onclick(|_| {}), onmounted(|_| {})],
            button { "Independent action" }
        } }
    });
    for expected in [
        "<nav",
        "</nav>",
        "id=\"native\"",
        "role=\"navigation\"",
        "aria-label=\"Caller navigation\"",
        "dir=\"rtl\"",
        "data-caller=\"yes\"",
        "caller",
        "margin:7px",
        "<button",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(!html.contains("<div"));
}
