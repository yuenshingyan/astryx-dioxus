use dioxus::prelude::*;

use astryx_dioxus::{
    Card, CardVariant, Center, Heading, HeadingLevel, LayerContentBoundary, Layout, LayoutContent,
    LayoutFooter, LayoutHeader, LayoutPanel, SizeValue, SpacingStep, Text, TextColor, VStack,
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
            div { style: if is_full { "width:100%;height:212px" } else { "width:100%;height:148px" },
                Layout {
                    header: rsx! { LayoutHeader { has_divider: true, Card { variant: CardVariant::MUTED } } },
                    start: rsx! { if is_full { LayoutPanel { width: SizeValue::from(140), has_divider: true, Card { variant: CardVariant::MUTED } } } },
                    footer: rsx! { if is_full { LayoutFooter { has_divider: true, Card { variant: CardVariant::MUTED } } } },
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
            }
        }
    }
}

#[component]
fn InsetExample(
    id: &'static str,
    #[props(default)] has_start: bool,
    #[props(default)] has_end: bool,
    #[props(default)] has_dividers: bool,
    #[props(default = SpacingStep::Two)] layout_padding: SpacingStep,
    #[props(default)] padding: Option<SpacingStep>,
    #[props(default = "ltr")] direction: &'static str,
    children: Element,
) -> Element {
    rsx! {
        div { dir: direction, style: "width:600px;height:160px;--layout-padding-inner-x:20px;--layout-padding-inner-y:12px",
            Layout { content_width: SizeValue::from(360), padding: layout_padding,
                header: rsx! { LayoutHeader { has_divider: has_dividers, padding: SpacingStep::Zero, style: "box-sizing:border-box;height:20px", "Header" } },
                footer: rsx! { LayoutFooter { has_divider: has_dividers, padding: SpacingStep::Zero, style: "box-sizing:border-box;height:20px", "Footer" } },
                start: rsx! { if has_start { LayoutPanel { width: SizeValue::from(100), has_divider: true, "Start" } } },
                end: rsx! { if has_end { LayoutPanel { width: SizeValue::from(100), has_divider: true, "End" } } },
                LayoutContent { id, padding, is_scrollable: false, {children} }
            }
        }
    }
}

#[component]
fn InsetBoundaryExamples() -> Element {
    rsx! {
        InsetExample { id: "layout-content-no-panels",
            LayoutContent { id: "layout-content-descendant", is_scrollable: false, style: "height:auto", "Nested region" }
            LayerContentBoundary {
                Layout { header: rsx! { LayoutHeader { has_divider: true, "Nested header" } },
                    footer: rsx! { LayoutFooter { has_divider: true, "Nested footer" } },
                    LayoutContent { id: "layout-content-nested-layout", is_scrollable: false, "Nested Layout" }
                }
            }
        }
        InsetExample { id: "layout-content-start-panel", has_start: true, layout_padding: SpacingStep::Zero,
            div { "Open end" }
        }
        InsetExample { id: "layout-content-end-panel", has_end: true, has_dividers: true, direction: "rtl",
            div { "Open start" }
        }
        InsetExample { id: "layout-content-both-panels", has_start: true, has_end: true,
            div { "Both panels" }
        }
        InsetExample { id: "layout-content-explicit", padding: SpacingStep::Three,
            div { "Explicit inset" }
        }
        InsetExample { id: "layout-content-bleed", padding: SpacingStep::Zero,
            div { style: "height:240px", "Clipped full bleed" }
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
            h3 { "Inset boundaries" }
            InsetBoundaryExamples {}
        }
    }
}
