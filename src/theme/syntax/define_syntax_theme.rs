use std::fmt::{self, Write};

pub(super) const COLOR_KEYS: [&str; 14] = [
    "keyword",
    "string",
    "comment",
    "number",
    "function",
    "type",
    "variable",
    "operator",
    "constant",
    "tag",
    "attribute",
    "property",
    "punctuation",
    "background",
];
const DIFF_KEYS: [&str; 5] = [
    "added",
    "removed",
    "diff-header",
    "added-marker",
    "removed-marker",
];
const DIFF_DEFAULTS: [&str; 5] = [
    "color-mix(in srgb, var(--color-success-muted) 25%, transparent)",
    "color-mix(in srgb, var(--color-error-muted) 25%, transparent)",
    "transparent",
    "var(--color-syntax-punctuation)",
    "var(--color-syntax-punctuation)",
];

/// Single CSS color expression that cannot escape its custom-property declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyntaxColor(String);

/// A color expression contains unsupported syntax.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidSyntaxColor;

impl fmt::Display for InvalidSyntaxColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("expected one nonempty CSS color expression")
    }
}

impl std::error::Error for InvalidSyntaxColor {}

impl SyntaxColor {
    /// Accepts a single color expression, including modern color functions and variable references.
    pub fn new(value: impl AsRef<str>) -> Result<Self, InvalidSyntaxColor> {
        let value = value.as_ref().trim();
        let mut depth = 0_u32;
        if value.is_empty() || value.contains("/*") || value.contains("*/") {
            return Err(InvalidSyntaxColor);
        }
        for byte in value.bytes() {
            match byte {
                b'(' => depth += 1,
                b')' if depth > 0 => depth -= 1,
                b',' if depth > 0 => (),
                b'#' | b'.' | b'%' | b'-' | b'_' | b'+' | b'/' | b' ' => (),
                byte if byte.is_ascii_alphanumeric() => (),
                _ => return Err(InvalidSyntaxColor),
            }
        }
        if depth != 0 {
            return Err(InvalidSyntaxColor);
        }
        Ok(Self(value.to_owned()))
    }

    pub(super) fn preset(value: &str) -> Self {
        Self(value.to_owned())
    }
}

/// A mode-independent color or a pair selected by the browser's color scheme.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyntaxTokenValue {
    Color(SyntaxColor),
    LightDark {
        light: SyntaxColor,
        dark: SyntaxColor,
    },
}

impl fmt::Display for SyntaxTokenValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Color(color) => f.write_str(&color.0),
            Self::LightDark { light, dark } => write!(f, "light-dark({}, {})", light.0, dark.0),
        }
    }
}

/// Complete syntax palette with optional diff colors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyntaxThemeDefinition {
    pub name: String,
    /// Keyword, string, comment, number, function, type, variable, operator,
    /// constant, tag, attribute, property, punctuation, and background, in that order.
    pub colors: [SyntaxTokenValue; 14],
    /// Added row, removed row, diff header, added marker, and removed marker, in that order.
    pub diff_colors: [Option<SyntaxTokenValue>; 5],
}

impl SyntaxThemeDefinition {
    pub(super) fn declarations(&self) -> String {
        let mut style = String::new();
        for (key, color) in COLOR_KEYS.into_iter().zip(&self.colors) {
            let _ = write!(style, "--color-syntax-{key}:{color};");
        }
        for ((key, default), color) in DIFF_KEYS
            .into_iter()
            .zip(DIFF_DEFAULTS)
            .zip(&self.diff_colors)
        {
            if let Some(color) = color {
                let _ = write!(style, "--color-syntax-{key}:{color};");
            } else {
                let _ = write!(style, "--color-syntax-{key}:{default};");
            }
        }
        style
    }
}
