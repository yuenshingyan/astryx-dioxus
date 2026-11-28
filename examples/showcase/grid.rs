use dioxus::prelude::*;

use astryx_dioxus::{
    Card, CardVariant, Grid, GridColumns, GridRepeat, GridSpan, GridSpanColumns, SizeValue,
    SpacingStep, Stack, StackDirection, Text, TextDisplay, TextType,
};

#[component]
fn GridLabel(title: &'static str, detail: String, #[props(default)] is_spaced: bool) -> Element {
    let text = rsx! {
        Text { text_type: TextType::LABEL, display: TextDisplay::Block, "{title}" }
        Text { text_type: TextType::SUPPORTING, display: TextDisplay::Block, "{detail}" }
    };
    if is_spaced {
        rsx! { Stack { direction: StackDirection::Vertical, gap: SpacingStep::One, {text} } }
    } else {
        text
    }
}

#[component]
pub(super) fn GridExamples() -> Element {
    let mut panel_width = use_signal(|| 480);
    rsx! {
        section { id: "grid-examples", "data-astryx-theme": "neutral",
            h2 { "Grid — original patterns" }
            h3 { "Showcase" }
            Grid { id: "grid-fixed", columns: GridColumns::Fixed(3), gap: SpacingStep::Two, width: SizeValue::from(400),
                for item in 1..=12 { Card { key: "{item}", "Item {item}" } }
            }
            h3 { "Gallery" }
            Grid { id: "grid-gallery", columns: GridColumns::Responsive { min_width: 180.0, max: None, repeat: GridRepeat::Fill },
                gap: SpacingStep::Five, width: SizeValue::from("100%"), max_width: SizeValue::from(400),
                for (title, detail) in [
                    ("Getting Started", "Learn the basics of the platform."),
                    ("Components", "Browse the full component library."),
                    ("Design Tokens", "Colors, spacing, and typography."),
                    ("Theming", "Customize the look and feel."),
                    ("Accessibility", "Build inclusive experiences."),
                    ("Patterns", "Common UI composition patterns."),
                ] { Card { key: "{title}", GridLabel { title, detail: detail.to_owned(), is_spaced: true } } }
            }
            h3 { "Responsive auto-fit" }
            label { "for": "grid-panel-width", "Resize grid" }
            input { id: "grid-panel-width", r#type: "range", min: 100, max: 480, value: "{panel_width}",
                oninput: move |event| { if let Ok(width) = event.value().parse() { panel_width.set(width); } }
            }
            Card { variant: CardVariant::MUTED, padding: SpacingStep::Zero, height: SizeValue::from(400), width: SizeValue::from("100%"), max_width: SizeValue::from(500),
                div { style: "box-sizing:border-box;width:min({panel_width}px,100%);height:100%;padding:var(--spacing-4);overflow:auto;border-inline-end:1px solid var(--color-border)",
                    Grid { id: "grid-auto-fit", columns: GridColumns::Responsive { min_width: 180.0, max: None, repeat: GridRepeat::Fit }, gap: SpacingStep::Four, width: SizeValue::from("100%"),
                        for (title, count) in [("Design Systems",8), ("Frontend Platform",12), ("Developer Experience",6), ("Accessibility",4), ("Performance",7), ("Mobile Infrastructure",9)] {
                            Card { key: "{title}", GridLabel { title, detail: format!("{count} members"), is_spaced: true } }
                        }
                    }
                }
            }
            h3 { "Dashboard layout" }
            Grid { id: "grid-dashboard", columns: GridColumns::Fixed(4), gap: SpacingStep::Four, width: SizeValue::from("100%"), max_width: SizeValue::from(500),
                GridSpan { columns: GridSpanColumns::Count(2), rows: 2,
                    Card { GridLabel { title: "Weekly Traffic", detail: "Page views and unique visitors over the last 7 days" } }
                }
                for (title, detail) in [("Revenue","$48,290"), ("Active Users","12,841"), ("Conversion","3.2%"), ("Avg Response","245ms")] {
                    Card { key: "{title}",
                        Text { text_type: TextType::SUPPORTING, display: TextDisplay::Block, "{title}" }
                        Text { text_type: TextType::LABEL, display: TextDisplay::Block, "{detail}" }
                    }
                }
                GridSpan { columns: GridSpanColumns::Full,
                    Card { GridLabel { title: "Recent Activity", detail: "Latest events across all projects" } }
                }
            }
            h3 { "With spanning items" }
            Grid { id: "grid-with-span", columns: GridColumns::Fixed(3), gap: SpacingStep::Four, width: SizeValue::from("100%"), max_width: SizeValue::from(500),
                GridSpan { rows: 2,
                    Card { variant: CardVariant::CYAN, GridLabel { title: "Featured Release", detail: "Astryx 4.0 is now available with new layout primitives, refreshed tokens, and improved theming support across the system.", is_spaced: true } }
                }
                for (title, detail) in [("Components","54 available"), ("Templates","28 available"), ("Tokens","120 defined"), ("Themes","6 published"), ("Icons","312 available"), ("Patterns","18 documented"), ("Contributors","42 active")] {
                    Card { key: "{title}", GridLabel { title, detail: detail.to_owned() } }
                }
                GridSpan { columns: GridSpanColumns::Full,
                    Card { variant: CardVariant::CYAN, GridLabel { title: "Community Showcase", detail: "See how teams are building with Astryx across the organization", is_spaced: true } }
                }
            }
        }
    }
}
