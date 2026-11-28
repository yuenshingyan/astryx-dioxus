use astryx_dioxus::{
    Card, HStack, SizeValue, SpacingStep, StackCrossAlignment, StackItem, StackItemCrossAlignSelf,
    StackItemRoot, StackItemSize, StackRenderProps, Text, TextColor, TextType, VStack,
};
use dioxus::prelude::*;

/// Pinned StackItem patterns and focused layout and overflow probes.
#[component]
pub fn StackItemExamples() -> Element {
    let mut is_tall = use_signal(|| true);
    rsx! {
        section { id: "stack-item-examples", "data-astryx-theme": "neutral",
            h2 { "Stack Item" }
            for (id, title, max_width, labels) in [
                ("stack-item-showcase", "StackItem", 500, &["Static Width", "Fills remaining space", "Static Width"][..]),
                ("stack-item-fill", "StackItem — Fill", 400, &["Static", "Fills remaining space"][..]),
            ] {
                h3 { key: "{id}", "{title}" }
                HStack { id, gap: SpacingStep::Three, v_align: StackCrossAlignment::Center,
                    width: SizeValue::from("100%"), max_width: SizeValue::from(max_width),
                    for (index, label) in labels.iter().enumerate() {
                        StackItem { key: "{index}", size: if index == 1 { StackItemSize::Fill } else { StackItemSize::Static },
                            Card { padding: SpacingStep::Three,
                                Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "{label}" }
                            }
                        }
                    }
                }
            }
            h3 { "Logo, content, and actions" }
            HStack { id: "stack-item-source-example", gap: SpacingStep::Two,
                StackItem { size: StackItemSize::Static, "Logo" }
                StackItem { size: StackItemSize::Fill, "Content" }
                StackItem { size: StackItemSize::Static, "Actions" }
            }
            h3 { "Cross-axis alignment" }
            VStack { id: "stack-item-align", h_align: StackCrossAlignment::Center, width: SizeValue::from(240),
                span { "Unwrapped child" }
                for (align, label) in [
                    (StackItemCrossAlignSelf::Start, "Start"),
                    (StackItemCrossAlignSelf::Center, "Center"),
                    (StackItemCrossAlignSelf::End, "End"),
                    (StackItemCrossAlignSelf::Stretch, "Stretch"),
                ] {
                    StackItem { key: "{label}", cross_align_self: align, "{label}" }
                }
            }
            h3 { "Scrollable item" }
            button { id: "stack-item-toggle-overflow", onclick: move |_| is_tall.toggle(), "Toggle overflow" }
            VStack { height: SizeValue::from(100),
                StackItem { id: "stack-item-scroll", size: StackItemSize::Fill, is_scrollable: true, scroll_label: "Scrollable example",
                    div { style: if is_tall() { "height:240px" } else { "height:24px" }, "Content" }
                }
            }
            HStack { width: SizeValue::from(160),
                StackItem { id: "stack-item-scroll-span", root: StackItemRoot::Span, size: StackItemSize::Fill,
                    is_scrollable: true, "aria-label": "Wide content",
                    span { style: "display:block;width:280px", "Wide content" }
                }
            }
            StackItem { id: "stack-item-scroll-article", is_scrollable: true, style: "height:48px",
                root: StackItemRoot::Custom(Callback::new(|props: StackRenderProps| rsx! { article { ..props.attributes, {props.children} } })),
                div { style: "height:100px", "Native article" }
            }
        }
    }
}
