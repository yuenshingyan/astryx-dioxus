use dioxus::prelude::*;

use astryx_dioxus::{
    Button, ButtonVariant, HStack, Heading, HeadingLevel, Icon, IconSize, Section, SectionDivider,
    SectionVariant, SizeValue, SpacingStep, StackCrossAlignment, Text, TextColor, TextType,
    TextWeight, VStack,
};

#[component]
fn VariantContent(title: &'static str, description: &'static str) -> Element {
    rsx! {
        VStack { gap: SpacingStep::One,
            Text { weight: TextWeight::Bold, "{title}" }
            Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "{description}" }
        }
    }
}

#[component]
pub(super) fn SectionExamples() -> Element {
    rsx! {
        section { id: "section-examples", "data-astryx-theme": "neutral",
            h2 { "Section — original patterns" }
            h3 { "Center a Section with a maximum width" }
            div { id: "section-centered",
                Section {
                    max_width: SizeValue::from(1280),
                    style: "margin-inline-start:auto; margin-inline-end:auto;",
                    VStack { gap: SpacingStep::One,
                        Heading { level: HeadingLevel::Four, "Profile settings" }
                        Text { color: TextColor::SECONDARY, "Manage the information on your profile." }
                    }
                }
            }
            h3 { "Section — Variants" }
            VStack { id: "section-variants", gap: SpacingStep::Six,
                Section { padding: SpacingStep::Five, VariantContent { title: "Section", description: "White background." } }
                Section { variant: SectionVariant::MUTED, padding: SpacingStep::Five, VariantContent { title: "Wash", description: "Gray background." } }
                VStack { Section { variant: SectionVariant::TRANSPARENT, padding: SpacingStep::Five,
                    VariantContent { title: "Transparent", description: "No background, shows the color behind it." }
                } }
            }
            h3 { "Section — With Dividers" }
            VStack { id: "section-with-dividers", gap: SpacingStep::Zero,
                for (title, description, has_divider) in [("Account", "Manage your profile, email, and password.", true), ("Notifications", "Choose what updates you receive and how.", true), ("Privacy", "Control who can see your activity and data.", false)] {
                    Section { key: "{title}", padding: SpacingStep::Five, dividers: if has_divider { vec![SectionDivider::Bottom] } else { vec![] },
                        VStack { gap: SpacingStep::One,
                            Heading { level: HeadingLevel::Four, "{title}" }
                            Text { color: TextColor::SECONDARY, "{description}" }
                        }
                    }
                }
            }
            h3 { "Section — Default with Wash" }
            VStack { id: "section-wash-highlight", gap: SpacingStep::Two,
                Section { padding: SpacingStep::Four,
                    VStack { gap: SpacingStep::Three, h_align: StackCrossAlignment::Center,
                        VStack { gap: SpacingStep::One, h_align: StackCrossAlignment::Center,
                            Text { text_type: TextType::DISPLAY_3, "Pro Plan" }
                            Text { color: TextColor::SECONDARY, "Everything you need to scale your team." }
                        }
                        VStack { gap: SpacingStep::Two,
                            for feature in ["10 team members", "Unlimited projects", "Priority support", "Advanced analytics"] {
                                HStack { key: "{feature}", gap: SpacingStep::Two, v_align: StackCrossAlignment::Center,
                                    Icon { size: IconSize::Small, icon: rsx! {
                                        svg { view_box: "0 0 24 24", width: "1em", height: "1em", fill: "currentColor",
                                            path { fill_rule: "evenodd", clip_rule: "evenodd", d: "M19.916 4.626a.75.75 0 0 1 .208 1.04l-9 13.5a.75.75 0 0 1-1.154.114l-6-6a.75.75 0 0 1 1.06-1.06l5.353 5.353 8.493-12.74a.75.75 0 0 1 1.04-.207Z" }
                                        }
                                    } }
                                    Text { "{feature}" }
                                }
                            }
                        }
                    }
                }
                Section { variant: SectionVariant::MUTED, padding: SpacingStep::Six,
                    VStack { gap: SpacingStep::Two, h_align: StackCrossAlignment::Center,
                        HStack { gap: SpacingStep::Two, v_align: StackCrossAlignment::Center,
                            Text { text_type: TextType::DISPLAY_3, "$49" }
                            Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "/ month" }
                        }
                        Button { label: "Upgrade", variant: ButtonVariant::Primary }
                    }
                }
            }
        }
    }
}
