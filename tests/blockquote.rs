use astryx_dioxus::{Blockquote, BlockquoteCite, Card, SpacingStep};
use dioxus::events::{onclick, onmounted};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn native_quote_has_no_invented_role_state_or_citation() {
    let html = render(|| rsx! { Blockquote { "A quotation." } });
    assert!(html.starts_with("<blockquote"), "{html}");
    assert!(html.contains("class=\"astryx-blockquote "), "{html}");
    assert!(html.ends_with("A quotation.</blockquote>"), "{html}");
    for absent in ["<cite", "<footer", "role=", "tabindex=", "aria-", "<script"] {
        assert!(!html.contains(absent), "{absent}: {html}");
    }
}

#[test]
fn attribution_follows_rich_children_without_a_landmark() {
    let html = render(|| {
        rsx! {
            Blockquote { cite: rsx! { a { href: "#source", "Author & source" } },
                p { "First paragraph" }
                p { "Second paragraph" }
            }
        }
    });
    assert!(html.contains("<cite class=\"astryx-blockquote__cite\"><a href=\"#source\">Author &#38; source</a></cite>"), "{html}");
    assert!(html.find("Second paragraph").unwrap() < html.find("<cite").unwrap());
    assert!(!html.contains("<footer"));
    assert!(!html.contains("contentinfo"));
}

#[test]
fn shallow_renderability_preserves_whitespace_zero_and_empty_rich_nodes() {
    let html = render(|| {
        rsx! {
            Blockquote { id: "absent", "Quote" }
            Blockquote { id: "false", cite: false, "Quote" }
            Blockquote { id: "true", cite: true, "Quote" }
            Blockquote { id: "empty", cite: "", "Quote" }
            Blockquote { id: "explicit", cite: BlockquoteCite::Empty, "Quote" }
            Blockquote { id: "space", cite: " ", "Quote" }
            Blockquote { id: "zero", cite: 0, "Quote" }
            Blockquote { id: "rich-empty", cite: rsx! {}, "Quote" }
            Blockquote { id: "rich-null", cite: BlockquoteCite::Rich(VNode::empty()), "Quote" }
        }
    });
    assert_eq!(html.matches("<blockquote").count(), 9);
    assert_eq!(html.matches("<cite ").count(), 4, "{html}");
    assert!(html.contains("> </cite>"));
    assert!(html.contains(">0</cite>"));
}

#[test]
fn scalar_attribution_is_escaped_and_numeric_presentation_is_preserved() {
    let html = render(|| {
        rsx! {
            Blockquote { cite: "<script>not markup</script>", "<quote>" }
            Blockquote { cite: -0.0, "A" }
            Blockquote { cite: 1e-7, "B" }
            Blockquote { cite: 1e21, "C" }
            Blockquote { cite: f64::NAN, "D" }
            Blockquote { cite: f64::INFINITY, "E" }
            Blockquote { cite: f64::NEG_INFINITY, "F" }
            Blockquote { cite: u128::MAX, "G" }
        }
    });
    for expected in [
        "&#60;script&#62;not markup&#60;/script&#62;",
        "&#60;quote&#62;",
        ">0</cite>",
        ">1e-7</cite>",
        ">1e+21</cite>",
        ">NaN</cite>",
        ">Infinity</cite>",
        ">-Infinity</cite>",
        ">340282366920938463463374607431768211455</cite>",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(!html.contains("<script>"));
}

#[test]
fn attributes_style_events_and_mount_bridge_compile_on_the_native_root() {
    let html = render(|| {
        rsx! {
            Blockquote {
                id: "native", dir: "rtl", lang: "ar", tabindex: 0,
                "aria-label": "Important quote", "data-proof": "forwarded",
                class: "caller", style: "margin-block:7px;color:rebeccapurple",
                onmounted: move |_| {},
                attributes: vec![onclick(move |_| {}), onmounted(move |_| {})],
                "Quoted text"
            }
        }
    });
    for expected in [
        "id=\"native\"",
        "dir=\"rtl\"",
        "lang=\"ar\"",
        "tabindex=0",
        "aria-label=\"Important quote\"",
        "data-proof=\"forwarded\"",
        "astryx-blockquote caller",
        "margin-block:7px;color:rebeccapurple",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert_eq!(html.matches("<blockquote").count(), 1);
    assert!(!html.contains("onclick="));
}

#[test]
fn testimonial_reuses_existing_public_card_without_layout_or_typography_dependencies() {
    let html = render(|| {
        rsx! {
            Card { padding: SpacingStep::Four,
                Blockquote { cite: "Sarah K., Lead Engineer", "Shipping UI has never been this fast." }
            }
        }
    });
    assert!(html.contains("astryx-card "));
    assert!(html.contains("astryx-blockquote "));
    assert!(html.contains("Sarah K., Lead Engineer</cite>"));
    assert!(!html.contains("<footer"));
}
