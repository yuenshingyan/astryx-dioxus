use dioxus::prelude::*;

use astryx_dioxus::{
    Badge, HStack, Heading, HeadingLevel, SpacingStep, StackMainAlignment, Text, TextColor,
    TextType, VStack,
};

#[component]
pub(super) fn VStackExamples() -> Element {
    rsx! {
        section { id: "v-stack-examples", "data-astryx-theme": "neutral",
            h2 { "VStack — original patterns" }
            h3 { "Basic" }
            VStack { id: "v-stack-basic", gap: SpacingStep::Three,
                Heading { level: HeadingLevel::Five, "Weekly Report" }
                Text { color: TextColor::SECONDARY, "VStack arranges its children in a vertical column." }
                Text { color: TextColor::SECONDARY, "The gap prop controls the spacing between each item." }
            }
            h3 { "Showcase" }
            HStack { id: "v-stack-gap-showcase", gap: SpacingStep::Ten, h_align: StackMainAlignment::Center,
                for (step, gap) in [(2, SpacingStep::Two), (4, SpacingStep::Four), (6, SpacingStep::Six)] {
                    VStack { key: "{step}", gap: SpacingStep::Two,
                        Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "gap={step}" }
                        VStack { id: "v-stack-gap-{step}", gap,
                            Badge { label: "Step 1" }
                            Badge { label: "Step 2" }
                            Badge { label: "Step 3" }
                        }
                    }
                }
            }
        }
    }
}
