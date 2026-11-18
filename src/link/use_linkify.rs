// Adapted from Astryx Link/useLinkify.tsx. See THIRD_PARTY_NOTICES.md.

use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::ops::Range;
use std::rc::Rc;

use dioxus::prelude::*;

use crate::link::Link;

/// A match supplied by an application scanner. All offsets are UTF-8 byte offsets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkifyMatch {
    /// Nonempty range of the complete match.
    pub range: Range<usize>,
    /// Capture groups, excluding group zero; `None` represents an unmatched group.
    pub captures: Vec<Option<Range<usize>>>,
}

/// Validated strings passed to destination and label callbacks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkifyCaptures(Vec<Option<String>>);

impl LinkifyCaptures {
    /// Read a capture, with zero denoting the entire match.
    pub fn get(&self, index: usize) -> Option<&str> {
        self.0.get(index).and_then(Option::as_deref)
    }
}

type Matcher = Rc<dyn Fn(&str) -> Vec<LinkifyMatch>>;
type Builder = Rc<dyn Fn(&LinkifyCaptures) -> String>;

/// An application-owned matcher; no regex engine or dependency is imposed.
#[derive(Clone)]
pub struct LinkifyPattern {
    matcher: Matcher,
    href: Builder,
    label: Option<Builder>,
    /// Request external-link behavior from `Link`. Defaults to false.
    pub is_external: bool,
}

impl LinkifyPattern {
    /// Create a pattern from a scanner and a destination builder.
    ///
    /// The scanner runs once per scan and must terminate. Its output is validated
    /// before any builders run. Retain/clone patterns across renders for memoization.
    pub fn new(
        matcher: impl Fn(&str) -> Vec<LinkifyMatch> + 'static,
        href: impl Fn(&LinkifyCaptures) -> String + 'static,
    ) -> Self {
        Self {
            matcher: Rc::new(matcher),
            href: Rc::new(href),
            label: None,
            is_external: false,
        }
    }

    /// Override the displayed text (otherwise the full match is used).
    pub fn with_label(mut self, label: impl Fn(&LinkifyCaptures) -> String + 'static) -> Self {
        self.label = Some(Rc::new(label));
        self
    }
}

impl PartialEq for LinkifyPattern {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.matcher, &other.matcher)
            && Rc::ptr_eq(&self.href, &other.href)
            && match (&self.label, &other.label) {
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
            && self.is_external == other.is_external
    }
}

/// Link detection options. Custom patterns precede builtins at equal starts.
#[derive(Clone, PartialEq)]
pub struct UseLinkifyOptions {
    /// Ordered application matchers.
    pub patterns: Vec<LinkifyPattern>,
    /// Enable HTTP(S) and ASCII email detection (default true).
    pub has_builtins: bool,
}

impl Default for UseLinkifyOptions {
    fn default() -> Self {
        Self {
            patterns: Vec::new(),
            has_builtins: true,
        }
    }
}

/// A recoverable invalid application matcher result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkifyError {
    /// A match was empty, reversed, outside the input, or split a UTF-8 character.
    InvalidMatchRange {
        /// Pattern index in the options.
        pattern: usize,
        /// Match index returned by that pattern.
        match_index: usize,
    },
    /// A capture was reversed, outside its match, or split a UTF-8 character.
    InvalidCaptureRange {
        /// Pattern index in the options.
        pattern: usize,
        /// Match index returned by that pattern.
        match_index: usize,
        /// One-based capture index.
        capture: usize,
    },
}

impl fmt::Display for LinkifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMatchRange {
                pattern,
                match_index,
            } => write!(
                f,
                "linkify pattern {pattern}, match {match_index}: range must be nonempty, in bounds, and on UTF-8 boundaries"
            ),
            Self::InvalidCaptureRange {
                pattern,
                match_index,
                capture,
            } => write!(
                f,
                "linkify pattern {pattern}, match {match_index}, capture {capture}: range must be ordered, inside the match, and on UTF-8 boundaries"
            ),
        }
    }
}

impl Error for LinkifyError {}

/// Interleaved plain text and link data. Destinations are deliberately not sanitized
/// here: rendering must use `Link`, which owns URL safety and provider routing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkifyNode {
    /// Unchanged text, including the single empty node for empty input.
    Text(String),
    /// A detected destination and its display text.
    Link {
        /// Destination passed to `Link`.
        href: String,
        /// Display text, rendered as text rather than markup.
        label: String,
        /// External-link behavior passed to `Link`.
        is_external: bool,
    },
}

struct ResolvedMatch {
    range: Range<usize>,
    node: LinkifyNode,
}

fn is_valid_range(text: &str, range: &Range<usize>) -> bool {
    range.start <= range.end && text.get(range.clone()).is_some()
}

/// Scan without hooks. Earliest start wins; stable ties follow pattern/output order.
/// Invalid custom ranges return an error, never partially linkified output.
pub fn linkify(text: &str, options: &UseLinkifyOptions) -> Result<Vec<LinkifyNode>, LinkifyError> {
    if text.is_empty() {
        return Ok(vec![LinkifyNode::Text(text.to_owned())]);
    }
    let mut matches = Vec::new();
    // Validate all custom results before invoking href/label callbacks.
    let custom: Vec<_> = options.patterns.iter().map(|p| (p.matcher)(text)).collect();
    for (pattern, found) in custom.iter().enumerate() {
        for (match_index, item) in found.iter().enumerate() {
            if item.range.is_empty() || !is_valid_range(text, &item.range) {
                return Err(LinkifyError::InvalidMatchRange {
                    pattern,
                    match_index,
                });
            }
            for (index, capture) in item.captures.iter().enumerate() {
                if let Some(range) = capture
                    && (!is_valid_range(text, range)
                        || range.start < item.range.start
                        || range.end > item.range.end)
                {
                    return Err(LinkifyError::InvalidCaptureRange {
                        pattern,
                        match_index,
                        capture: index + 1,
                    });
                }
            }
        }
    }
    for (p, found) in options.patterns.iter().zip(custom) {
        for item in found {
            let full = text[item.range.clone()].to_owned();
            let captures =
                LinkifyCaptures(
                    std::iter::once(Some(full.clone()))
                        .chain(item.captures.iter().map(|range| {
                            range.as_ref().map(|range| text[range.clone()].to_owned())
                        }))
                        .collect(),
                );
            matches.push(ResolvedMatch {
                range: item.range,
                node: LinkifyNode::Link {
                    href: (p.href)(&captures),
                    label: p.label.as_ref().map_or(full, |label| label(&captures)),
                    is_external: p.is_external,
                },
            });
        }
    }
    if options.has_builtins {
        scan_urls(text, &mut matches);
        scan_emails(text, &mut matches);
    }
    matches.sort_by_key(|item| item.range.start);
    let mut nodes = Vec::new();
    let mut end = 0;
    for item in matches {
        if item.range.start < end {
            continue;
        }
        if item.range.start > end {
            nodes.push(LinkifyNode::Text(text[end..item.range.start].to_owned()));
        }
        end = item.range.end;
        nodes.push(item.node);
    }
    if end < text.len() || nodes.is_empty() {
        nodes.push(LinkifyNode::Text(text[end..].to_owned()));
    }
    Ok(nodes)
}

fn is_url_end(c: char) -> bool {
    // ECMAScript \s, not Rust's Unicode White_Space (which also contains U+0085).
    matches!(c, '\t'..='\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}'
        | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}'
        | '<' | '>' | '\'' | '"' | ')' | ']' | '}' | ',')
}

fn scan_urls(text: &str, matches: &mut Vec<ResolvedMatch>) {
    let mut cursor = 0;
    while cursor < text.len() {
        let suffix = &text[cursor..];
        if suffix.starts_with("http://") || suffix.starts_with("https://") {
            let body = cursor + if suffix.starts_with("https://") { 8 } else { 7 };
            let end = text[body..]
                .char_indices()
                .find(|(_, c)| is_url_end(*c))
                .map_or(text.len(), |(offset, _)| body + offset);
            if end > body {
                let label = text[cursor..end].to_owned();
                matches.push(ResolvedMatch {
                    range: cursor..end,
                    node: LinkifyNode::Link {
                        href: label.clone(),
                        label,
                        is_external: true,
                    },
                });
                cursor = end;
                continue;
            }
        }
        cursor += text[cursor..].chars().next().map_or(1, char::len_utf8);
    }
}

fn is_local(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || b"._%+-".contains(&byte)
}

fn is_domain(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-')
}

fn scan_emails(text: &str, matches: &mut Vec<ResolvedMatch>) {
    let bytes = text.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() {
        if !is_local(bytes[cursor]) {
            cursor += 1;
            continue;
        }
        let start = cursor;
        let has_boundary = start == 0 || !is_local(bytes[start - 1]);
        while cursor < bytes.len() && is_local(bytes[cursor]) {
            cursor += 1;
        }
        if !has_boundary || bytes.get(cursor) != Some(&b'@') {
            continue;
        }
        cursor += 1;
        let domain_start = cursor;
        let mut scan = cursor;
        let mut end = None;
        // Remember the last qualifying dot/suffix, reproducing greedy regex
        // backtracking without repeatedly rescanning a long domain token.
        while scan < bytes.len() && is_domain(bytes[scan]) {
            if bytes[scan] == b'.' && scan > domain_start {
                let letters = scan + 1;
                scan = letters;
                while scan < bytes.len() && bytes[scan].is_ascii_alphabetic() {
                    scan += 1;
                }
                if scan - letters >= 2 {
                    end = Some(scan);
                }
            } else {
                scan += 1;
            }
        }
        if let Some(end) = end {
            let label = text[start..end].to_owned();
            matches.push(ResolvedMatch {
                range: start..end,
                node: LinkifyNode::Link {
                    href: format!("mailto:{label}"),
                    label,
                    is_external: false,
                },
            });
            cursor = end;
        }
    }
}

fn render_nodes(nodes: Vec<LinkifyNode>) -> Element {
    rsx! {
        for (index, node) in nodes.into_iter().enumerate() {
            Fragment { key: "linkify-{index}",
                {match node {
                    LinkifyNode::Text(text) => rsx! { "{text}" },
                    LinkifyNode::Link { href, label, is_external } => rsx! {
                        Link { href: Some(href), is_external_link: is_external, "{label}" }
                    },
                }}
            }
        }
    }
}

type Cached = (String, UseLinkifyOptions, Result<Element, LinkifyError>);

/// Memoized linkification for a component. Cloned options retain callback identity;
/// changing text, callback identity, pattern order, or flags invalidates the cache.
/// All detected links render through the real `Link` and its provider/safety policy.
pub fn use_linkify(
    text: impl Into<String>,
    options: UseLinkifyOptions,
) -> Result<Element, LinkifyError> {
    let text = text.into();
    let cache = use_hook(|| Rc::new(RefCell::new(None::<Cached>)));
    if let Some((previous_text, previous_options, result)) = cache.borrow().as_ref()
        && previous_text == &text
        && previous_options == &options
    {
        return result.clone();
    }
    let result = linkify(&text, &options).map(render_nodes);
    *cache.borrow_mut() = Some((text, options, result.clone()));
    result
}
