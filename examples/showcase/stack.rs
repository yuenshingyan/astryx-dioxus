use astryx_dioxus::{
    Button, ButtonSize, ButtonVariant, Card, SizeValue, SpacingStep, Stack, StackAlignment,
    StackCrossAlignment, StackDirection, StackMainAlignment, StackRenderProps, StackWrap, Text,
    TextColor, TextType, TextWeight,
};
use dioxus::events::{onclick, onkeydown, onmounted};
use dioxus::prelude::*;

// Static subset of pinned Token.tsx, Avatar.tsx and Stack/stackItem.stylex.ts.
// Keep these declarations demo-local; the independent owners are not public ports.
const DEMO_CSS: &str = "
.stack-frame { max-width:600px; padding:16px; }
.stack-probe > span { display:block; flex:none; width:30px; height:20px; background:currentColor; }
.stack-demo-token { display:inline-flex; align-items:center; gap:var(--spacing-1); padding-block:0; padding-inline:var(--spacing-2); border:0 none; border-radius:var(--radius-inner); font-family:inherit; font-size:var(--text-supporting-size); line-height:var(--text-supporting-leading); font-weight:var(--font-weight-medium); white-space:nowrap; text-decoration:none; max-width:100%; overflow:hidden; height:calc(var(--size-element-md) - 8px); background:var(--color-neutral); color:var(--color-text-primary); }
.stack-demo-token > span { overflow:hidden; text-overflow:ellipsis; white-space:nowrap; min-width:0; unicode-bidi:isolate; }
.stack-demo-avatar { position:relative; display:inline-flex; flex-shrink:0; border-radius:var(--radius-full); width:36px; height:36px; }
.stack-demo-avatar-content { display:flex; align-items:center; justify-content:center; width:100%; height:100%; overflow:hidden; user-select:none; border-radius:var(--radius-full); }
.stack-demo-avatar-fallback { display:flex; align-items:center; justify-content:center; width:100%; height:100%; background:var(--color-neutral); color:var(--color-text-primary); font-family:var(--font-family-body); font-weight:var(--font-weight-medium); text-transform:uppercase; font-size:14.4px; }
.stack-demo-static { flex-grow:0; flex-shrink:0; min-width:0; min-height:0; }
.stack-demo-fill { flex-grow:1; min-width:0; min-height:0; }
.stack-class-override { gap:13px; padding-inline-start:17px; }
";

/// Standalone Stack patterns. Static Token/Avatar/item declarations are derived
/// from the pinned owners for this demo only; no supporting public port is claimed.
#[component]
pub fn StackExamples() -> Element {
    let mut event_count = use_signal(|| 0);
    let mut mounted_count = use_signal(|| 0);
    let mut key_count = use_signal(|| 0);
    let mut is_native_ref = use_signal(|| false);
    let mut is_horizontal = use_signal(|| false);
    let mut is_visible = use_signal(|| true);
    rsx! {
        document::Style { "{DEMO_CSS}" }
        section { id: "stack-examples", class: "stack-frame",
            h2 { "Stack" }
            p { "One-dimensional layout; Token, Avatar and StackItem static framing uses pinned source declarations, demo-only. Avatar tooltip/focus behavior is not implemented. No supporting public ports are claimed." }
            h3 { "Stack — Alignment" }
            Stack { id: "st-alignment", gap: SpacingStep::Three, width: SizeValue::from("100%"), style: "max-width:500px",
                for (label, value) in [("Start (left)", StackAlignment::Start), ("Center", StackAlignment::Center), ("End (right)", StackAlignment::End)] {
                    Card { padding: SpacingStep::Four,
                        Stack { gap: SpacingStep::Four,
                            Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "{label}" }
                            Stack { direction: StackDirection::Horizontal, gap: SpacingStep::One, h_align: value,
                                Button { label: "Cancel", variant: ButtonVariant::Secondary, size: ButtonSize::Small }
                                Button { label: "Save", size: ButtonSize::Small }
                            }
                        }
                    }
                }
            }
            h3 { "Stack — Directions (source-derived static Token framing)" }
            Stack { id: "st-directions", direction: StackDirection::Horizontal, gap: SpacingStep::Ten, h_align: StackAlignment::Center,
                Stack { direction: StackDirection::Horizontal, gap: SpacingStep::Two, v_align: StackAlignment::Center,
                    for _ in 0..3 { span { class: "stack-demo-token", span { dir: "ltr", "Horizontal" } } }
                }
                Stack { gap: SpacingStep::Two,
                    for _ in 0..3 { span { class: "stack-demo-token", span { dir: "ltr", "Vertical" } } }
                }
            }
            h3 { "Stack — Fill Item (source-derived static item/avatar framing)" }
            Stack { id: "st-fill", gap: SpacingStep::Three, width: SizeValue::from("100%"), style: "max-width:300px",
                for (name, role, initials) in [("Olivia Chen", "Engineering Lead", "OC"), ("Marcus Rivera", "Product Designer", "MR"), ("Aisha Patel", "Marketing Manager", "AP")] {
                    Stack { direction: StackDirection::Horizontal, gap: SpacingStep::Three, v_align: StackAlignment::Center,
                        div { class: "stack-demo-static",
                            div { class: "stack-demo-avatar", "aria-hidden": "true",
                                div { class: "stack-demo-avatar-content",
                                    div { class: "stack-demo-avatar-fallback", "{initials}" }
                                }
                            }
                        }
                        div { class: "stack-demo-fill",
                            Stack { gap: SpacingStep::Zero,
                                Text { text_type: TextType::BODY, weight: TextWeight::Bold, "{name}" }
                                Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "{role}" }
                            }
                        }
                        div { class: "stack-demo-static", Button { label: "View", variant: ButtonVariant::Secondary, size: ButtonSize::Small } }
                    }
                }
            }
            h3 { "Source contract probes" }
            div { id: "st-matrix",
                for direction in [StackDirection::Horizontal, StackDirection::Vertical] {
                    for main in [StackMainAlignment::Start, StackMainAlignment::Center, StackMainAlignment::End, StackMainAlignment::Between, StackMainAlignment::Around, StackMainAlignment::Evenly] {
                        for cross in [StackCrossAlignment::Start, StackCrossAlignment::Center, StackCrossAlignment::End, StackCrossAlignment::Stretch] {
                            Stack { class: "stack-probe", direction, justify: main, align: cross, width: SizeValue::from(180), height: SizeValue::from(100), gap: SpacingStep::Two,
                                "data-main": main.as_str(), "data-cross": cross.as_str(), span {} span {}
                            }
                        }
                    }
                }
            }
            div { id: "st-steps",
                for step in [SpacingStep::Zero, SpacingStep::Half, SpacingStep::One, SpacingStep::OneAndHalf, SpacingStep::Two, SpacingStep::Three, SpacingStep::Four, SpacingStep::Five, SpacingStep::Six, SpacingStep::Eight, SpacingStep::Ten] {
                    Stack { class: "stack-probe", direction: StackDirection::Horizontal, gap: step, padding: step, span {} span {} }
                }
            }
            Stack { id: "st-default", span { "One" } span { "Two" } }
            Stack { id: "st-padding", padding: SpacingStep::Ten, padding_inline: SpacingStep::Four, padding_block: SpacingStep::Three, padding_inline_start: SpacingStep::Zero, padding_block_end: SpacingStep::Half, span { "Local padding" } }
            Stack { id: "st-invalid", direction: StackDirection::Horizontal, h_align: StackAlignment::Stretch, v_align: StackAlignment::Between, justify: StackMainAlignment::End, align: StackCrossAlignment::Center }
            Stack { id: "st-explicit-h", direction: StackDirection::Horizontal, h_align: StackAlignment::End, v_align: StackAlignment::Start, justify: StackMainAlignment::Center, align: StackCrossAlignment::Stretch }
            Stack { id: "st-explicit-v", h_align: StackAlignment::Start, v_align: StackAlignment::End, justify: StackMainAlignment::Center, align: StackCrossAlignment::Stretch }
            Stack { id: "st-style", class: "stack-class-override", gap: SpacingStep::Two, padding: SpacingStep::Two, style: "width:1px;height:2px;max-width:3px;min-height:4px;gap:19px", width: SizeValue::from(200), height: SizeValue::from(120), max_width: SizeValue::from(240), min_height: SizeValue::from(80) }
            div { style: "width:137px;height:63px", Stack { id: "st-inherit", width: SizeValue::from("inherit"), height: SizeValue::from("inherit") } }
            for wrap in [StackWrap::Nowrap, StackWrap::Wrap, StackWrap::WrapReverse] {
                Stack { class: "stack-probe st-wrap", direction: StackDirection::Horizontal, width: SizeValue::from(70), height: SizeValue::from(60), gap: SpacingStep::Two, wrap, span {} span {} span {} }
            }
            Stack { id: "st-scroll", height: SizeValue::from(40), is_scrollable: true,
                button { style: "flex:none;height:100px", "Caller focusable scroll content" }
            }
            Stack { id: "st-height", height: SizeValue::from(40), span { style: "flex:none;height:100px", "Height alone does not scroll" } }
            Stack { id: "st-nested", padding: SpacingStep::Ten, gap: SpacingStep::Ten, Stack { id: "st-inner", span { "No inherited gap or padding" } } }
            Stack { id: "st-native", as_element: |props: StackRenderProps| rsx! { nav { ..props.attributes, {props.children} } },
                role: "navigation", "aria-label": "Caller navigation", "data-caller": "yes", dir: "rtl",
                onmounted: move |_event: MountedEvent| {
                    mounted_count += 1;
                    is_native_ref.set(false);
                    #[cfg(target_arch = "wasm32")]
                    is_native_ref.set(_event.data().downcast::<web_sys::Element>().is_some_and(|element| element.id() == "st-native" && element.tag_name() == "NAV"));
                },
                onclick: move |_| event_count += 1,
                attributes: vec![onclick(move |_| event_count += 1), onmounted(move |_| mounted_count += 1), onkeydown(move |_| key_count += 1)],
                button { id: "st-action", "Native action" }
            }
            output { id: "st-events", "{event_count}:{mounted_count}:{key_count}:{is_native_ref}" }
            button { id: "st-change", onclick: move |_| is_horizontal.toggle(), "Change direction" }
            button { id: "st-toggle", onclick: move |_| is_visible.toggle(), "Toggle stack" }
            if is_visible() {
                Stack { id: "st-changing", direction: if is_horizontal() { StackDirection::Horizontal } else { StackDirection::Vertical }, span { "Stable child" } }
            }
        }
    }
}
