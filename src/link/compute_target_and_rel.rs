// Adapted from Astryx Link/computeTargetAndRel.ts. See THIRD_PARTY_NOTICES.md.

pub(crate) fn compute_target_and_rel(
    target: Option<String>,
    rel: Option<String>,
) -> (Option<String>, Option<String>) {
    if target.as_deref() != Some("_blank") {
        return (target, rel);
    }
    let mut tokens: Vec<_> = rel
        .as_deref()
        .unwrap_or_default()
        .split(|c: char| (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}')
        .filter(|token| !token.is_empty())
        .collect();
    for token in ["noopener", "noreferrer"] {
        if !tokens.contains(&token) {
            tokens.push(token);
        }
    }
    (target, Some(tokens.join(" ")))
}

#[cfg(test)]
mod tests {
    use super::compute_target_and_rel;

    #[test]
    fn blank_targets_preserve_and_extend_tokens_without_duplication() {
        assert_eq!(
            compute_target_and_rel(Some("_blank".into()), Some("help noopener".into())),
            (
                Some("_blank".into()),
                Some("help noopener noreferrer".into())
            )
        );
        assert_eq!(
            compute_target_and_rel(Some("_self".into()), Some("help".into())),
            (Some("_self".into()), Some("help".into()))
        );
    }
}
