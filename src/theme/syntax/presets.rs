use super::define_syntax_theme::{SyntaxColor, SyntaxThemeDefinition, SyntaxTokenValue};

/// Pinned palettes used in the SyntaxTheme catalog examples.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntaxThemePreset {
    OneDarkPro,
    Dracula,
    GitHubLight,
}

impl SyntaxThemePreset {
    /// Returns the preset's syntax palette.
    pub fn definition(self) -> SyntaxThemeDefinition {
        let (name, colors) = match self {
            Self::OneDarkPro => (
                "one-dark-pro",
                [
                    "#c678dd", "#98c379", "#5c6370", "#d19a66", "#61afef", "#e5c07b", "#e06c75",
                    "#56b6c2", "#d19a66", "#e06c75", "#d19a66", "#e06c75", "#abb2bf", "#282c34",
                ],
            ),
            Self::Dracula => (
                "dracula",
                [
                    "#ff79c6", "#f1fa8c", "#6272a4", "#bd93f9", "#50fa7b", "#8be9fd", "#f8f8f2",
                    "#ff79c6", "#bd93f9", "#ff79c6", "#50fa7b", "#66d9ef", "#f8f8f2", "#282a36",
                ],
            ),
            Self::GitHubLight => (
                "github-light",
                [
                    "#cf222e", "#0a3069", "#6e7781", "#0550ae", "#8250df", "#953800", "#24292f",
                    "#cf222e", "#0550ae", "#116329", "#0550ae", "#0550ae", "#24292f", "#ffffff",
                ],
            ),
        };
        SyntaxThemeDefinition {
            name: name.to_owned(),
            colors: colors.map(|value| SyntaxTokenValue::Color(SyntaxColor::preset(value))),
            diff_colors: [None, None, None, None, None],
        }
    }
}
