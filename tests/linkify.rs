use std::cell::Cell;
use std::rc::Rc;

use astryx_dioxus::{
    LinkProvider, LinkRenderProps, LinkifyError, LinkifyMatch, LinkifyNode, LinkifyPattern,
    UseLinkifyOptions, linkify, use_linkify,
};
use dioxus::core::consume_context_from_scope;
use dioxus::prelude::*;

fn links(text: &str) -> Vec<(String, String, bool)> {
    linkify(text, &UseLinkifyOptions::default())
        .unwrap()
        .into_iter()
        .filter_map(|node| match node {
            LinkifyNode::Link {
                href,
                label,
                is_external,
            } => Some((href, label, is_external)),
            LinkifyNode::Text(_) => None,
        })
        .collect()
}

fn issue_pattern() -> LinkifyPattern {
    LinkifyPattern::new(
        |text| {
            let bytes = text.as_bytes();
            let mut found = Vec::new();
            let mut cursor = 0;
            while cursor < bytes.len() {
                if bytes[cursor] != b'#' {
                    cursor += 1;
                    continue;
                }
                let start = cursor;
                cursor += 1;
                let digits = cursor;
                while cursor < bytes.len() && bytes[cursor].is_ascii_digit() {
                    cursor += 1;
                }
                if cursor > digits {
                    found.push(LinkifyMatch {
                        range: start..cursor,
                        captures: vec![Some(digits..cursor)],
                    });
                }
            }
            found
        },
        |m| {
            format!(
                "https://example.com/issues/{}",
                m.get(1).unwrap_or_default()
            )
        },
    )
}

#[test]
fn plain_empty_and_disabled_are_unchanged() {
    for text in ["", "No links here, just a plain note.", "注釈 🦀"] {
        assert_eq!(
            linkify(text, &UseLinkifyOptions::default()).unwrap(),
            vec![LinkifyNode::Text(text.into())]
        );
    }
    let options = UseLinkifyOptions {
        patterns: vec![],
        has_builtins: false,
    };
    let text = "https://example.com hi@example.com";
    assert_eq!(
        linkify(text, &options).unwrap(),
        vec![LinkifyNode::Text(text.into())]
    );
}

#[test]
fn urls_follow_source_boundaries_not_url_punctuation_heuristics() {
    assert_eq!(
        links("https://example.com")[0],
        (
            "https://example.com".into(),
            "https://example.com".into(),
            true
        )
    );
    for delimiter in [
        " ", "\n", "<", ">", "'", "\"", ")", "]", "}", ",", "\u{feff}", "\u{2028}",
    ] {
        let text = format!("前https://a.test{delimiter}後");
        assert_eq!(links(&text)[0].1, "https://a.test", "{delimiter:?}");
    }
    for suffix in [".", ";", "!", "?", "(", "[", "{", "é", "\u{85}"] {
        assert_eq!(
            links(&format!("https://a.test{suffix}"))[0].1,
            format!("https://a.test{suffix}")
        );
    }
    assert!(links("HTTP://a.test ftp://a.test http:// https://").is_empty());
    assert_eq!(links("xhttp://a.test https://b.test").len(), 2);
}

#[test]
fn emails_reproduce_ascii_greedy_and_lookbehind_rules() {
    for (text, expected) in [
        (
            "Mail first.last+tag@sub.example.co.uk now",
            "first.last+tag@sub.example.co.uk",
        ),
        ("hi@example.com.", "hi@example.com"),
        ("hi@example.com123", "hi@example.com"),
        ("hi@example.com.x", "hi@example.com"),
        ("a@b..co", "a@b..co"),
        ("éhi@example.com界", "hi@example.com"),
        ("._%+-hi@example.com", "._%+-hi@example.com"),
    ] {
        assert_eq!(
            links(text),
            vec![(format!("mailto:{expected}"), expected.into(), false)],
            "{text}"
        );
    }
    for text in ["a@b.c", "a@.com", "a@例.com", "a@b.12", "@b.com"] {
        assert!(links(text).is_empty(), "{text}");
    }
    assert_eq!(links("https://a.test/hi@example.com").len(), 1);
    assert_eq!(links("a@b.c a@b.co").len(), 1);
    assert_eq!(links("a@b.com123@d.com").len(), 1);
}

#[test]
fn long_builtin_tokens_are_scanned_without_recursive_backtracking() {
    let text = "a".repeat(100_000);
    assert!(links(&text).is_empty());
    assert!(links(&format!("{text}@{}.x", "b.".repeat(50_000))).is_empty());
    assert_eq!(links(&format!("https://{text}")).len(), 1);
}

#[test]
fn comment_template_and_unicode_surroundings_are_preserved() {
    let options = UseLinkifyOptions {
        patterns: vec![issue_pattern()],
        ..Default::default()
    };
    let nodes = linkify(
        "前🦀 #123 後 https://example.com/spec or support@example.com.",
        &options,
    )
    .unwrap();
    assert_eq!(nodes[0], LinkifyNode::Text("前🦀 ".into()));
    assert_eq!(
        nodes[1],
        LinkifyNode::Link {
            href: "https://example.com/issues/123".into(),
            label: "#123".into(),
            is_external: false,
        }
    );
    assert_eq!(nodes.last(), Some(&LinkifyNode::Text(".".into())));
    assert_eq!(
        linkify("Fixed by #123, tracking the regression in #456.", &options)
            .unwrap()
            .len(),
        5
    );
    assert_eq!(
        linkify("No links here, just a plain note.", &options).unwrap(),
        vec![LinkifyNode::Text(
            "No links here, just a plain note.".into()
        )]
    );
}

fn fixed(range: std::ops::Range<usize>, href: &'static str) -> LinkifyPattern {
    LinkifyPattern::new(
        move |_| {
            vec![LinkifyMatch {
                range: range.clone(),
                captures: vec![],
            }]
        },
        move |_| href.into(),
    )
}

#[test]
fn stable_start_then_pattern_order_and_overlap_discard() {
    let text = "https://example.com/#123";
    let options = UseLinkifyOptions {
        patterns: vec![
            fixed(0..text.len(), "/custom"),
            fixed(0..5, "/tie"),
            issue_pattern(),
        ],
        ..Default::default()
    };
    assert_eq!(
        linkify(text, &options).unwrap(),
        vec![LinkifyNode::Link {
            href: "/custom".into(),
            label: text.into(),
            is_external: false,
        }]
    );
    let options = UseLinkifyOptions {
        patterns: vec![
            fixed(3..6, "/later"),
            fixed(0..4, "/early"),
            fixed(4..6, "/adjacent"),
        ],
        has_builtins: false,
    };
    let nodes = linkify("abcdef", &options).unwrap();
    assert_eq!(nodes.len(), 2);
    assert!(matches!(&nodes[0], LinkifyNode::Link { href, .. } if href == "/early"));
    assert!(matches!(&nodes[1], LinkifyNode::Link { href, .. } if href == "/adjacent"));
}

#[test]
fn labels_captures_and_external_flag() {
    let mut pattern = issue_pattern().with_label(|m| format!("Issue {}", m.get(1).unwrap()));
    pattern.is_external = true;
    let options = UseLinkifyOptions {
        patterns: vec![pattern],
        has_builtins: false,
    };
    assert_eq!(
        linkify("#5678", &options).unwrap(),
        vec![LinkifyNode::Link {
            href: "https://example.com/issues/5678".into(),
            label: "Issue 5678".into(),
            is_external: true,
        }]
    );
}

#[test]
fn invalid_ranges_return_errors_without_calling_builders() {
    let reversed = std::ops::Range { start: 3, end: 2 };
    for range in [0..0, reversed.clone(), 0..99, 1..2, 0..1] {
        let pattern = LinkifyPattern::new(
            move |_| {
                vec![LinkifyMatch {
                    range: range.clone(),
                    captures: vec![],
                }]
            },
            |_| panic!("invalid match reached builder"),
        );
        assert!(matches!(
            linkify(
                "éx",
                &UseLinkifyOptions {
                    patterns: vec![pattern],
                    has_builtins: false
                }
            ),
            Err(LinkifyError::InvalidMatchRange { .. })
        ));
    }
    for range in [1..2, 0..4, reversed] {
        let pattern = LinkifyPattern::new(
            move |_| {
                vec![LinkifyMatch {
                    range: 0..3,
                    captures: vec![Some(range.clone())],
                }]
            },
            |_| panic!("invalid capture reached builder"),
        );
        assert!(matches!(
            linkify(
                "éx",
                &UseLinkifyOptions {
                    patterns: vec![pattern],
                    has_builtins: false
                }
            ),
            Err(LinkifyError::InvalidCaptureRange { capture: 1, .. })
        ));
    }
}

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn rendered_nodes_use_links_and_escape_labels() {
    let html = render(|| {
        let options = use_hook(|| UseLinkifyOptions {
            patterns: vec![issue_pattern().with_label(|_| "<issue>".into())],
            ..Default::default()
        });
        let nodes = use_linkify(
            "See #123 or hi@example.com and https://example.com",
            options,
        )
        .unwrap();
        rsx! { p { {nodes} } }
    });
    assert!(
        html.contains("href=\"https://example.com/issues/123\""),
        "{html}"
    );
    assert!(html.contains("href=\"mailto:hi@example.com\""), "{html}");
    assert!(html.contains("&#60;issue&#62;"), "{html}");
    assert!(!html.contains("<issue>"), "{html}");
    assert!(html.contains("target=\"_blank\""), "{html}");
}

#[test]
fn unsafe_custom_destinations_are_rejected_by_real_link() {
    for href in [
        "javascript:alert(1)",
        "data:text/html,unsafe",
        "vbscript:msgbox(1)",
    ] {
        let mut dom = VirtualDom::new_with_props(unsafe_destination, href.to_owned());
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(!html.contains("href=\"javascript:"), "{html}");
        assert!(!html.contains("href=\"data:"), "{html}");
        assert!(!html.contains("href=\"vbscript:"), "{html}");
        assert!(html.contains("unsafe"), "{html}");
    }
}

fn unsafe_destination(href: String) -> Element {
    let options = use_hook(move || UseLinkifyOptions {
        patterns: vec![LinkifyPattern::new(
            |_| {
                vec![LinkifyMatch {
                    range: 0..6,
                    captures: vec![],
                }]
            },
            move |_| href.clone(),
        )],
        has_builtins: false,
    });
    let nodes = use_linkify("unsafe", options).unwrap();
    rsx! { LinkProvider { component: reject_unsafe_router, {nodes} } }
}

fn reject_unsafe_router(_: LinkRenderProps) -> Element {
    panic!("unsafe linkify destinations must not reach application routing")
}

#[derive(Clone)]
struct MemoState {
    calls: Rc<Cell<usize>>,
    revision: Signal<usize>,
}

fn memo_example() -> Element {
    let revision = use_signal(|| 0usize);
    let calls = use_hook(|| Rc::new(Cell::new(0)));
    use_context_provider(|| MemoState {
        calls: calls.clone(),
        revision,
    });
    let mut options = use_hook(move || UseLinkifyOptions {
        patterns: vec![LinkifyPattern::new(
            move |_| {
                calls.set(calls.get() + 1);
                vec![]
            },
            |_| String::new(),
        )],
        has_builtins: false,
    });
    options.has_builtins = revision() >= 3;
    let text = if revision() < 2 {
        "unchanged"
    } else {
        "changed"
    };
    use_linkify(text, options).unwrap()
}

#[test]
fn hook_memoizes_unchanged_inputs_and_invalidates_changed_text() {
    let mut dom = VirtualDom::new(memo_example);
    dom.rebuild_in_place();
    let mut state = dom.in_runtime(|| {
        consume_context_from_scope::<MemoState>(ScopeId::APP).expect("memo test state")
    });
    assert_eq!(state.calls.get(), 1);
    dom.in_runtime(|| state.revision.set(1));
    dom.render_immediate_to_vec();
    assert_eq!(state.calls.get(), 1);
    dom.in_runtime(|| state.revision.set(2));
    dom.render_immediate_to_vec();
    assert_eq!(state.calls.get(), 2);
    dom.in_runtime(|| state.revision.set(3));
    dom.render_immediate_to_vec();
    assert_eq!(state.calls.get(), 3);
}
