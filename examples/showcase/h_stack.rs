use dioxus::prelude::*;

use astryx_dioxus::{
    Badge, HStack, SizeValue, SpacingStep, StackCrossAlignment, StackMainAlignment,
    StackRenderProps, Text, TextColor, TextType, VStack,
};

#[component]
pub(super) fn HStackExamples() -> Element {
    rsx! {
        section { id: "h-stack-examples", "data-astryx-theme": "neutral",
            h2 { "HStack — original patterns" }
            h3 { "Basic" }
            HStack { id: "h-stack-basic", gap: SpacingStep::Two, v_align: StackCrossAlignment::Center,
                Badge { label: "React" }
                Badge { label: "TypeScript" }
                Badge { label: "Node.js" }
            }
            h3 { "Showcase" }
            VStack { id: "h-stack-showcase", width: SizeValue::from("100%"), style: "max-width:400px", gap: SpacingStep::Six,
                for (id, label, alignment, gap, items) in [
                    ("h-stack-start", "HAlign: start", None, SpacingStep::Two, ["React", "TypeScript", "Node.js"]),
                    ("h-stack-center", "HAlign: center", Some(StackMainAlignment::Center), SpacingStep::Four, ["Design", "Engineering", "Product"]),
                    ("h-stack-between", "HAlign: between", Some(StackMainAlignment::Between), SpacingStep::Two, ["Start", "Middle", "End"]),
                ] {
                    VStack { key: "{id}", gap: SpacingStep::Two,
                        Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "{label}" }
                        HStack { id, h_align: alignment, gap,
                            for label in items { Badge { key: "{label}", label } }
                        }
                    }
                }
            }
            HStackNativeRoot {}
        }
    }
}

#[component]
fn HStackNativeRoot() -> Element {
    let mut is_root_mounted = use_signal(|| false);
    let mut clicks = use_signal(|| 0);
    rsx! {
        h3 { "Native root" }
        HStack {
            as_element: |props: StackRenderProps| rsx! { nav { ..props.attributes, {props.children} } },
            id: "h-stack-native-root", "aria-label": "Example pages", gap: SpacingStep::Two,
            "data-root-mounted": "{is_root_mounted}", "data-clicks": "{clicks}",
            onmounted: move |event| is_root_mounted.set(is_native_root(event)),
            onclick: move |_| clicks += 1,
            a { id: "h-stack-overview", href: "#h-stack-examples", "Overview" }
            a { id: "h-stack-patterns", href: "#h-stack-showcase", "Patterns" }
        }
    }
}

fn is_native_root(event: MountedEvent) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        event
            .data()
            .downcast::<web_sys::Element>()
            .is_some_and(|node| node.tag_name() == "NAV" && node.id() == "h-stack-native-root")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        drop(event);
        false
    }
}
