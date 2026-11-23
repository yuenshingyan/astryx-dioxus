use dioxus::prelude::*;

use astryx_dioxus::{
    Badge, BadgeVariant, Button, ButtonVariant, Card, Theme, ThemeMode, ThemeName,
};

const DEMO_THEMES: &str = r#"
[data-astryx-theme="warm-docs"] {
  --color-accent: light-dark(#d97706, #fbbf24);
  --color-background-surface: light-dark(#fff7ed, #1f1300);
  --color-background-card: light-dark(#fffbeb, #2a1a05);
  --color-text-primary: light-dark(#3b2f15, #fef3c7);
  --color-text-secondary: light-dark(#92400e, #fcd34d);
  --color-border: light-dark(#fed7aa, #92400e66);
  --radius-container: 20px;
}
[data-astryx-theme="forest-docs"] {
  --color-accent: light-dark(#15803d, #86efac);
  --color-background-surface: light-dark(#f0fdf4, #052e16);
  --color-background-card: light-dark(#ffffff, #0f3d24);
  --color-text-primary: light-dark(#052e16, #dcfce7);
  --color-text-secondary: light-dark(#166534, #bbf7d0);
  --color-border: light-dark(#bbf7d0, #15803d66);
  --radius-container: 8px;
}
"#;

#[component]
pub(super) fn ThemeExamples() -> Element {
    let mut is_dark = use_signal(|| false);
    let mode = if is_dark() {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    };
    rsx! {
        section { id: "theme-examples",
            h2 { "Theme" }
            style { {DEMO_THEMES} }
            Button { label: "Switch preview scheme", onclick: move |_| is_dark.toggle() }
            div { style: "display:flex;flex-wrap:wrap;gap:var(--spacing-3)",
                for (name, label) in [("warm-docs", "Warm"), ("forest-docs", "Forest")] {
                    Theme { key: "{name}", theme: ThemeName::new(name)?, mode,
                        ThemeCard { label }
                    }
                }
            }
            Theme { theme: ThemeName::NEUTRAL, mode: ThemeMode::Dark,
                Theme { theme: ThemeName::CORE,
                    div { id: "theme-system-in-dark", "Nested system scheme" }
                }
            }
        }
    }
}

#[component]
fn ThemeCard(label: &'static str) -> Element {
    rsx! {
        Card {
            h3 { "{label}" }
            Badge { label: "Active", variant: BadgeVariant::SUCCESS }
            p { "The same content inherits the provider’s colors and radius." }
            Button { label: "Primary" }
            Button { label: "Secondary", variant: ButtonVariant::Secondary }
            Button { label: "Ghost", variant: ButtonVariant::Ghost }
        }
    }
}
