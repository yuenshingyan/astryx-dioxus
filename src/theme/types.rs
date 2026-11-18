// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// A semantic theme name was empty or not an ASCII identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidTextName;

impl Display for InvalidTextName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("text theme names must start with an ASCII letter or underscore and contain only ASCII letters, digits, underscores, or hyphens")
    }
}

impl Error for InvalidTextName {}

fn validate_name(value: &str) -> Result<(), InvalidTextName> {
    let mut chars = value.chars();
    if !chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        || !chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(InvalidTextName);
    }
    Ok(())
}

macro_rules! theme_name {
    ($name:ident, $description:literal, $default:ident, {$($constant:ident => $value:literal),+ $(,)?}) => {
        #[doc = $description]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $name(Cow<'static, str>);

        impl $name {
            $(#[doc = concat!("Built-in `", $value, "` theme value.")]
            pub const $constant: Self = Self(Cow::Borrowed($value));)+

            /// Validates a built-in or consumer-defined theme name.
            pub fn new(value: impl Into<String>) -> Result<Self, InvalidTextName> {
                let value = value.into();
                validate_name(&value)?;
                Ok(Self(Cow::Owned(value)))
            }

            /// Canonical name reflected in the component's theme data attribute.
            pub fn as_str(&self) -> &str { &self.0 }
        }

        impl Default for $name {
            fn default() -> Self { Self::$default }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str(self.as_str()) }
        }

        impl FromStr for $name {
            type Err = InvalidTextName;
            fn from_str(value: &str) -> Result<Self, Self::Err> { Self::new(value) }
        }

        impl TryFrom<String> for $name {
            type Error = InvalidTextName;
            fn try_from(value: String) -> Result<Self, Self::Error> { Self::new(value) }
        }
    };
}

theme_name!(TextType, "Open semantic text type. Custom names retain body baseline styles until theme CSS overrides them.", BODY, {
    BODY => "body", LARGE => "large", LABEL => "label", SUPPORTING => "supporting",
    CODE => "code", DISPLAY_1 => "display-1", DISPLAY_2 => "display-2", DISPLAY_3 => "display-3",
    INHERIT => "inherit",
});

theme_name!(TextColor, "Open semantic text color. Custom names retain primary baseline styles until theme CSS overrides them.", PRIMARY, {
    PRIMARY => "primary", SECONDARY => "secondary", DISABLED => "disabled",
    PLACEHOLDER => "placeholder", ACCENT => "accent", INHERIT => "inherit",
});

impl TextType {
    /// Built-in typography baseline; custom types use body.
    pub fn baseline(&self) -> &str {
        match self.as_str() {
            "body" | "large" | "label" | "supporting" | "code" | "display-1" | "display-2"
            | "display-3" | "inherit" => self.as_str(),
            _ => "body",
        }
    }

    /// Upstream default color, including custom-type and inherit behavior.
    pub fn default_color(&self) -> TextColor {
        match self.as_str() {
            "supporting" => TextColor::SECONDARY,
            "inherit" => TextColor::INHERIT,
            _ => TextColor::PRIMARY,
        }
    }
}

impl TextColor {
    /// Built-in color baseline; custom colors use primary.
    pub fn baseline(&self) -> &str {
        match self.as_str() {
            "primary" | "secondary" | "disabled" | "placeholder" | "accent" | "inherit" => {
                self.as_str()
            }
            _ => "primary",
        }
    }
}

macro_rules! closed_options {
    ($name:ident, $description:literal, {$($variant:ident => $value:literal),+ $(,)?}) => {
        #[doc = $description]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name {
            $(#[doc = concat!("Upstream `", $value, "` option.")]
            $variant),+
        }
        impl $name {
            /// Upstream CSS/theme spelling.
            pub const fn as_str(self) -> &'static str {
                match self { $(Self::$variant => $value),+ }
            }
        }
    };
}

closed_options!(TextSize, "Explicit size override; preserves the type's leading, family and weight.", {
    FourExtraSmall => "4xs", ThreeExtraSmall => "3xs", TwoExtraSmall => "2xs", ExtraSmall => "xsm",
    Small => "sm", Base => "base", Large => "lg", ExtraLarge => "xl", TwoExtraLarge => "2xl",
    ThreeExtraLarge => "3xl", FourExtraLarge => "4xl",
});
closed_options!(TextWeight, "Closed Text/Heading weight options.", {
    Normal => "normal", Medium => "medium", Semibold => "semibold", Bold => "bold",
});
/// Text display before clamping or capsizing overrides it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextDisplay {
    /// Inline text flow (default).
    #[default]
    Inline,
    /// Block text flow.
    Block,
}

impl TextDisplay {
    /// Upstream CSS spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::Block => "block",
        }
    }
}

/// Logical text alignment, suitable for RTL.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextJustify {
    /// Logical start; leaves the inherited text alignment unchanged.
    #[default]
    Start,
    /// Centered alignment.
    Center,
    /// Logical end alignment.
    End,
}

impl TextJustify {
    /// Upstream CSS spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}
closed_options!(WordBreak, "Word breaking applied only while truncating.", {
    BreakWord => "break-word", BreakAll => "break-all",
});
closed_options!(TextWrap, "Native CSS text-wrap behavior.", {
    Wrap => "wrap", NoWrap => "nowrap", Balance => "balance", Pretty => "pretty",
});
