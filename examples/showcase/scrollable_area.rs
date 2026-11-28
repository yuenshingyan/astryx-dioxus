use dioxus::prelude::*;

use astryx_dioxus::{
    Card, CardVariant, HStack, Heading, HeadingLevel, Link, ScrollAxis, ScrollOverscroll,
    ScrollableArea, ScrollableAreaRole, ScrollableAreaStickyContainment, SizeValue, SpacingStep,
    StackCrossAlignment, StackMainAlignment, Text, TextType, TextWeight, VStack,
};

const GROUPS: [(&str, [&str; 4]); 4] = [
    (
        "Recently opened",
        [
            "Q3 launch brief",
            "Pricing rework",
            "Onboarding flow v4",
            "Support macros",
        ],
    ),
    (
        "Shared with you",
        [
            "Brand refresh",
            "Checkout audit",
            "Partner deck",
            "Research synthesis",
        ],
    ),
    (
        "Templates",
        [
            "Weekly report",
            "Launch checklist",
            "Design review",
            "Retro board",
        ],
    ),
    (
        "Archived",
        [
            "2025 roadmap",
            "Legacy pricing",
            "Old brand kit",
            "Beta feedback",
        ],
    ),
];

#[component]
fn FileGroup(index: usize, #[props(default)] is_sticky: bool) -> Element {
    let (title, files) = GROUPS[index];
    rsx! {
        div {
            h4 { style: if is_sticky { "position:sticky;inset-block-start:0;background:var(--color-background-surface);margin:0;padding:8px" } else { "margin:0;padding:8px" }, "{title}" }
            for file in files {
                div { key: "{file}", style: "padding:12px;border-block-end:1px solid var(--color-border)", "{file}" }
            }
        }
    }
}

#[component]
fn ScrollableAreaInlineRail() -> Element {
    const FILES: [(&str, &str); 7] = [
        ("Q3 launch brief", "Edited 2 days ago"),
        ("Pricing rework", "Edited 3 days ago"),
        ("Onboarding flow v4", "Edited 4 days ago"),
        ("Support macros", "Edited last week"),
        ("Brand refresh", "Priya Raman"),
        ("Checkout audit", "Tomas Lindqvist"),
        ("Partner deck", "Ines Okafor"),
    ];
    rsx! {
        Card { id: "scroll-area-rail-card", width: SizeValue::from(460), padding: SpacingStep::Four,
            VStack { gap: SpacingStep::Three,
                HStack { h_align: StackMainAlignment::Between, v_align: StackCrossAlignment::Center,
                    VStack { gap: SpacingStep::Half,
                        Heading { id: "scroll-area-rail-heading", level: HeadingLevel::Three, "Workspace files" }
                        Text { text_type: TextType::SUPPORTING, "Recently opened" }
                    }
                    Link { href: "#scroll-area-block", "View all" }
                }
                ScrollableArea {
                    id: "scroll-area-inline", axis: ScrollAxis::Inline, role: ScrollableAreaRole::Region,
                    label: "Recently opened files", is_full_bleed: true, padding_inline: SpacingStep::Four,
                    padding_block_start: SpacingStep::Half, padding_block_end: SpacingStep::Four,
                    HStack { gap: SpacingStep::Two,
                        for (title, details) in FILES {
                            Card { key: "{title}", variant: CardVariant::MUTED, padding: SpacingStep::Three, width: SizeValue::from(170),
                                VStack { gap: SpacingStep::One,
                                    Text { weight: TextWeight::Medium, max_lines: 1, "{title}" }
                                    Text { text_type: TextType::SUPPORTING, max_lines: 1, "{details}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub(super) fn ScrollableAreaExamples() -> Element {
    let mut is_tall = use_signal(|| true);
    rsx! {
        section { id: "scrollable-area-examples", "data-astryx-theme": "neutral",
            h2 { "ScrollableArea" }
            h3 { "Showcase" }
            Card { width: SizeValue::from(460), padding: SpacingStep::Four,
                h4 { "Workspace files" }
                ScrollableArea { id: "scroll-area-block", label: "Workspace files", height: SizeValue::from(300), is_full_bleed: true, padding_inline: SpacingStep::Four,
                    for index in [0, 1, 3] { FileGroup { key: "{index}", index } }
                }
            }
            h3 { "Inline rail" }
            ScrollableAreaInlineRail {}
            h3 { "Sticky group headers" }
            ScrollableArea { id: "scroll-area-sticky", label: "Workspace files with sticky groups", width: SizeValue::from(460), height: SizeValue::from(300),
                for index in [0, 1, 3] { FileGroup { key: "{index}", index, is_sticky: true } }
            }
            h3 { "Sticky passthrough" }
            ScrollableArea { label: "Workspace files with nested sticky headers", width: SizeValue::from(460), height: SizeValue::from(300),
                ScrollableArea { id: "scroll-area-pass", label: "Shared with you", FileGroup { index: 1, is_sticky: true } }
                ScrollableArea { id: "scroll-area-hold", label: "Templates", sticky_containment: ScrollableAreaStickyContainment::Always, FileGroup { index: 2, is_sticky: true } }
                FileGroup { index: 3, is_sticky: true }
            }
            h3 { "Two-axis board" }
            ScrollableArea { id: "scroll-area-both", label: "Workspace board", axis: ScrollAxis::Both, width: SizeValue::from(460), height: SizeValue::from(150),
                div { style: "display:flex;gap:12px",
                    for index in 0..4 { div { key: "{index}", style: "width:200px", FileGroup { index } } }
                }
            }
            h3 { "Overscroll chaining" }
            ScrollableArea { label: "Workspace files with nested scrollers", width: SizeValue::from(460), height: SizeValue::from(300),
                h4 { "Shared with you" }
                ScrollableArea { id: "scroll-area-allow", label: "Shared with you", height: SizeValue::from(120), FileGroup { index: 1 } }
                h4 { "Archived" }
                ScrollableArea { id: "scroll-area-contain", label: "Archived", height: SizeValue::from(120), overscroll: ScrollOverscroll::Contain, FileGroup { index: 3 } }
                FileGroup { index: 2 }
            }
            h3 { "Changing content" }
            button { id: "scroll-area-toggle", onclick: move |_| is_tall.toggle(), "Toggle overflow" }
            ScrollableArea { id: "scroll-area-change", label: "Changing files", width: SizeValue::from(240), height: SizeValue::from(120),
                div { style: if is_tall() { "height:240px" } else { "height:24px" }, "Files" }
            }
        }
    }
}
