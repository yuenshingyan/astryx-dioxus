// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

/// Icon dimensions at a 16px root: 12, 16, 20 and 24px, expressed in rem.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconSize {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
}

impl IconSize {
    /// Upstream size selector.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExtraSmall => "xsm",
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
        }
    }

    /// Default glyph dimension, before theme overrides.
    pub fn dimension(self) -> &'static str {
        match self {
            Self::ExtraSmall => "0.75rem",
            Self::Small => "1rem",
            Self::Medium => "1.25rem",
            Self::Large => "1.5rem",
        }
    }
}
