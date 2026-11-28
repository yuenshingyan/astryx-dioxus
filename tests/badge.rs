use astryx_dioxus::{Badge, BadgeLabel, BadgeVariant, Icon, IconSize};
use dioxus::events::onclick;
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn default_badge_is_static_text_with_a_native_title_and_isolated_label() {
    let html = render(|| rsx! { Badge { label: "Active" } });
    for value in [
        "<span",
        "astryx-badge",
        "data-variant=\"neutral\"",
        "title=\"Active\"",
        "astryx-badge__label",
        "dir=\"ltr\"",
        ">Active</span>",
    ] {
        assert!(html.contains(value), "{html}");
    }
    for value in [
        "role=",
        "tabindex=",
        "aria-live=",
        "aria-hidden=",
        "data-astryx-feedback-id",
        "astryx-tooltip",
        "<script",
    ] {
        assert!(!html.contains(value), "{html}");
    }
    assert_eq!(BadgeVariant::default(), BadgeVariant::NEUTRAL);
}

#[derive(Clone, PartialEq, Props)]
struct CaseProps {
    variant: BadgeVariant,
    label: BadgeLabel,
}
fn case(props: CaseProps) -> Element {
    rsx! { Badge { label: props.label, variant: props.variant } }
}

#[test]
fn all_14_variants_accept_text_number_rich_and_empty_without_inventing_state() {
    for variant in [
        BadgeVariant::NEUTRAL,
        BadgeVariant::INFO,
        BadgeVariant::SUCCESS,
        BadgeVariant::WARNING,
        BadgeVariant::ERROR,
        BadgeVariant::BLUE,
        BadgeVariant::CYAN,
        BadgeVariant::GREEN,
        BadgeVariant::ORANGE,
        BadgeVariant::PINK,
        BadgeVariant::PURPLE,
        BadgeVariant::RED,
        BadgeVariant::TEAL,
        BadgeVariant::YELLOW,
    ] {
        for label in [
            BadgeLabel::Text("Status".into()),
            BadgeLabel::Number(0.0),
            BadgeLabel::Rich(rsx! { b { "Rich" } }),
            BadgeLabel::Empty,
        ] {
            let mut dom = VirtualDom::new_with_props(
                case,
                CaseProps {
                    variant: variant.clone(),
                    label: label.clone(),
                },
            );
            dom.rebuild_in_place();
            let html = dioxus_ssr::render(&dom);
            assert_eq!(
                html.matches(&format!("data-variant=\"{}\"", variant.as_str()))
                    .count(),
                1,
                "{html}"
            );
            assert_eq!(html.matches("astryx-badge__label").count(), 1, "{html}");
            assert!(!html.contains("role="), "{html}");
            assert!(!html.contains("aria-live="), "{html}");
            match label {
                BadgeLabel::Text(_) => {
                    assert!(html.contains("title=\"Status\""), "{html}");
                    assert!(html.contains("astryx-badge--labelled"), "{html}");
                }
                BadgeLabel::Number(_) => {
                    assert!(html.contains("title=\"0\""), "{html}");
                    assert!(html.contains(">0</span>"), "{html}");
                }
                BadgeLabel::Rich(_) => {
                    assert!(!html.contains("title="), "{html}");
                    assert!(!html.contains("dir="), "{html}");
                    assert!(html.contains("<b>Rich</b>"), "{html}");
                }
                BadgeLabel::Empty => {
                    assert!(!html.contains("title="), "{html}");
                    assert!(!html.contains("astryx-badge--labelled"), "{html}");
                }
            }
        }
    }
}

#[test]
fn scalar_title_and_empty_slot_rules_preserve_full_content_without_inspecting_rich_nodes() {
    let html = render(|| {
        rsx! {
            Badge { label: "", id: "empty" }
            Badge { label: false, id: "false" }
            Badge { label: true, id: "true" }
            Badge { label: " ", id: "space" }
            Badge { label: 9_007_199_254_740_993_u64, id: "integer" }
            Badge { label: BadgeLabel::Rich(rsx! { span {} }), id: "rich-empty" }
            Badge { label: "<script>&\"", id: "escaped" }
        }
    });
    assert_eq!(html.matches("title=").count(), 3, "{html}");
    assert!(html.contains("title=\"9007199254740993\""), "{html}");
    assert!(html.contains("title=\" \""), "{html}");
    assert_eq!(html.matches("astryx-badge--labelled").count(), 4, "{html}");
    assert!(!html.contains("<script>"), "{html}");
}

#[test]
fn plain_mixed_direction_uses_any_rtl_character_but_rich_content_stays_caller_owned() {
    let html = render(|| {
        rsx! {
            Badge { label: "+3" }
            Badge { label: "https://" }
            Badge { label: "iPhone جديد" }
            Badge { label: BadgeLabel::Rich(rsx! { bdi { dir: "rtl", "שלום" } }) }
        }
    });
    assert_eq!(html.matches("dir=\"ltr\"").count(), 2, "{html}");
    assert!(html.contains("<bdi dir=\"rtl\">שלום</bdi>"), "{html}");
    assert!(!html.contains("dir=\"auto\""), "{html}");
}

#[test]
fn icon_is_a_direct_leading_node_and_preserves_its_own_size_and_accessibility() {
    let html = render(|| {
        rsx! {
            Badge { label: "Verified", icon: rsx! { Icon { size: IconSize::Large, label: "Named glyph", icon: rsx! { svg {} } } } }
            Badge { label: "Raw icon", icon: rsx! { svg { id: "raw-icon", role: "img", "aria-label": "Raw glyph", width: "7", height: "9" } } }
        }
    });
    assert!(
        html.find("Named glyph").expect("glyph") < html.find("astryx-badge__label").expect("label")
    );
    assert!(html.contains("data-size=\"lg\""), "{html}");
    assert!(html.contains("aria-label=\"Raw glyph\""), "{html}");
    assert!(
        !html.contains("aria-hidden=\"true\" id=\"raw-icon\""),
        "{html}"
    );
    assert_eq!(html.matches("role=\"img\"").count(), 2, "{html}");
}

#[test]
fn native_props_and_caller_title_pass_through_but_theme_state_is_owned() {
    let html = render(|| {
        rsx! {
            Badge { label: "Full label", variant: BadgeVariant::SUCCESS, id: "native", class: "caller", style: "height:30px", title: "Caller title", role: "note", tabindex: "-1", "aria-describedby": "details", lang: "fr", onmounted: |_| {},
                attributes: vec![onclick(|_| {}),Attribute::new("data-variant","error",None,false)] }
        }
    });
    for value in [
        "id=\"native\"",
        "caller",
        "height:30px",
        "title=\"Caller title\"",
        "role=\"note\"",
        "tabindex=\"-1\"",
        "aria-describedby=\"details\"",
        "lang=\"fr\"",
        "data-variant=\"success\"",
    ] {
        assert!(html.contains(value), "{html}");
    }
    assert_eq!(html.matches("title=").count(), 1, "{html}");
    assert!(!html.contains("title=\"Full label\""), "{html}");
    assert!(!html.contains("data-variant=\"error\""), "{html}");
}

#[test]
fn explicit_empty_title_overrides_derived_title_without_removing_label_text() {
    let html = render(|| {
        rsx! {
            div { title: "Ancestor advisory text",
                Badge { label: "Deployment completed with pending review", title: "" }
            }
        }
    });
    let root = html
        .split_once("<span")
        .and_then(|(_, rest)| rest.split_once('>'))
        .map(|(attributes, _)| attributes)
        .expect("Badge root span");
    assert!(root.contains(" title=\"\""), "{html}");
    assert_eq!(html.matches(" title=").count(), 2, "{html}");
    assert!(
        html.contains(">Deployment completed with pending review</span>"),
        "{html}"
    );
}

#[test]
fn custom_variants_are_validated_open_names_without_a_neutral_fallback() {
    for invalid in ["", "two words", "red\n", "<script>", "bad/color"] {
        assert!(BadgeVariant::new(invalid).is_err());
    }
    let custom = BadgeVariant::new("premium-tier").expect("valid name");
    assert_eq!(custom.as_str(), "premium-tier");
    fn custom_badge() -> Element {
        let variant = BadgeVariant::new("premium-tier")?;
        rsx! {Badge{label:"Premium",variant}}
    }
    let html = render(custom_badge);
    assert!(html.contains("data-variant=\"premium-tier\""), "{html}");
    assert!(!html.contains("data-variant=\"neutral\""), "{html}");
}
