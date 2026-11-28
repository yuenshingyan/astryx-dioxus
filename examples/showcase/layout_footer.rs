use dioxus::prelude::*;

use astryx_dioxus::{
    Button, ButtonVariant, Card, CardVariant, Center, HStack, Layout, LayoutContent, LayoutFooter,
    LayoutHeader, LayoutPanel, SizeValue, SpacingStep, StackMainAlignment,
};

#[component]
fn FooterPattern(id: &'static str, #[props(default)] has_sidebar: bool) -> Element {
    let mut message = use_signal(String::new);
    rsx! {
        Center { width: SizeValue::from(400), max_width: SizeValue::from("100%"),
            div { style: "width:100%;min-width:0",
                div { style: "height:200px",
                    Layout { default_has_dividers: true,
                        header: rsx! { if has_sidebar {
                            LayoutHeader { Card { variant: CardVariant::MUTED } }
                        } },
                        start: rsx! { if has_sidebar {
                            LayoutPanel { width: SizeValue::from(140), has_divider: true,
                                Card { variant: CardVariant::MUTED }
                            }
                        } },
                        footer: rsx! {
                            LayoutFooter { id,
                                HStack { gap: SpacingStep::Two, h_align: StackMainAlignment::End,
                                    Button { label: "Cancel", variant: ButtonVariant::Secondary,
                                        onclick: move |_| message.set("Demo: your changes would be discarded here".into()) }
                                    Button { label: "Save", variant: ButtonVariant::Primary,
                                        onclick: move |_| message.set("Demo: your changes would be saved here".into()) }
                                }
                            }
                        },
                        LayoutContent {
                            Card { variant: CardVariant::MUTED, style: "min-height:200px" }
                        }
                    }
                }
                output { role: "status", "{message}" }
            }
        }
    }
}

#[component]
pub(super) fn LayoutFooterExamples() -> Element {
    rsx! {
        section { id: "layout-footer-examples", "data-astryx-theme": "neutral",
            h2 { "LayoutFooter — original patterns" }
            h3 { "Showcase" }
            FooterPattern { id: "layout-footer-showcase", has_sidebar: true }
            h3 { "Actions" }
            FooterPattern { id: "layout-footer-actions" }
        }
    }
}
