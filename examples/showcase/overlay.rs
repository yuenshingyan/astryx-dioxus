use dioxus::prelude::*;

use astryx_dioxus::{
    AspectRatio, Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Overlay, OverlayAlign,
    OverlayPosition, OverlayShowOn, SpacingStep, VStack,
};

const LANDSCAPE: Asset = asset!("/tests/fixtures/media-theme/light-scene-horizontal-1.png");
const WORKSPACE: Asset = asset!("/tests/fixtures/overlay/light-working-horizontal-1.png");
const HIGHLIGHTS: Asset = asset!("/tests/fixtures/overlay/illustrative-horizontal-1.png");

#[component]
fn Picture(src: Asset, alt: &'static str) -> Element {
    rsx! { AspectRatio { ratio: 16.0 / 9.0,
        img { src: src.to_string(), alt, style: "width:100%;height:100%;object-fit:cover" }
    } }
}

#[component]
pub(super) fn OverlayExamples() -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        section { id: "overlay-examples", "data-astryx-theme": "neutral",
            h2 { "Overlay" }
            h3 { "Showcase" }
            Overlay { id: "overlay-showcase", style: "width:520px;max-width:100%;border-radius:16px", align: OverlayAlign::Center,
                content: rsx! { VStack { gap: SpacingStep::Two, style: "text-align:center",
                    strong { "Design system foundations" }
                    Button { label: "Open gallery", variant: ButtonVariant::Secondary, size: ButtonSize::Small }
                } },
                Picture { src: LANDSCAPE, alt: "Abstract landscape" }
            }
            h3 { "Bottom strip" }
            Overlay { id: "overlay-strip", style: "width:420px;max-width:100%;border-radius:12px", position: OverlayPosition::Bottom, align: OverlayAlign::Start,
                content: rsx! { VStack { gap: SpacingStep::One,
                    Badge { label: "New", variant: BadgeVariant::GREEN }
                    strong { "Weekly product highlights" }
                    span { "12 updates across templates and tokens" }
                } },
                Picture { src: HIGHLIGHTS, alt: "Product highlight preview" }
            }
            h3 { "Hover or focus reveal" }
            Overlay { id: "overlay-hover", tabindex: "0", style: "width:420px;max-width:100%;border-radius:12px", show_on: OverlayShowOn::Hover, align: OverlayAlign::Center,
                content: rsx! { Button { label: "Quick view", variant: ButtonVariant::Secondary, size: ButtonSize::Small } },
                Picture { src: WORKSPACE, alt: "Workspace preview" }
            }
            h3 { "Controlled" }
            button { id: "overlay-control", r#type: "button", onclick: move |_| is_open.toggle(), "Toggle overlay" }
            Overlay { id: "overlay-controlled", style: "width:240px;max-width:100%", is_open: is_open(), position: OverlayPosition::Top,
                content: rsx! { Button { label: "Details" } },
                Picture { src: LANDSCAPE, alt: "Landscape" }
            }
        }
    }
}
