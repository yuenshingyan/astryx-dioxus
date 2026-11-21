use dioxus::prelude::*;

use astryx_dioxus::{
    Card, ScrollAxis, ScrollOverscroll, ScrollableArea, ScrollableAreaStickyContainment, SizeValue,
    SpacingStep,
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
            ScrollableArea { id: "scroll-area-inline", label: "Recently opened files", axis: ScrollAxis::Inline, width: SizeValue::from(460),
                div { style: "display:flex;gap:12px",
                    for (_, files) in GROUPS.into_iter().take(2) {
                        for file in files { div { key: "{file}", style: "width:170px;padding:12px;background:var(--color-background-muted)", "{file}" } }
                    }
                }
            }
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
