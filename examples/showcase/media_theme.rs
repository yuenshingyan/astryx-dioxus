use dioxus::prelude::*;

use astryx_dioxus::{
    AspectRatio, Badge, Button, ButtonSize, ButtonVariant, MediaTheme, MediaThemeMode, Section,
    SectionVariant, SpacingStep, Text, TextType, TextWeight, VStack,
};

const LANDSCAPE: Asset = asset!("/tests/fixtures/media-theme/light-scene-horizontal-1.png");
const ROOM: Asset = asset!("/tests/fixtures/media-theme/light-home-square-1.png");
const FRAME: &str =
    "width:360px;max-width:100%;border-radius:var(--radius-container);overflow:hidden";

#[component]
pub(super) fn MediaThemeExamples() -> Element {
    let mut phase = use_signal(|| 0);
    let (mode, background) = match phase() {
        0 => (MediaThemeMode::Auto, "#000"),
        1 => (MediaThemeMode::Auto, "#fff"),
        2 => (MediaThemeMode::Auto, "linear-gradient(white, black)"),
        3 => (MediaThemeMode::Light, "#fff"),
        _ => (MediaThemeMode::Off, "#fff"),
    };
    rsx! {
        section { id: "media-theme-examples", "data-astryx-theme": "neutral",
            h2 { "MediaTheme" }
            h3 { "Media overlay" }
            Section { variant: SectionVariant::TRANSPARENT, padding: SpacingStep::Four,
                style: "{FRAME};min-height:230px;display:flex;align-items:flex-end;background:center / cover linear-gradient(180deg,#0a13170d,#0a1317d1),center / cover url('{LANDSCAPE}');box-shadow:var(--shadow-med)",
                MediaTheme { mode: MediaThemeMode::Dark,
                    VStack { gap: SpacingStep::Three,
                        div { Text { weight: TextWeight::Bold, "Media overlay " } Badge { label: "Live" } }
                        Text { text_type: TextType::SUPPORTING, "Text, icons, badges, and button variants inherit legible colors on top of the dark image treatment." }
                        div { style: "display:flex;gap:8px;flex-wrap:wrap",
                            Button { label: "Primary", size: ButtonSize::Small }
                            Button { label: "Secondary", size: ButtonSize::Small, variant: ButtonVariant::Secondary }
                            Button { label: "Ghost", size: ButtonSize::Small, variant: ButtonVariant::Ghost }
                        }
                    }
                }
            }
            h3 { "Image overlay" }
            AspectRatio { ratio: 16.0 / 9.0, style: FRAME,
                img { src: LANDSCAPE.to_string(), alt: "Landscape", style: "width:100%;height:100%;object-fit:cover" }
                Section { variant: SectionVariant::TRANSPARENT, padding: SpacingStep::Four,
                    style: "position:absolute;inset-inline:0;inset-block-end:0;background:linear-gradient(180deg,transparent,#0a1317c7)",
                    MediaTheme { mode: MediaThemeMode::Dark,
                        VStack { gap: SpacingStep::Two,
                            Text { weight: TextWeight::Bold, "Product launch livestream" }
                            Text { text_type: TextType::SUPPORTING, "MediaTheme keeps overlay text and controls readable without hard-coded color overrides." }
                            div { style: "display:flex;gap:8px;flex-wrap:wrap",
                                Button { label: "Watch", size: ButtonSize::Small }
                                Button { label: "Details", size: ButtonSize::Small, variant: ButtonVariant::Secondary }
                            }
                        }
                    }
                }
            }
            h3 { "Light scrim" }
            AspectRatio { ratio: 16.0 / 9.0, style: FRAME,
                img { src: ROOM.to_string(), alt: "Bright room", style: "width:100%;height:100%;object-fit:cover" }
                Section { variant: SectionVariant::TRANSPARENT, padding: SpacingStep::Four,
                    style: "position:absolute;inset-block-start:16px;inset-inline-start:16px;max-width:250px;background:#ffffffd1;border-radius:var(--radius-container);box-shadow:var(--shadow-med)",
                    MediaTheme { mode: MediaThemeMode::Light,
                        VStack { gap: SpacingStep::Two,
                            Text { weight: TextWeight::Bold, "Bright media surface" }
                            Text { text_type: TextType::SUPPORTING, "Use mode=\"light\" when content sits on a light card or scrim." }
                            Button { label: "Open", size: ButtonSize::Small, variant: ButtonVariant::Ghost }
                        }
                    }
                }
            }
            h3 { "Automatic mode" }
            button { id: "media-theme-cycle", onclick: move |_| phase.set((phase() + 1) % 5), "Change media surface" }
            div { id: "media-theme-probe", style: "background:{background};color-scheme:light;padding:8px",
                MediaTheme { mode, input { id: "media-theme-retained", "aria-label": "Retained input" } }
            }
        }
    }
}
