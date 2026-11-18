// Adapted from Meta Astryx, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

/// Shared typography contracts owned by the upstream theme module.
pub mod types;

pub use types::{
    InvalidTextName, TextColor, TextDisplay, TextJustify, TextSize, TextType, TextWeight, TextWrap,
    WordBreak,
};
