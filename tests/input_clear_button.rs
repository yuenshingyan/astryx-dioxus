use dioxus::prelude::*;
use dioxus_ssr::render;

use astryx_dioxus::{
    BaseTypeaheadOption, InternationalizationProvider, MessagesByLocale, TokenizerWithClear,
};

#[test]
fn contextual_name_stays_separate_from_the_generic_tooltip() {
    for (messages, name, tooltip) in [
        (MessagesByLocale::default(), "Clear all", "Clear"),
        (
            [(
                "fr".into(),
                [
                    ("@astryx.tokenizer.clearAll".into(), "Tout effacer".into()),
                    ("@astryx.inputClearButton.tooltip".into(), "Effacer".into()),
                ]
                .into(),
            )]
            .into(),
            "Tout effacer",
            "Effacer",
        ),
    ] {
        let mut dom = VirtualDom::new_with_props(clearable_tokenizer, messages);
        dom.rebuild_in_place();
        let html = render(&dom);
        let (before, _) = html
            .split_once("astryx-input-clear-button")
            .expect("clear target");
        let clear = &html[before.rfind('<').expect("clear tag start")..];
        let (tag, children) = clear.split_once('>').expect("clear opening tag");
        assert!(tag.starts_with("<button "), "{tag}");
        assert!(tag.contains("type=\"button\""), "{tag}");
        assert!(tag.contains(&format!("aria-label=\"{name}\"")), "{tag}");
        assert!(!tag.contains("tabindex="), "{tag}");
        let description = tag
            .split_once("aria-describedby=\"")
            .expect("tooltip reference")
            .1
            .split('"')
            .next()
            .expect("tooltip ID");
        let (button, after) = children
            .split_once("</button>")
            .expect("clear button contents");
        let glyph = button
            .split('<')
            .filter_map(|node| node.split('>').next())
            .find(|tag| tag.contains("astryx-input-clear-icon"))
            .expect("clear glyph target");
        assert!(glyph.contains("aria-hidden=\"true\""), "{glyph}");
        let tip = after
            .split('<')
            .find(|node| {
                node.starts_with("div ") && node.contains(&format!("id=\"{description}\""))
            })
            .expect("matching tooltip");
        let (tip_tag, tip_text) = tip.split_once('>').expect("tooltip tag and text");
        assert!(tip_tag.contains("role=\"tooltip\""), "{tip_tag}");
        assert_eq!(tip_text, tooltip);
    }
}

fn clearable_tokenizer(messages: MessagesByLocale) -> Element {
    rsx! {
        InternationalizationProvider { locale: "fr", messages,
            TokenizerWithClear { label: "Projets", options: vec![], value: vec![BaseTypeaheadOption {
                id: "design".into(), label: "Design".into(), description: None,
            }], on_change: |_| {} }
        }
    }
}
