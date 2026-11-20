use std::borrow::Cow;
use std::fmt::{Display, Formatter};

/// A shared size input: a pixel number or an open-ended CSS dimension value.
///
/// Conversions preserve input; call [`Self::as_css`] at the rendering boundary.
/// CSS grammar and property-specific validity remain the browser's responsibility.
#[derive(Clone, Debug, PartialEq)]
pub enum SizeValue {
    /// Pixels, required to be finite and nonnegative when validated.
    Pixels(f64),
    /// A nonempty CSS value isolated from adjacent declarations.
    Css(Cow<'static, str>),
}

impl SizeValue {
    /// Checks numeric validity and CSS declaration isolation, not CSS grammar.
    /// Functions, custom variables, CSS-wide keywords and future units stay open.
    pub fn validate(&self) -> Result<(), SizeValueError> {
        match self {
            Self::Pixels(value) if !value.is_finite() => Err(SizeValueError::NonFinitePixels),
            Self::Pixels(value) if *value < 0.0 => Err(SizeValueError::NegativePixels),
            Self::Pixels(_) => Ok(()),
            Self::Css(value) if !is_self_contained_css(value) => Err(SizeValueError::InvalidCss),
            Self::Css(_) => Ok(()),
        }
    }

    /// Validates and serializes for an actual CSS dimension declaration.
    ///
    /// Apply the result to `width`, `height`, etc. directly. Storing CSS-wide
    /// keywords in a custom property changes their meaning: `inherit` would
    /// inherit that custom property instead of the dimension. Strings are
    /// preserved (including comments and escapes); numbers acquire `px`.
    pub fn as_css(&self) -> Result<String, SizeValueError> {
        self.validate()?;
        Ok(match self {
            Self::Pixels(value) => format!("{value}px"),
            Self::Css(value) => value.to_string(),
        })
    }

    pub(crate) fn css_wide_keyword(&self) -> Option<CssWideKeyword> {
        let Self::Css(value) = self else {
            return None;
        };
        match css_identifier(value).to_ascii_lowercase().as_str() {
            "inherit" => Some(CssWideKeyword::Inherit),
            "initial" => Some(CssWideKeyword::Initial),
            "unset" => Some(CssWideKeyword::Unset),
            "revert" => Some(CssWideKeyword::Revert),
            "revert-layer" => Some(CssWideKeyword::RevertLayer),
            _ => None,
        }
    }

    /// Whether this is the CSS identifier `auto`, ignoring case, surrounding
    /// whitespace/comments, and identifier escapes. Does not resolve `var()`
    /// or CSS-wide keywords. Component-specific scrolling policy is caller-owned.
    /// Validate first; this predicate alone is not an input-validity check.
    pub fn is_auto(&self) -> bool {
        match self {
            Self::Pixels(_) => false,
            Self::Css(value) => css_identifier(value).eq_ignore_ascii_case("auto"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CssWideKeyword {
    Inherit,
    Initial,
    Unset,
    Revert,
    RevertLayer,
}
impl CssWideKeyword {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Inherit => "inherit",
            Self::Initial => "initial",
            Self::Unset => "unset",
            Self::Revert => "revert",
            Self::RevertLayer => "revert-layer",
        }
    }
}

impl From<&str> for SizeValue {
    fn from(value: &str) -> Self {
        Self::Css(Cow::Owned(value.to_owned()))
    }
}

impl From<String> for SizeValue {
    fn from(value: String) -> Self {
        Self::Css(Cow::Owned(value))
    }
}

macro_rules! pixel_sizes {
    ($($ty:ty),* $(,)?) => {
        $(impl From<$ty> for SizeValue {
            fn from(value: $ty) -> Self {
                Self::Pixels(f64::from(value))
            }
        })*
    };
}

pixel_sizes!(u8, u16, u32, i8, i16, i32, f32, f64);

/// Recoverable failure validating a shared size input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeValueError {
    /// A pixel number is NaN or infinite.
    NonFinitePixels,
    /// A pixel number is less than zero.
    NegativePixels,
    /// CSS is empty or could terminate or consume adjacent declarations.
    InvalidCss,
}

impl Display for SizeValueError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::NonFinitePixels => "size pixels must be finite, not NaN or infinite",
            Self::NegativePixels => "size pixels must be nonnegative",
            Self::InvalidCss => "size CSS must be nonempty and isolated from adjacent declarations",
        })
    }
}

impl std::error::Error for SizeValueError {}

// This is deliberately a declaration-boundary scanner, not a CSS dimension
// allowlist. Complete functions, quoted tokens, escapes and comments remain
// available, including in custom-variable fallbacks.
fn is_self_contained_css(value: &str) -> bool {
    if value.trim().is_empty() || value.contains('\0') {
        return false;
    }
    let mut chars = value.chars().peekable();
    let mut quote = None;
    let mut delimiters = Vec::new();
    let mut has_token = false;
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            let Some(escaped) = chars.next() else {
                return false;
            };
            if escaped == '\r' && chars.peek() == Some(&'\n') {
                chars.next();
            }
            has_token = true;
            continue;
        }
        if let Some(end) = quote {
            if ch == end {
                quote = None;
            } else if matches!(ch, '\n' | '\r' | '\u{c}') {
                return false;
            }
            continue;
        }
        match ch {
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                let mut has_end = false;
                while let Some(comment) = chars.next() {
                    if comment == '*' && chars.peek() == Some(&'/') {
                        chars.next();
                        has_end = true;
                        break;
                    }
                }
                if !has_end {
                    return false;
                }
                continue;
            }
            '\'' | '"' => quote = Some(ch),
            '(' => delimiters.push(')'),
            '[' => delimiters.push(']'),
            '{' => delimiters.push('}'),
            ')' | ']' | '}' if delimiters.pop() != Some(ch) => return false,
            // Priority is not part of a dimension value. A top-level priority
            // annotation could defeat the caller's final style precedence.
            ';' | '!' if delimiters.is_empty() => return false,
            _ => {}
        }
        has_token |= !ch.is_whitespace();
    }
    has_token && quote.is_none() && delimiters.is_empty()
}

// Comments separate tokens; they cannot splice `au/**/to` into one keyword.
// CSS hex escapes consume at most six digits and one optional whitespace.
fn css_identifier(value: &str) -> String {
    let mut decoded = String::new();
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            while let Some(comment) = chars.next() {
                if comment == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    break;
                }
            }
            decoded.push(' ');
        } else if ch == '\\' {
            let mut hex = String::new();
            while let Some(&digit) = chars.peek() {
                if hex.len() == 6 || !digit.is_ascii_hexdigit() {
                    break;
                }
                chars.next();
                hex.push(digit);
            }
            if hex.is_empty() {
                if let Some(escaped) = chars.next() {
                    // Escaped whitespace belongs to the identifier (or is an
                    // invalid newline escape), never surrounding whitespace.
                    decoded.push(if matches!(escaped, ' ' | '\t' | '\n' | '\r' | '\u{c}') {
                        '\u{fffd}'
                    } else {
                        escaped
                    });
                }
            } else {
                if chars
                    .peek()
                    .is_some_and(|ch| matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{c}'))
                {
                    let whitespace = chars.next();
                    if whitespace == Some('\r') && chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                }
                let character = u32::from_str_radix(&hex, 16)
                    .ok()
                    .filter(|code| *code != 0)
                    .and_then(char::from_u32)
                    .unwrap_or('\u{fffd}');
                decoded.push(if matches!(character, ' ' | '\t' | '\n' | '\r' | '\u{c}') {
                    '\u{fffd}'
                } else {
                    character
                });
            }
        } else {
            decoded.push(ch);
        }
    }
    decoded
        .trim_matches([' ', '\t', '\n', '\r', '\u{c}'])
        .to_owned()
}

/// Resting surface elevation; each component owns how its shadow is composed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Elevation {
    /// Flat, with no elevation shadow.
    #[default]
    None,
    /// The `--shadow-low` step.
    Low,
    /// The `--shadow-med` step.
    Med,
    /// The `--shadow-high` step.
    High,
}

impl Elevation {
    /// The original theme-axis value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Low => "low",
            Self::Med => "med",
            Self::High => "high",
        }
    }
}

/// The eleven public Astryx spacing steps (not the larger internal token scale).
/// Omission is intentionally not modeled as a step: it can select theme padding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpacingStep {
    /// Step 0: `--spacing-0` (0px).
    Zero,
    /// Step 0.5: `--spacing-0-5` (2px).
    Half,
    /// Step 1: `--spacing-1` (4px).
    One,
    /// Step 1.5: `--spacing-1-5` (6px).
    OneAndHalf,
    /// Step 2: `--spacing-2` (8px).
    Two,
    /// Step 3: `--spacing-3` (12px).
    Three,
    /// Step 4: `--spacing-4` (16px).
    Four,
    /// Step 5: `--spacing-5` (20px).
    Five,
    /// Step 6: `--spacing-6` (24px).
    Six,
    /// Step 8: `--spacing-8` (32px).
    Eight,
    /// Step 10: `--spacing-10` (40px).
    Ten,
}
