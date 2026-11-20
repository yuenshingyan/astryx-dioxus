// Adapted from Meta Astryx Layer/useLayer.tsx at
// 476a3153417dbf75c565f7df05b0784e94ddd59e. See THIRD_PARTY_NOTICES.md.

/// Position relative to an anchor. Start/end follow the anchor's direction.
/// This shared type does not imply a completed public Layer component.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LayerPlacement {
    /// Above the anchor (default for truncation tooltips).
    #[default]
    Above,
    /// Below the anchor.
    Below,
    /// Logical inline start.
    Start,
    /// Logical inline end.
    End,
}

impl LayerPlacement {
    /// Authoritative placement spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Above => "above",
            Self::Below => "below",
            Self::Start => "start",
            Self::End => "end",
        }
    }
}
