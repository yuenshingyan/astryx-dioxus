//! Pinned Heading templates, with native div adapters for unported Stack/Layout.
use astryx_dioxus::{
    Card, Heading, HeadingLevel, HeadingType, LayerPlacement, SizeValue, Text, TextColor,
    TextDisplay, TextJustify, TextType, TextWeight, TextWrap,
};
use dioxus::prelude::*;

const LEVELS: [HeadingLevel; 6] = [
    HeadingLevel::One,
    HeadingLevel::Two,
    HeadingLevel::Three,
    HeadingLevel::Four,
    HeadingLevel::Five,
    HeadingLevel::Six,
];

/// All four Heading patterns and Text's HeadingLevels cross-family pattern.
#[component]
pub fn HeadingExamples() -> Element {
    let mut is_changed = use_signal(|| false);
    let mut events = use_signal(|| 0_u32);
    let mut mounts = use_signal(|| 0_u32);
    rsx! {
        div { id: "heading-patterns", style: "display:flex;flex-direction:column;gap:32px;padding:24px",
            section { id: "heading-showcase", style: "display:flex;flex-direction:column;align-items:flex-start;gap:var(--spacing-2)",
                for level in LEVELS {
                    Heading { level, "data-pattern": "showcase", "Heading Level {level.number()}" }
                }
            }
            section { id: "heading-page", style: "display:flex;flex-direction:column;gap:var(--spacing-6);width:100%;max-width:400px",
                div {
                    Heading { level: HeadingLevel::One, "Dashboard Overview" }
                    Text { text_type: TextType::SUPPORTING, display: TextDisplay::Block, "Last updated 5 minutes ago" }
                }
                div {
                    Heading { level: HeadingLevel::Two, "Recent Activity" }
                    Text { display: TextDisplay::Block, "Here's what's been happening in your workspace." }
                }
                div {
                    Heading { level: HeadingLevel::Three, "Today" }
                    Text { display: TextDisplay::Block, "• Project Alpha updated" br {} "• 3 new comments" br {} "• Task completed" }
                }
            }
            section { id: "heading-card",
                Card { width: SizeValue::Pixels(300.0),
                    div { style: "display:flex;flex-direction:column;gap:var(--spacing-2)",
                        Heading { level: HeadingLevel::Three, "Card Title" }
                        Text { max_lines: 2, display: TextDisplay::Block,
                            "This is a card description that might be quite long and needs to be truncated after two lines to keep the card compact and uniform."
                        }
                        Text { text_type: TextType::SUPPORTING, display: TextDisplay::Block, "Updated 1 hour ago" }
                    }
                }
            }
            section { id: "heading-truncation", style: "display:flex;flex-direction:column;gap:24px",
                div { style: "box-sizing:border-box;width:300px;border:1px solid var(--color-border);padding:12px",
                    Heading { id: "heading-single", level: HeadingLevel::Two, max_lines: 1,
                        "Very Long Heading That Will Be Truncated To One Line With Ellipsis"
                    }
                }
                div { style: "box-sizing:border-box;width:300px;border:1px solid var(--color-border);padding:12px",
                    Heading { id: "heading-multi", level: HeadingLevel::Two, max_lines: 2,
                        "Very Long Heading That Will Be Truncated To Two Lines To Keep Card Layout Compact"
                    }
                }
            }
            section { id: "text-heading-levels", style: "display:flex;flex-direction:column;gap:var(--spacing-3)",
                for level in LEVELS {
                    Heading { level, "Heading {level.number()}" }
                }
            }
            section { id: "heading-probes", style: "display:flex;flex-direction:column;gap:16px;max-width:400px",
                for (index, heading_type) in [HeadingType::DISPLAY_1, HeadingType::DISPLAY_2, HeadingType::DISPLAY_3].into_iter().enumerate() {
                    Heading { id: "display-{index}", level: HeadingLevel::Two, heading_type, "Hero title" }
                }
                Heading { id: "heading-bold", level: HeadingLevel::Two, heading_type: HeadingType::DISPLAY_1, weight: TextWeight::Bold, "Bold display" }
                Heading { id: "heading-custom", level: HeadingLevel::Two,
                    heading_type: HeadingType::new("hero").ok(), color: TextColor::new("brand").unwrap_or_default(), "Custom baseline"
                }
                Heading { id: "heading-outline", level: HeadingLevel::Two, accessibility_level: HeadingLevel::Four, "Outline override" }
                Heading { id: "heading-raw-outline", level: HeadingLevel::Two, accessibility_level: HeadingLevel::Three, "aria-level": "5", "Caller outline" }
                div { Heading { id: "heading-inline", level: HeadingLevel::Three, display: TextDisplay::Inline, "Inline" } }
                Heading { id: "heading-capsize", level: HeadingLevel::Two, max_lines: 2, has_capsize: true, style: "width:160px", "Capsize removes multiline clamp and trims optical edges across several lines" }
                Heading { id: "heading-treatment", level: HeadingLevel::Three, text_wrap: TextWrap::Balance,
                    has_strikethrough: true, justify: TextJustify::End, color: TextColor::SECONDARY, "Balanced end aligned heading"
                }
                Heading { id: "heading-disabled-tip", level: HeadingLevel::Two, max_lines: 1, has_truncate_tooltip: false, style: "width:150px", "Disabled tooltip still clips this long heading" }
                Heading { id: "heading-placement", level: HeadingLevel::Two, max_lines: 1, truncate_tooltip_placement: LayerPlacement::End, style: "width:150px", "Logical end placement heading with long content" }
                Heading { id: "heading-reactive", level: HeadingLevel::Two,
                    max_lines: if is_changed() { 2 } else { 1 }, style: "width:150px",
                    onmounted: move |_| mounts += 1,
                    onclick: move |_| events += 1,
                    onwheel: move |_| events += 1,
                    oncopy: move |_| events += 1,
                    onanimationstart: move |_| events += 1,
                    ontransitionend: move |_| events += 1,
                    ondragstart: move |_| events += 1,
                    if is_changed() { "Changed heading with more than two lines of content for measurement" }
                    else { "Initial very long heading with rich " strong { "content" } }
                }
                button { id: "heading-change", onclick: move |_| is_changed.toggle(), "Change heading" }
                output { id: "heading-events", "{events}" }
                output { id: "heading-mounts", "{mounts}" }
                Heading { id: "heading-uncontrolled", level: HeadingLevel::Four, max_lines: if is_changed() { 2 } else { 1 },
                    input { id: "heading-input", initial_value: "caller-owned" }
                    "Content after input that remains caller-owned during clamp updates"
                }
                Heading { id: "heading-token", level: HeadingLevel::One,
                    "https://example.com/aVeryLongUnbrokenTokenThatIsWiderThan320pxAndMustWrapWithoutHorizontalScrolling"
                }
            }
        }
    }
}
