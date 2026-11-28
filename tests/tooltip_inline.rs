use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{STYLES, Tooltip};

#[test]
fn descriptions_are_linked_and_redundant_help_adds_no_tab_stop() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            Tooltip { text: "Privacy policy", content: "Data is encrypted" }
            Tooltip { text: "Returns", content: "Standard 30-day agreement", has_hover_indication: false }
            Tooltip { text: "Duplicate", content: " Duplicate " }
            Tooltip { text: "Blank help", content: "   " }
        }
    });
    dom.rebuild_in_place();
    let html = render(&dom);
    assert_eq!(html.matches("tabindex=\"0\"").count(), 2, "{html}");
    assert_eq!(html.matches("role=\"tooltip\"").count(), 2, "{html}");
    assert_eq!(html.matches("aria-describedby=").count(), 2, "{html}");
    assert_eq!(html.matches("data-hover-indication=").count(), 1, "{html}");

    for (text, content, has_hover_indication) in [
        ("Privacy policy", "Data is encrypted", true),
        ("Returns", "Standard 30-day agreement", false),
    ] {
        let (before, _) = html.split_once(&format!(">{text}</span>")).unwrap();
        let (_, trigger) = before.rsplit_once("<span ").unwrap();
        assert!(trigger.contains("tabindex=\"0\""), "{trigger}");
        assert_eq!(
            trigger.contains("data-hover-indication=\"true\""),
            has_hover_indication,
            "{trigger}"
        );
        let (_, description) = trigger.split_once("aria-describedby=\"").unwrap();
        let (id, _) = description.split_once('"').unwrap();
        assert_eq!(html.matches(&format!("id=\"{id}\"")).count(), 1, "{html}");
        let (_, surface) = html.split_once(&format!("id=\"{id}\"")).unwrap();
        let (surface, _) = surface.split_once("</div>").unwrap();
        assert!(surface.contains("role=\"tooltip\""), "{surface}");
        assert!(surface.ends_with(&format!(">{content}")), "{surface}");
    }
}

#[test]
fn inline_hover_cue_uses_the_source_decoration_in_the_public_bundle() {
    let (_, rule) = STYLES
        .split_once(".astryx-tooltip-text[data-hover-indication] {")
        .unwrap();
    let (declarations, _) = rule.split_once('}').unwrap();
    for declaration in [
        "text-decoration-line: underline;",
        "text-decoration-style: dashed;",
        "text-decoration-color: var(--color-border-emphasized);",
        "text-underline-offset: 2px;",
    ] {
        assert!(
            declarations.lines().any(|line| line.trim() == declaration),
            "missing {declaration} in {declarations}"
        );
    }
}
