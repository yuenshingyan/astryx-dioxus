use dioxus::prelude::*;

use astryx_dioxus::{
    Card, CardVariant, Center, LayoutContent, LayoutFooter, LayoutHeader, LayoutPanel, SizeValue,
};

#[component]
fn PanelPattern(id: &'static str, #[props(default)] is_full: bool) -> Element {
    rsx! {
        Center { width: SizeValue::from(400), max_width: SizeValue::from("100%"),
            div { style: "width:100%;min-width:0",
                if is_full { LayoutHeader { has_divider: true, Card { variant: CardVariant::MUTED } } }
                div { style: "display:flex;min-height:0",
                    LayoutPanel { id, width: SizeValue::from(140), has_divider: true,
                        class: "astryx-layout-panel--start", role: "navigation",
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
                    LayoutContent { Card { variant: CardVariant::MUTED } }
                }
                if is_full { LayoutFooter { has_divider: true, Card { variant: CardVariant::MUTED } } }
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
        }
    }
}
