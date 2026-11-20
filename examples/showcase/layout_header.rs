use dioxus::prelude::*;

use astryx_dioxus::{
    Button, ButtonVariant, Card, CardVariant, Center, HStack, Heading, HeadingLevel, LayoutHeader,
    SizeValue, SpacingStep, StackCrossAlignment, StackMainAlignment,
};

#[component]
fn HeaderPattern(id: &'static str, #[props(default)] has_export: bool) -> Element {
    let mut message = use_signal(String::new);
    rsx! {
        Center { width: SizeValue::from(400), max_width: SizeValue::from("100%"),
            div { style: "width:100%;min-width:0",
                LayoutHeader { id, has_divider: true,
                    HStack { gap: SpacingStep::Two, v_align: StackCrossAlignment::Center, h_align: StackMainAlignment::Between,
                        Heading { level: HeadingLevel::Four, "Dashboard" }
                        HStack { gap: SpacingStep::Two,
                            if has_export {
                                Button { label: "Export", variant: ButtonVariant::Secondary,
                                    onclick: move |_| message.set("Demo: an export would start here".into()) }
                            }
                            Button { label: "New Item", variant: ButtonVariant::Primary,
                                onclick: move |_| message.set("Demo: a new item form would open here".into()) }
                        }
                    }
                }
                div { style: "display:flex",
                    if has_export {
                        div { style: "box-sizing:border-box;width:140px;padding:var(--spacing-4);border-inline-end:1px solid var(--color-border)",
                            Card { variant: CardVariant::MUTED }
                        }
                    }
                    div { style: "min-width:0;flex:1;padding:var(--spacing-4)", Card { variant: CardVariant::MUTED } }
                }
                if has_export {
                    div { style: "padding:var(--spacing-4);border-block-start:1px solid var(--color-border)", Card { variant: CardVariant::MUTED } }
                }
                output { role: "status", "{message}" }
            }
        }
    }
}

#[component]
pub(super) fn LayoutHeaderExamples() -> Element {
    rsx! {
        section { id: "layout-header-examples", "data-astryx-theme": "neutral",
            h2 { "LayoutHeader — original patterns" }
            h3 { "Showcase" }
            HeaderPattern { id: "layout-header-showcase", has_export: true }
            h3 { "With actions" }
            HeaderPattern { id: "layout-header-actions" }
        }
    }
}
