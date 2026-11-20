use dioxus::prelude::*;

use astryx_dioxus::{
    Badge, HStack, SizeValue, SpacingStep, Stack, StackCrossAlignment, StackMainAlignment, Text,
    TextColor, TextType,
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
            Stack { width: SizeValue::from("100%"), max_width: SizeValue::from(400), gap: SpacingStep::Six,
                for (id, label, alignment, gap, items) in [
                    ("h-stack-start", "HAlign: start", None, SpacingStep::Two, ["React", "TypeScript", "Node.js"]),
                    ("h-stack-center", "HAlign: center", Some(StackMainAlignment::Center), SpacingStep::Four, ["Design", "Engineering", "Product"]),
                    ("h-stack-between", "HAlign: between", Some(StackMainAlignment::Between), SpacingStep::Two, ["Start", "Middle", "End"]),
                ] {
                    Stack { key: "{id}", gap: SpacingStep::Two,
                        Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "{label}" }
                        HStack { id, h_align: alignment, gap,
                            for label in items { Badge { key: "{label}", label } }
                        }
                    }
                }
            }
        }
    }
}
