use dioxus::prelude::*;

use astryx_dioxus::{
    ButtonSize, ButtonVariant, Card, Center, CenterAxis, Heading, HeadingLevel, Icon, IconButton,
    IconSize, SizeValue, SpacingStep, Stack, StackAlignment, StackDirection, Text, TextColor,
    TextType, TextWeight,
};

// Heroicons v2.2.0 / optimized/24/outline (MIT); see THIRD_PARTY_NOTICES.md.
const FORMATTING_GLYPHS: [(&str, &str); 6] = [
    (
        "Bold",
        "M6.75 3.744h-.753v8.25h7.125a4.125 4.125 0 0 0 0-8.25H6.75Zm0 0v.38m0 16.122h6.747a4.5 4.5 0 0 0 0-9.001h-7.5v9h.753Zm0 0v-.37m0-15.751h6a3.75 3.75 0 1 1 0 7.5h-6m0-7.5v7.5m0 0v8.25m0-8.25h6.375a4.125 4.125 0 0 1 0 8.25H6.75m.747-15.38h4.875a3.375 3.375 0 0 1 0 6.75H7.497v-6.75Zm0 7.5h5.25a3.75 3.75 0 0 1 0 7.5h-5.25v-7.5Z",
    ),
    (
        "Italic",
        "M5.248 20.246H9.05m0 0h3.696m-3.696 0 5.893-16.502m0 0h-3.697m3.697 0h3.803",
    ),
    (
        "Underline",
        "M17.995 3.744v7.5a6 6 0 1 1-12 0v-7.5m-2.25 16.502h16.5",
    ),
    (
        "List",
        "M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0ZM3.75 12h.007v.008H3.75V12Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm-.375 5.25h.007v.008H3.75v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z",
    ),
    (
        "Link",
        "M13.19 8.688a4.5 4.5 0 0 1 1.242 7.244l-4.5 4.5a4.5 4.5 0 0 1-6.364-6.364l1.757-1.757m13.35-.622 1.757-1.757a4.5 4.5 0 0 0-6.364-6.364l-4.5 4.5a4.5 4.5 0 0 0 1.242 7.244",
    ),
    (
        "Image",
        "m2.25 15.75 5.159-5.159a2.25 2.25 0 0 1 3.182 0l5.159 5.159m-1.5-1.5 1.409-1.409a2.25 2.25 0 0 1 3.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 0 0 1.5-1.5V6a1.5 1.5 0 0 0-1.5-1.5H3.75A1.5 1.5 0 0 0 2.25 6v12a1.5 1.5 0 0 0 1.5 1.5Zm10.5-11.25h.008v.008h-.008V8.25Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z",
    ),
];
const INBOX: &str = "M2.25 13.5h3.86a2.25 2.25 0 0 1 2.012 1.244l.256.512a2.25 2.25 0 0 0 2.013 1.244h3.218a2.25 2.25 0 0 0 2.013-1.244l.256-.512a2.25 2.25 0 0 1 2.013-1.244h3.859m-19.5.338V18a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 0 0-2.15-1.588H6.911a2.25 2.25 0 0 0-2.15 1.588L2.35 13.177a2.25 2.25 0 0 0-.1.661Z";

fn glyph(path_data: &'static str) -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", width: "1em", height: "1em", fill: "none", stroke: "currentColor",
            stroke_width: "1.5", stroke_linecap: "round", stroke_linejoin: "round",
            path { d: path_data }
        }
    }
}

#[component]
pub(super) fn CenterExamples() -> Element {
    rsx! {
        section { id: "center-examples", "data-astryx-theme": "neutral",
            h2 { "Center — original patterns" }
            h3 { "Horizontal Center" }
            Card { width: SizeValue::from(520), max_width: SizeValue::from("100%"), padding: SpacingStep::Two,
                Center { id: "center-formatting", axis: CenterAxis::Horizontal, width: SizeValue::from("100%"),
                    Stack { direction: StackDirection::Horizontal, gap: SpacingStep::Zero, v_align: StackAlignment::Center,
                        for (label, path_data) in FORMATTING_GLYPHS {
                            IconButton { key: "{label}", label, variant: ButtonVariant::Ghost, size: ButtonSize::Small, icon: rsx! { Icon { icon: glyph(path_data) } } }
                        }
                    }
                }
            }
            p { "In vertical writing, Horizontal follows the flex main axis, not the physical horizontal axis." }
            h3 { "Vertical & Horizontal Center" }
            Card { width: SizeValue::from(400), max_width: SizeValue::from("100%"),
                Center { id: "center-empty-state", height: SizeValue::from(200),
                    Stack { direction: StackDirection::Vertical, gap: SpacingStep::Two, h_align: StackAlignment::Center,
                        Icon { icon: glyph(INBOX), size: IconSize::Large, color: "var(--color-icon-secondary)".to_owned() }
                        Text { weight: TextWeight::Bold, "No messages yet" }
                        Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY, "Messages from your team will appear here." }
                    }
                }
            }
            h3 { "Center" }
            Center { id: "center-showcase", width: SizeValue::from("100%"), height: SizeValue::from(240),
                Stack { direction: StackDirection::Vertical, gap: SpacingStep::Two, h_align: StackAlignment::Center,
                    Heading { level: HeadingLevel::Four, "Centered content" }
                    Text { color: TextColor::SECONDARY, "Horizontally and vertically aligned." }
                }
            }
            h3 { "Vertical Center with logical padding" }
            Center { id: "center-vertical", axis: CenterAxis::Vertical,
                width: SizeValue::from(160), height: SizeValue::from(96),
                padding: SpacingStep::One, padding_inline: SpacingStep::Two,
                padding_inline_start: SpacingStep::Zero, padding_block_end: SpacingStep::Three,
                span { "Cross-axis center" }
            }
            h3 { "Inline Center" }
            div { "Status: " Center { id: "center-inline", is_inline: true,
                Icon { icon: glyph(INBOX), size: IconSize::Small }
            } " Inbox" }
            div { style: "width:240px;max-width:100%;min-height:48px",
                Center { id: "center-inherited-size", width: SizeValue::from("inherit"),
                    max_width: SizeValue::from("inherit"), min_height: SizeValue::from("inherit"),
                    "Inherited dimensions"
                }
            }
        }
    }
}
