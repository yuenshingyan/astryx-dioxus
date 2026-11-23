// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

mod component;
mod media_theme;
mod syntax;
/// Shared typography contracts owned by the upstream theme module.
pub mod types;

pub use component::{InvalidThemeName, Theme, ThemeMode, ThemeName, ThemeProps};
pub use media_theme::{MediaTheme, MediaThemeFallback, MediaThemeMode, MediaThemeProps};
pub use syntax::{
    InvalidSyntaxColor, SyntaxColor, SyntaxTheme, SyntaxThemeDefinition, SyntaxThemePreset,
    SyntaxThemeProps, SyntaxTokenValue,
};

pub use types::{
    InvalidTextName, TextColor, TextDisplay, TextJustify, TextSize, TextType, TextWeight, TextWrap,
    WordBreak,
};
