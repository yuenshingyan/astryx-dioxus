use dioxus::prelude::*;

use astryx_dioxus::{
    Card, CardVariant, Center, Divider, Layout, LayoutContent, LayoutFooter, LayoutHeader,
    LayoutHeight, LayoutPanel, SizeValue, SpacingStep,
};

#[component]
fn PanelPattern(id: &'static str, #[props(default)] is_full: bool) -> Element {
    rsx! {
        Center { width: SizeValue::from(400), max_width: SizeValue::from("100%"),
            Layout { style: "width:100%;min-width:0",
                header: rsx! { if is_full { LayoutHeader { has_divider: true, Card { variant: CardVariant::MUTED } } } },
                footer: rsx! { if is_full { LayoutFooter { has_divider: true, Card { variant: CardVariant::MUTED } } } },
                start: rsx! {
                    LayoutPanel { id, width: SizeValue::from(140), has_divider: true, role: "navigation",
                        label: if is_full { "Dashboard navigation" } else { "Example navigation" },
                        ul { style: "list-style:none;margin:0;padding:0",
                            for name in ["Overview", "Analytics", "Reports", "Settings"] {
                                if is_full || name != "Reports" {
                                    li { key: "{name}", "aria-current": (name == "Overview").then_some("page"),
                                        style: if name == "Overview" { "padding:8px;background:var(--color-background-muted);border-radius:var(--radius-inner)" } else { "padding:8px" },
                                        "{name}"
                                    }
                                }
                            }
                        }
                    }
                },
                LayoutContent { Card { variant: CardVariant::MUTED } }
            }
        }
    }
}

#[component]
fn PanelGeometry(is_narrow: bool) -> Element {
    rsx! {
        Card { id: "layout-panel-geometry", variant: CardVariant::TRANSPARENT, padding: SpacingStep::Two,
            width: SizeValue::from(260),
            Layout { height: LayoutHeight::Auto, padding: SpacingStep::Six, default_has_dividers: true,
                header: rsx! { LayoutHeader { "Header" } },
                start: rsx! {
                    LayoutPanel { id: "layout-panel-gap", width: SizeValue::from("calc(100px + 12px)"),
                        is_scrollable: false,
                        Divider { is_full_bleed: true }
                    }
                },
                end: rsx! {
                    LayoutPanel { id: "layout-panel-divider", width: SizeValue::from(if is_narrow { 0 } else { 160 }),
                        has_divider: true, is_scrollable: false, role: "complementary", label: "Inspector",
                        Divider { is_full_bleed: true }
                    }
                },
                div { id: "layout-panel-neighbor", style: "color:var(--color-border);height:32px" }
            }
            Layout { height: LayoutHeight::Auto, padding: SpacingStep::Six,
                footer: rsx! { LayoutFooter { "Footer" } },
                start: rsx! {
                    LayoutPanel { id: "layout-panel-explicit", width: SizeValue::from(112), padding: SpacingStep::OneAndHalf,
                        is_scrollable: false,
                        Divider { is_full_bleed: true }
                    }
                },
                end: rsx! {
                    LayoutPanel { id: "layout-panel-bleed", width: SizeValue::from(112), padding: SpacingStep::Zero,
                        is_scrollable: false,
                        Divider { is_full_bleed: true }
                    }
                },
                div { style: "height:32px" }
            }
        }
    }
}

#[component]
pub(super) fn LayoutPanelExamples() -> Element {
    let mut is_narrow = use_signal(|| false);
    rsx! {
        section { id: "layout-panel-examples", "data-astryx-theme": "neutral",
            h2 { "LayoutPanel" }
            h3 { "Navigation" }
            PanelPattern { id: "layout-panel-navigation" }
            h3 { "Showcase" }
            PanelPattern { id: "layout-panel-showcase", is_full: true }
            h3 { "Fixed-width overflow" }
            button { id: "layout-panel-toggle-width", onclick: move |_| is_narrow.toggle(), "Toggle width" }
            LayoutPanel { id: "layout-panel-scroll", width: SizeValue::from(if is_narrow() { 0 } else { 140 }),
                style: "height:96px", scroll_label: "Scrollable panel example",
                div { style: "height:240px", "Panel content" }
            }
            h3 { "Side-slot insets and dividers" }
            PanelGeometry { is_narrow: is_narrow() }
        }
    }
}
