use dioxus::prelude::*;

use astryx_dioxus::{SyntaxTheme, SyntaxThemePreset};

#[component]
fn PalettePreview(preset: SyntaxThemePreset) -> Element {
    rsx! {
        SyntaxTheme { theme: preset.definition(),
            pre { style: "padding:12px;background:var(--color-syntax-background);color:var(--color-syntax-punctuation)",
                code {
                    span { style: "color:var(--color-syntax-keyword)", "const" }
                    " greeting = "
                    span { style: "color:var(--color-syntax-string)", "\"Hello\"" }
                    ";"
                }
            }
        }
    }
}

#[component]
pub(super) fn SyntaxThemeExamples() -> Element {
    rsx! {
        section { id: "syntax-theme-examples",
            h2 { "SyntaxTheme" }
            p { "These samples apply the palette to hand-marked code. CodeBlock is not yet ported." }
            h3 { "Showcase: One Dark Pro" }
            PalettePreview { preset: SyntaxThemePreset::OneDarkPro }
            h3 { "Dark preset: Dracula" }
            PalettePreview { preset: SyntaxThemePreset::Dracula }
            h3 { "Light preset: GitHub Light" }
            PalettePreview { preset: SyntaxThemePreset::GitHubLight }
        }
    }
}
