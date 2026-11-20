// Copyright (c) Meta Platforms, Inc. and affiliates.
// Adapted from Stack/stack.stylex.ts at Astryx 476a3153417dbf75c565f7df05b0784e94ddd59e.

/// Flex flow direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StackDirection {
    /// `horizontal`.
    Horizontal,
    /// `vertical`.
    #[default]
    Vertical,
}
impl StackDirection {
    /// Original reflected spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Flex wrapping mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackWrap {
    /// `nowrap`.
    Nowrap,
    /// `wrap`.
    Wrap,
    /// `wrap-reverse`.
    WrapReverse,
}
impl StackWrap {
    /// Original reflected spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Nowrap => "nowrap",
            Self::Wrap => "wrap",
            Self::WrapReverse => "wrap-reverse",
        }
    }
}

/// Main-axis alignment accepted by the justify alias.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackMainAlignment {
    /// `start`.
    Start,
    /// `center`.
    Center,
    /// `end`.
    End,
    /// `between`.
    Between,
    /// `around`.
    Around,
    /// `evenly`.
    Evenly,
}
impl StackMainAlignment {
    /// Original reflected spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Between => "between",
            Self::Around => "around",
            Self::Evenly => "evenly",
        }
    }
}

/// Cross-axis alignment accepted by the align alias.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackCrossAlignment {
    /// `start`.
    Start,
    /// `center`.
    Center,
    /// `end`.
    End,
    /// `stretch`.
    Stretch,
}
impl StackCrossAlignment {
    /// Original reflected spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Stretch => "stretch",
        }
    }
}

/// Physical alignment union. Values inapplicable to the resolved axis are ignored.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackAlignment {
    /// `start`.
    Start,
    /// `center`.
    Center,
    /// `end`.
    End,
    /// `between`.
    Between,
    /// `around`.
    Around,
    /// `evenly`.
    Evenly,
    /// `stretch`.
    Stretch,
}
impl StackAlignment {
    /// Original reflected spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Between => "between",
            Self::Around => "around",
            Self::Evenly => "evenly",
            Self::Stretch => "stretch",
        }
    }
}

impl StackAlignment {
    pub(super) const fn main_css(self) -> Option<&'static str> {
        match self {
            Self::Stretch => None,
            value => Some(value.as_str()),
        }
    }
    pub(super) const fn cross_css(self) -> Option<&'static str> {
        match self {
            Self::Between | Self::Around | Self::Evenly => None,
            value => Some(value.as_str()),
        }
    }
}
impl From<StackMainAlignment> for StackAlignment {
    fn from(value: StackMainAlignment) -> Self {
        match value {
            StackMainAlignment::Start => Self::Start,
            StackMainAlignment::Center => Self::Center,
            StackMainAlignment::End => Self::End,
            StackMainAlignment::Between => Self::Between,
            StackMainAlignment::Around => Self::Around,
            StackMainAlignment::Evenly => Self::Evenly,
        }
    }
}
impl From<StackCrossAlignment> for StackAlignment {
    fn from(value: StackCrossAlignment) -> Self {
        match value {
            StackCrossAlignment::Start => Self::Start,
            StackCrossAlignment::Center => Self::Center,
            StackCrossAlignment::End => Self::End,
            StackCrossAlignment::Stretch => Self::Stretch,
        }
    }
}
