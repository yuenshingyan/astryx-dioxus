// Adapted from Astryx utils/safeUrl.ts. See THIRD_PARTY_NOTICES.md.

/// Normalize a navigation URL for inspection, rejecting only the shared blocked
/// schemes. Resource URLs require a separate policy. Accepted router inputs
/// need not be replaced with this normalized value.
pub(crate) fn sanitize_url(url: &str) -> Option<String> {
    let normalized: String = url
        .chars()
        .filter(|c| !matches!(*c as u32, 0..=31 | 127))
        .collect();
    let normalized =
        normalized.trim_matches(|c: char| (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}');
    let lower = normalized.to_lowercase();
    let comparable = match lower.split_once(':') {
        Some((scheme, body))
            if scheme.starts_with(|c: char| c.is_ascii_alphabetic())
                && scheme
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-')) =>
        {
            format!("{scheme}:{}", body.trim_start_matches(' '))
        }
        _ => lower,
    };
    if comparable.starts_with("javascript:")
        || comparable.starts_with("vbscript:")
        || comparable.starts_with("data:text/html")
    {
        return None;
    }
    Some(normalized.to_string())
}

/// Inspect without changing the destination supplied to a native/router sink.
pub(crate) fn is_safe_url(url: &str) -> bool {
    sanitize_url(url).is_some()
}

#[cfg(test)]
mod tests {
    use super::{is_safe_url, sanitize_url};

    #[test]
    fn navigation_blocks_browser_ignored_controls_and_html_data_schemes() {
        for value in [
            "javascript:run()",
            " JaVa\nScRiPt:run() ",
            "vbscript:run()",
            "data: text/html,hello",
            "data:\tTEXT/HTML;charset=utf8,hello",
            "\u{feff}javascript:run()",
        ] {
            assert!(!is_safe_url(value), "{value:?}");
        }
        for value in [
            "",
            "/docs",
            "#part",
            "../next",
            "https://example.test",
            "mailto:hi@example.test",
            "tel:+123",
            "app:route",
            "data:image/png;base64,AA==",
            "data:text/plain,hello",
            "\u{85}javascript:relative-not-a-scheme",
        ] {
            assert!(is_safe_url(value), "{value:?}");
        }
        assert_eq!(sanitize_url(" \n/docs\u{7f} ").as_deref(), Some("/docs"));
    }
}
