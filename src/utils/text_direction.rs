// Copyright (c) Meta Platforms, Inc. and affiliates.
// Adapted from Astryx utils/textDirection.ts; see THIRD_PARTY_NOTICES.md.

use crate::i18n::first_strong_direction::has_strong_right_to_left;

/// Return `dir="ltr"` for plain text with no upstream strong-RTL match.
///
/// A match anywhere returns `None`, preserving the surrounding direction rather
/// than forcing RTL. Callers handle rich labels separately and stringify numeric
/// labels before calling. This helper reads no browser or application state.
pub(crate) fn text_direction(text: &str) -> Option<&'static str> {
    if has_strong_right_to_left(text) {
        return None;
    }
    Some("ltr")
}

#[cfg(test)]
mod tests {
    use super::text_direction;

    #[test]
    fn plain_text_without_matches_is_explicitly_ltr() {
        for text in [
            "",
            "Hello",
            "123",
            "+3",
            "https://example.test",
            "😀",
            "\u{301}",
        ] {
            assert_eq!(text_direction(text), Some("ltr"), "{text:?}");
        }
        assert_eq!(text_direction(&(-3).to_string()), Some("ltr"));
    }

    #[test]
    fn any_match_leaves_direction_inherited_not_forced_rtl() {
        for text in [
            "עברית",
            "مرحبا",
            "English then עברית",
            "Latin \u{591}",
            "\u{660}",
            "\u{200f}",
            "\u{61c}",
            "\u{10800}",
            "\u{1efff}",
        ] {
            assert_eq!(text_direction(text), None, "{text:?}");
        }
    }
}
