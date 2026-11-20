use dioxus::prelude::*;

use astryx_dioxus::{
    Card, CardVariant, Center, Heading, HeadingLevel, LayoutContent, LayoutFooter, LayoutHeader,
    SizeValue, SpacingStep, Text, TextColor, VStack,
};

#[component]
fn ContentPattern(id: &'static str, #[props(default)] is_full: bool) -> Element {
    let title = if is_full {
        "Main Content Area"
    } else {
        "Main Content"
    };
    rsx! {
        Center { width: SizeValue::from(if is_full { 500 } else { 400 }), max_width: SizeValue::from("100%"),
            div { style: "width:100%;min-width:0;display:flex;flex-direction:column",
                LayoutHeader { has_divider: true, Card { variant: CardVariant::MUTED } }
                div { style: "display:flex;min-height:0",
                    if is_full {
                        div { style: "box-sizing:border-box;width:140px;padding:var(--spacing-4);border-inline-end:1px solid var(--color-border)", Card { variant: CardVariant::MUTED } }
                    }
                    LayoutContent { id, role: "region", label: title,
                        VStack { gap: SpacingStep::Three,
                            Heading { level: HeadingLevel::Five, "{title}" }
                            if is_full {
                                Text { color: TextColor::SECONDARY, "LayoutContent provides automatic padding and scroll containment. It fills the remaining space between the header and footer." }
                                Text { color: TextColor::SECONDARY, "Content that overflows will scroll within this area while the header and footer remain fixed." }
                            } else {
                                Text { color: TextColor::SECONDARY, "LayoutContent fills the remaining space and scrolls when its content overflows, while the header stays fixed." }
                            }
                        }
                    }
                }
                if is_full { LayoutFooter { has_divider: true, Card { variant: CardVariant::MUTED } } }
            }
        }
    }
}

#[component]
pub(super) fn LayoutContentExamples() -> Element {
    let mut is_tall = use_signal(|| true);
    rsx! {
        section { id: "layout-content-examples", "data-astryx-theme": "neutral",
            h2 { "LayoutContent — original arrangements" }
            h3 { "Basic" }
            ContentPattern { id: "layout-content-basic" }
            h3 { "Showcase" }
            ContentPattern { id: "layout-content-showcase", is_full: true }
            h3 { "Overflow naming" }
            button { id: "layout-content-toggle", onclick: move |_| is_tall.toggle(), "Toggle overflow" }
            LayoutContent { id: "layout-content-scroll", style: "height:96px", scroll_label: "Scrollable example",
                div { style: if is_tall() { "height:240px" } else { "height:24px" }, "Content" }
            }
        }
    }
}
