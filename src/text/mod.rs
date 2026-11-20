// Adapted from Meta Astryx Text, baseline 476a3153417dbf75c565f7df05b0784e94ddd59e.
// See THIRD_PARTY_NOTICES.md.

#[path = "text.rs"]
mod component;
pub(crate) mod text_clamp_context;
mod use_truncation;

use crate::theme::TextColor;

pub use component::{Text, TextTag};
pub use use_truncation::{Truncation, use_truncation};

/// Shared built-in color baseline; custom colors keep their theme data name.
pub(crate) fn resolve_style_color(color: &TextColor) -> &str {
    color.baseline()
}
