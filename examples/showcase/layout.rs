use dioxus::prelude::*;

use astryx_dioxus::{
    Badge, BadgeVariant, Button, ButtonVariant, Card, HStack, Heading, HeadingLevel, Layout,
    LayoutContent, LayoutFooter, LayoutHeader, LayoutHeight, LayoutPanel, Section, SectionVariant,
    SizeValue, SpacingStep, StackMainAlignment, Text, VStack,
};

#[component]
fn Band(title: &'static str) -> Element {
    rsx! { LayoutHeader { has_divider: true, Heading { level: HeadingLevel::Four, "{title}" } } }
}

#[component]
fn Actions(primary: Option<&'static str>, secondary: Option<&'static str>) -> Element {
    rsx! { LayoutFooter { has_divider: true,
        HStack { gap: SpacingStep::Two, h_align: StackMainAlignment::End,
            if let Some(label) = secondary { Button { label, variant: ButtonVariant::Secondary } }
            if let Some(label) = primary { Button { label, variant: ButtonVariant::Primary } }
        }
    } }
}

#[component]
fn DemoList(items: &'static [&'static str], selected: &'static str) -> Element {
    rsx! { ul { style: "list-style:none;margin:0;padding:0",
        for &item in items {
            li { key: "{item}", "aria-current": (item == selected).then_some("page"),
                style: if item == selected { "padding:8px;background:var(--color-background-muted)" } else { "padding:8px" }, "{item}" }
        }
    } }
}

#[component]
fn GeometryProbes() -> Element {
    rsx! {
        h3 { "Geometry and inset boundaries" }
        Layout { id: "layout-arbitrary-content", style: "width:600px;height:180px", content_width: SizeValue::from(360),
            start: rsx! { LayoutPanel { width: SizeValue::from(120), padding: SpacingStep::Zero,
                LayoutContent { padding: SpacingStep::Zero, "Panel content" }
            } },
            section { id: "layout-arbitrary-lane", "Arbitrary main content" }
        }
        for (id, width) in [("layout-string-zero", SizeValue::from("0")), ("layout-numeric-zero", SizeValue::from(0))] {
            Layout { key: "{id}", id, style: "width:600px;height:100px", padding: SpacingStep::Zero, content_width: width,
                LayoutContent { "Zero alignment width" }
            }
        }
        for (id, height) in [("layout-fill-probe", LayoutHeight::Fill), ("layout-auto-probe", LayoutHeight::Auto)] {
            Section { key: "{id}", width: SizeValue::from(600), height: SizeValue::from(180),
                padding_inline_start: SpacingStep::Two, padding_inline_end: SpacingStep::Three,
                padding_block_start: SpacingStep::Four, padding_block_end: SpacingStep::Six,
                Layout { id, height,
                    header: rsx! { LayoutHeader { "Title" } },
                    LayoutContent { div { style: "height:300px", "Tall content" } }
                }
            }
        }
        Section { width: SizeValue::from(600), padding: SpacingStep::Six,
            Layout { padding: SpacingStep::Zero, content_width: SizeValue::from(360),
                header: rsx! { LayoutHeader { id: "layout-outer-inset", "Outer title" } },
                Layout { id: "layout-nested-inset", role: "region", aria_label: "Nested workspace",
                    header: rsx! { LayoutHeader { "Nested title" } },
                    LayoutContent { "Nested content" }
                }
            }
        }
    }
}

#[component]
pub(super) fn LayoutExamples() -> Element {
    let mut has_panel = use_signal(|| true);
    rsx! {
        section { id: "layout-examples", "data-astryx-theme": "neutral",
            h2 { "Layout — original arrangements" }
            h3 { "Basic card layout" }
            Card { height: SizeValue::from(300), width: SizeValue::from("100%"), style: "max-width:400px",
                Layout { id: "layout-basic", header: rsx! { Band { title: "Card Title" } },
                    footer: rsx! { Actions { secondary: "Cancel", primary: "Save" } },
                    LayoutContent { VStack { gap: SpacingStep::Three,
                        Text { "This is a basic card layout with a header, scrollable content area, and footer with actions." }
                        Text { "When content exceeds the available height, the content area scrolls while the header and footer stay in place." }
                        Text { "This pattern works well for modal dialogs, detail panels, and other constrained containers." }
                        Text { "The dividers between header, content, and footer provide clear visual separation." }
                    } }
                }
            }
            h3 { "Content only" }
            Card { width: SizeValue::from("100%"), style: "max-width:400px",
                Layout { id: "layout-content-only", LayoutContent { VStack { gap: SpacingStep::Three,
                    Heading { level: HeadingLevel::Four, "Simple Content" }
                    Text { "Layout can render just the content area without a header, footer, or side panels." }
                } } }
            }
            h3 { "Content width" }
            Layout { id: "layout-width", content_width: SizeValue::from(360),
                header: rsx! { Band { title: "Centered Form" } }, footer: rsx! { Actions { secondary: "Cancel", primary: "Submit" } },
                LayoutContent { VStack { gap: SpacingStep::Three,
                    Text { "The contentWidth prop constrains content to a maximum width and centers it horizontally." }
                    Text { "Common widths: 640 for forms, 960 for content pages." }
                } }
            }
            h3 { "Dual panels" }
            Card { width: SizeValue::from("100%"), style: "max-width:500px",
                Layout { id: "layout-dual", header: rsx! { Band { title: "File Browser" } },
                    start: rsx! { LayoutPanel { width: SizeValue::from(120), has_divider: true,
                        Text { "Folders" }
                        DemoList { items: &["Documents", "Projects", "Downloads"], selected: "Projects" }
                    } },
                    end: rsx! { LayoutPanel { width: SizeValue::from(120), has_divider: true,
                        Text { "Details" } Text { "Select a file to view details" }
                    } },
                    LayoutContent { Text { "Files" } Text { "Select a folder to view its contents" } }
                }
            }
            h3 { "Full bleed content" }
            Card { width: SizeValue::from("100%"), style: "max-width:400px",
                Layout { id: "layout-bleed", header: rsx! { Band { title: "Full Bleed Example" } },
                    footer: rsx! { Actions { secondary: "Close" } },
                    LayoutContent { padding: SpacingStep::Zero,
                        Section { variant: SectionVariant::MUTED,
                            Text { "Section automatically escapes the parent container padding to fill edge-to-edge. Useful for wash backgrounds, tables, or images that need to span the full width." }
                        }
                    }
                }
            }
            h3 { "Showcase" }
            Section { padding: SpacingStep::Four,
                Layout { id: "layout-showcase",
                    header: rsx! { LayoutHeader { has_divider: true,
                        HStack { gap: SpacingStep::Two,
                            Heading { level: HeadingLevel::Four, "Projects" }
                            Badge { variant: BadgeVariant::INFO, label: "3 active" }
                        }
                    } },
                    start: rsx! { LayoutPanel { width: SizeValue::from(140), has_divider: true,
                        DemoList { items: &["Dashboard", "Analytics", "Settings"], selected: "Dashboard" }
                    } },
                    footer: rsx! { Actions { primary: "New Project" } },
                    LayoutContent { VStack { gap: SpacingStep::Two,
                        Heading { level: HeadingLevel::Five, "Welcome back" }
                        Text { "You have 3 active projects and 2 pending reviews." }
                    } }
                }
            }
            h3 { "Sidebar" }
            Card { width: SizeValue::from("100%"), style: "max-width:500px",
                Layout { id: "layout-sidebar", header: rsx! { Band { title: "Settings" } },
                    start: rsx! { LayoutPanel { width: SizeValue::from(150), has_divider: true, role: "navigation",
                        DemoList { items: &["General", "Account", "Privacy", "Notifications", "Security"], selected: "General" }
                    } },
                    footer: rsx! { Actions { secondary: "Reset", primary: "Save Changes" } },
                    LayoutContent { VStack { gap: SpacingStep::Three,
                        Heading { level: HeadingLevel::Five, "General Settings" }
                        Text { "Configure your general preferences here. The sidebar navigation stays visible while content scrolls." }
                    } }
                }
            }
            h3 { "Slot changes" }
            button { id: "layout-toggle-panel", onclick: move |_| has_panel.toggle(), "Toggle side panel" }
            Layout { id: "layout-slot-changes", style: "width:600px;height:180px", content_width: SizeValue::from(360),
                padding: SpacingStep::Two,
                start: rsx! { if has_panel() { LayoutPanel { id: "layout-test-panel", width: SizeValue::from(120), has_divider: true, "Start" } } },
                LayoutContent { id: "layout-test-content", div { style: "height:340px", "Content" } }
            }
            GeometryProbes {}
        }
    }
}
