use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use dioxus::events::onmounted as mounted_listener;
use dioxus::prelude::*;

/// A Skeleton dimension: a pixel number or an application-supplied CSS value.
#[derive(Clone, Debug, PartialEq)]
pub enum SkeletonLength {
    /// Finite, nonnegative pixels, validated when the component renders.
    Pixels(f64),
    /// A nonempty, self-contained CSS value. The browser validates its grammar.
    Css(Cow<'static, str>),
}

impl Default for SkeletonLength {
    fn default() -> Self {
        Self::Css(Cow::Borrowed("100%"))
    }
}

impl From<&str> for SkeletonLength {
    fn from(value: &str) -> Self {
        Self::Css(Cow::Owned(value.to_owned()))
    }
}

impl From<String> for SkeletonLength {
    fn from(value: String) -> Self {
        Self::Css(Cow::Owned(value))
    }
}

macro_rules! pixel_lengths {
    ($($ty:ty),* $(,)?) => {
        $(impl From<$ty> for SkeletonLength {
            fn from(value: $ty) -> Self {
                Self::Pixels(f64::from(value))
            }
        })*
    };
}

pixel_lengths!(u8, u16, u32, i8, i16, i32, f32, f64);

impl SkeletonLength {
    fn css_value(&self) -> Option<String> {
        match self {
            Self::Pixels(value) if value.is_finite() && *value >= 0.0 => Some(format!("{value}px")),
            Self::Css(value) if is_self_contained_css(value) => Some(value.to_string()),
            _ => None,
        }
    }
}

// Values are serialized beside other declarations. Reject a value that could
// terminate its declaration or consume the following declarations; ordinary CSS
// functions, quoted tokens, escapes and complete comments remain open-ended.
fn is_self_contained_css(value: &str) -> bool {
    if value.trim().is_empty() || value.contains('\0') {
        return false;
    }
    let mut chars = value.chars().peekable();
    let mut quote = None;
    let mut delimiters = Vec::new();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if chars.next().is_none() {
                return false;
            }
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
            }
            '\'' | '"' => quote = Some(ch),
            '(' => delimiters.push(')'),
            '[' => delimiters.push(']'),
            '{' => delimiters.push('}'),
            ')' | ']' | '}' if delimiters.pop() != Some(ch) => return false,
            ';' if delimiters.is_empty() => return false,
            _ => {}
        }
    }
    quote.is_none() && delimiters.is_empty()
}

/// Original numeric radius levels, plus literal square and fully rounded shapes.
/// Levels Three and Four intentionally share the container token.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SkeletonRadius {
    /// Literal zero, independent of the zero-radius token.
    None,
    /// Original level 0: `--radius-none`.
    Zero,
    /// Original level 1: `--radius-inner`.
    One,
    /// Original level 2: `--radius-element`.
    Two,
    /// Original level 3: `--radius-container` (default).
    #[default]
    Three,
    /// Original level 4: also `--radius-container`.
    Four,
    /// `--radius-full`, suitable for an avatar or pill.
    Rounded,
}

impl SkeletonRadius {
    fn css_value(self) -> &'static str {
        match self {
            Self::None => "0px",
            Self::Zero => "var(--radius-none)",
            Self::One => "var(--radius-inner)",
            Self::Two => "var(--radius-element)",
            Self::Three | Self::Four => "var(--radius-container)",
            Self::Rounded => "var(--radius-full)",
        }
    }
}

/// Recoverable invalid dimension input. CSS strings remain browser-validated;
/// only empty or declaration-breaking strings are rejected here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkeletonError {
    /// Invalid width.
    InvalidWidth,
    /// Invalid height.
    InvalidHeight,
}

impl Display for SkeletonError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let axis = match self {
            Self::InvalidWidth => "width",
            Self::InvalidHeight => "height",
        };
        write!(
            formatter,
            "Skeleton {axis} must be finite nonnegative pixels or a nonempty self-contained CSS value"
        )
    }
}

impl std::error::Error for SkeletonError {}

// CSS-wide keywords cannot be stored as custom-property values: `inherit`
// would inherit the private variable, not the dimension. Private classes apply
// these five keywords to the real property while keeping caller CSS precedence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CssWideKeyword {
    Inherit,
    Initial,
    Unset,
    Revert,
    RevertLayer,
}

impl CssWideKeyword {
    fn as_str(self) -> &'static str {
        match self {
            Self::Inherit => "inherit",
            Self::Initial => "initial",
            Self::Unset => "unset",
            Self::Revert => "revert",
            Self::RevertLayer => "revert-layer",
        }
    }
}

fn dimension_keyword(value: &str) -> Option<CssWideKeyword> {
    let mut without_comments = String::new();
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
            without_comments.push(' ');
        } else if ch == '\\' {
            let mut hex = String::new();
            while let Some(&digit) = chars.peek() {
                if hex.len() == 6 || !digit.is_ascii_hexdigit() {
                    break;
                }
                chars.next();
                hex.push(digit);
            }
            let decoded = if !hex.is_empty() {
                if chars
                    .peek()
                    .is_some_and(|ch| matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{c}'))
                {
                    let whitespace = chars.next();
                    if whitespace == Some('\r') && chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                }
                let code = u32::from_str_radix(&hex, 16).ok()?;
                char::from_u32(code)?
            } else {
                chars.next()?
            };
            // Escaped whitespace belongs to the identifier, not its surrounding trivia.
            if !decoded.is_ascii_alphabetic() && decoded != '-' {
                return None;
            }
            without_comments.push(decoded);
        } else {
            without_comments.push(ch);
        }
    }
    [
        CssWideKeyword::Inherit,
        CssWideKeyword::Initial,
        CssWideKeyword::Unset,
        CssWideKeyword::Revert,
        CssWideKeyword::RevertLayer,
    ]
    .into_iter()
    .find(|keyword| {
        without_comments
            .trim_matches([' ', '\t', '\n', '\r', '\u{c}'])
            .eq_ignore_ascii_case(keyword.as_str())
    })
}

/// Decorative, CSS-only loading shape. The application owns the surrounding
/// busy state and replaces the shape with real content when loading finishes.
///
/// Width/height default to 100%; numbers are pixels. Pass `index` to stagger the
/// pulse by 100ms per block after a 1000ms initial delay. Reduced motion removes
/// the pulse. Native `aria-hidden` can override the default exclusion from AT.
/// Geometry is not reflected as a public theme axis; `.astryx-skeleton` is the
/// sole target. Native styling and event builders reach the childless div.
#[component]
pub fn Skeleton(
    /// Pixel number or CSS width.
    #[props(default, into)]
    width: SkeletonLength,
    /// Pixel number or CSS height.
    #[props(default, into)]
    height: SkeletonLength,
    /// Original radius scale, default level Three.
    #[props(default)]
    radius: SkeletonRadius,
    /// Nonnegative stagger index. Delay arithmetic is widened to avoid overflow.
    #[props(default)]
    index: u32,
    /// Additional root class names.
    #[props(default)]
    class: String,
    /// Caller styles applied after component geometry.
    #[props(default)]
    style: String,
    /// Caller-owned native root ref callback.
    #[props(default)]
    onmounted: Option<EventHandler<MountedEvent>>,
    /// Native ARIA/data/DOM attributes and additional event builders.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    let width = width.css_value().ok_or(SkeletonError::InvalidWidth)?;
    let height = height.css_value().ok_or(SkeletonError::InvalidHeight)?;
    let mut class = format!("astryx-skeleton {class}");
    for (axis, value) in [("width", &width), ("height", &height)] {
        if let Some(keyword) = dimension_keyword(value) {
            class.push_str(&format!(" astryx-skeleton--{axis}-{}", keyword.as_str()));
        }
    }
    let delay = 1_000 + 100 * u64::from(index);
    let style = format!(
        "--_skeleton-width:{width};--_skeleton-height:{height};--_skeleton-radius:{};--_skeleton-delay:{delay}ms;{style}",
        radius.css_value(),
    );
    let is_hidden_by_default = !attributes.iter().any(|attr| attr.name == "aria-hidden");
    let mut attributes = attributes;
    if let Some(handler) = onmounted {
        attributes.push(mounted_listener(handler));
    }

    rsx! {
        div {
            class,
            style,
            "aria-hidden": is_hidden_by_default.then_some("true"),
            ..attributes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CssWideKeyword, SkeletonLength, SkeletonRadius, dimension_keyword, is_self_contained_css,
    };

    #[test]
    fn dimensions_keep_open_css_but_cannot_change_other_declarations() {
        for value in [
            "0",
            "100%",
            "auto",
            "calc(100% - 2rem)",
            "var(--width, 12px)",
            "attr(data-size type(<length>), 20px)",
            "var(--w, 'quoted;{} value')",
            "var(--w, 12px; ignored-fallback)",
            "var(--w, {ignored-fallback})",
            "/* ;{} */ min(20rem, 100%)",
            "var(--\\31 width)",
        ] {
            assert!(is_self_contained_css(value), "{value}");
        }
        for value in [
            "",
            "  ",
            "20px;color:red",
            "20px}div{color:red",
            "var(--w",
            "'open",
            "/*open",
            "0\\",
            "0\0",
            "20px)",
            "[)",
            "'newline\n'",
        ] {
            assert!(!is_self_contained_css(value), "{value}");
        }
        for value in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(SkeletonLength::Pixels(value).css_value().is_none());
        }
        assert_eq!(
            SkeletonLength::from(12.5).css_value().as_deref(),
            Some("12.5px")
        );
        assert_eq!(SkeletonLength::from(0).css_value().as_deref(), Some("0px"));
    }

    #[test]
    fn radius_levels_preserve_the_original_zero_and_shared_container_distinctions() {
        assert_eq!(SkeletonRadius::default(), SkeletonRadius::Three);
        assert_eq!(SkeletonRadius::None.css_value(), "0px");
        assert_eq!(SkeletonRadius::Zero.css_value(), "var(--radius-none)");
        assert_eq!(SkeletonRadius::One.css_value(), "var(--radius-inner)");
        assert_eq!(SkeletonRadius::Two.css_value(), "var(--radius-element)");
        assert_eq!(
            SkeletonRadius::Three.css_value(),
            SkeletonRadius::Four.css_value()
        );
        assert_eq!(SkeletonRadius::Rounded.css_value(), "var(--radius-full)");
    }

    #[test]
    fn css_wide_keywords_are_not_inherited_as_private_variables() {
        assert_eq!(
            dimension_keyword(" /*comment*/ INHERIT "),
            Some(CssWideKeyword::Inherit)
        );
        assert_eq!(dimension_keyword("initial"), Some(CssWideKeyword::Initial));
        assert_eq!(dimension_keyword("unset"), Some(CssWideKeyword::Unset));
        assert_eq!(dimension_keyword("revert"), Some(CssWideKeyword::Revert));
        assert_eq!(
            dimension_keyword("revert-layer"),
            Some(CssWideKeyword::RevertLayer)
        );
        assert_eq!(dimension_keyword("in/**/herit"), None);
        assert_eq!(
            dimension_keyword("\\69 nherit"),
            Some(CssWideKeyword::Inherit)
        );
        assert_eq!(
            dimension_keyword("\\69nherit"),
            Some(CssWideKeyword::Inherit)
        );
        assert_eq!(
            dimension_keyword("\\000069nherit"),
            Some(CssWideKeyword::Inherit)
        );
        assert_eq!(
            dimension_keyword("\\69\r\nnherit"),
            Some(CssWideKeyword::Inherit)
        );
        assert_eq!(
            dimension_keyword("in\\herit"),
            Some(CssWideKeyword::Inherit)
        );
        assert_eq!(dimension_keyword("var(--inherit)"), None);
    }
}
