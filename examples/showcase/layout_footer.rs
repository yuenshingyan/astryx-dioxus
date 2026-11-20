use dioxus::prelude::*;

use astryx_dioxus::{
    Button, ButtonVariant, Card, CardVariant, Center, HStack, LayoutFooter, LayoutHeader,
    SizeValue, SpacingStep, StackMainAlignment,
};

#[component]
fn FooterPattern(id: &'static str, #[props(default)] has_sidebar: bool) -> Element {
    let mut message = use_signal(String::new);
    rsx! {
        Center { width: SizeValue::from(400), max_width: SizeValue::from("100%"),
            div { style: "width:100%;min-width:0",
                if has_sidebar {
                    LayoutHeader { has_divider: true, Card { variant: CardVariant::MUTED } }
                }
                div { style: "display:flex",
                    if has_sidebar {
                        div { style: "box-sizing:border-box;width:140px;padding:var(--spacing-4);border-inline-end:1px solid var(--color-border)",
                            Card { variant: CardVariant::MUTED }
                        }
                    }
                    div { style: "min-width:0;flex:1;padding:var(--spacing-4)", Card { variant: CardVariant::MUTED } }
                }
                LayoutFooter { id, has_divider: true,
                    HStack { gap: SpacingStep::Two, h_align: StackMainAlignment::End,
                        Button { label: "Cancel", variant: ButtonVariant::Secondary,
                            onclick: move |_| message.set("Demo: your changes would be discarded here".into()) }
                        Button { label: "Save", variant: ButtonVariant::Primary,
                            onclick: move |_| message.set("Demo: your changes would be saved here".into()) }
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
