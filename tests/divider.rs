use astryx_dioxus::{Divider, DividerLabel, DividerOrientation, DividerVariant};
use dioxus::core::{AttributeValue, ScopeId, consume_context_from_scope};
use dioxus::events::onclick;
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn defaults_are_a_single_unnamed_horizontal_subtle_rule() {
    let html = render(|| rsx! {Divider{}});
    for attribute in [
        "role=\"separator\"",
        "aria-orientation=\"horizontal\"",
        "data-orientation=\"horizontal\"",
        "data-variant=\"subtle\"",
    ] {
        assert!(html.contains(attribute), "{html}");
    }
    assert_eq!(html.matches("astryx-divider__line").count(), 1, "{html}");
    for attribute in [
        "astryx-divider__label",
        "aria-label=",
        "aria-labelledby=",
        "tabindex=",
        "aria-live=",
        "astryx-tooltip",
        "data-astryx-feedback-id",
    ] {
        assert!(!html.contains(attribute), "{html}");
    }
    assert_eq!(DividerVariant::default(), DividerVariant::SUBTLE);
}

#[derive(Clone, PartialEq, Props)]
struct CaseProps {
    orientation: DividerOrientation,
    variant: DividerVariant,
    label: DividerLabel,
    is_full_bleed: bool,
}
fn case(props: CaseProps) -> Element {
    rsx! {Divider{orientation:props.orientation,variant:props.variant,label:props.label,is_full_bleed:props.is_full_bleed}}
}

#[test]
fn all_48_axis_weight_bleed_and_label_cases_preserve_anatomy_and_metadata() {
    for (orientation, axis) in [
        (DividerOrientation::Horizontal, "horizontal"),
        (DividerOrientation::Vertical, "vertical"),
    ] {
        for variant in [DividerVariant::SUBTLE, DividerVariant::STRONG] {
            for is_full_bleed in [false, true] {
                for (label, has_label) in [
                    (DividerLabel::Empty, false),
                    (DividerLabel::Text(String::new()), false),
                    (DividerLabel::Text("or".into()), true),
                    (DividerLabel::Number(0.0), true),
                    (DividerLabel::Rich(rsx! {b{"Rich"}}), true),
                    (DividerLabel::Rich(rsx! {span{}}), true),
                ] {
                    let mut dom = VirtualDom::new_with_props(
                        case,
                        CaseProps {
                            orientation,
                            variant: variant.clone(),
                            label,
                            is_full_bleed,
                        },
                    );
                    dom.rebuild_in_place();
                    let html = dioxus_ssr::render(&dom);
                    assert_eq!(
                        html.matches("astryx-divider__line").count(),
                        if has_label { 2 } else { 1 },
                        "{html}"
                    );
                    assert_eq!(
                        html.matches("astryx-divider__label").count(),
                        usize::from(has_label),
                        "{html}"
                    );
                    assert_eq!(html.matches("role=\"separator\"").count(), 1, "{html}");
                    assert!(
                        html.contains(&format!("aria-orientation=\"{axis}\"")),
                        "{html}"
                    );
                    assert!(
                        html.contains(&format!("data-orientation=\"{axis}\"")),
                        "{html}"
                    );
                    assert!(
                        html.contains(&format!("data-variant=\"{}\"", variant.as_str())),
                        "{html}"
                    );
                    assert_eq!(
                        html.contains("astryx-divider--full-bleed"),
                        is_full_bleed,
                        "{html}"
                    );
                    assert_eq!(html.contains("aria-labelledby="), has_label, "{html}");
                    assert!(!html.contains("aria-live="), "{html}");
                }
            }
        }
    }
}

#[test]
fn scalar_skips_zero_rich_content_and_exact_large_integers_match_the_source_slot_rules() {
    let html = render(|| {
        rsx! {
            Divider { label:false }
            Divider { label:true }
            Divider { label:"" }
            Divider { label:0 }
            Divider { label:9_007_199_254_740_993_u64 }
            Divider { label:" " }
            Divider { label:DividerLabel::Rich(rsx!{strong{"Rich "}em{"label"}}) }
        }
    });
    assert_eq!(html.matches("astryx-divider__label").count(), 4, "{html}");
    assert!(html.contains(">0</div>"), "{html}");
    assert!(html.contains("9007199254740993"), "{html}");
    assert!(
        html.contains("<strong>Rich </strong><em>label</em>"),
        "{html}"
    );
    assert!(!html.contains("title="), "{html}");
}

#[test]
fn explicit_name_precedence_and_unique_visible_label_references_reach_the_root() {
    let html = render(|| {
        rsx! {
            Divider { label:"Visible one" }
            Divider { label:"Visible two" }
            Divider { label:"Visible", "aria-label":"Caller name" }
            Divider { label:"Visible", "aria-label":"Ignored by reference", "aria-labelledby":"external-name" }
        }
    });
    let refs: Vec<_> = html
        .split("aria-labelledby=\"")
        .skip(1)
        .map(|part| part.split('"').next().expect("reference"))
        .collect();
    assert_eq!(refs.len(), 3, "{html}");
    assert_ne!(refs[0], refs[1]);
    assert_eq!(refs[2], "external-name");
    for id in &refs[..2] {
        assert_eq!(html.matches(&format!("id=\"{id}\"")).count(), 1, "{html}");
    }
    assert!(html.contains("aria-label=\"Caller name\""), "{html}");
}

#[test]
fn caller_semantics_are_forwarded_while_the_visual_target_axes_are_protected() {
    let html = render(|| {
        rsx! {
            Divider { role:"note", "aria-orientation":"vertical", label:"Note", attributes:vec![Attribute::new("data-orientation","vertical",None,false),Attribute::new("data-variant","strong",None,false)] }
        }
    });
    assert!(html.contains("role=\"note\""), "{html}");
    assert!(!html.contains("role=\"separator\""), "{html}");
    assert!(html.contains("aria-orientation=\"vertical\""), "{html}");
    assert!(html.contains("data-orientation=\"horizontal\""), "{html}");
    assert!(html.contains("data-variant=\"subtle\""), "{html}");
    assert!(!html.contains("data-variant=\"strong\""), "{html}");
}

#[test]
fn native_ref_events_styling_and_aria_inputs_compose_without_installing_a_controller() {
    let html = render(|| {
        rsx! {
            Divider { id:"native",class:"caller",style:"height:auto;align-self:stretch",lang:"ar",tabindex:"-1","data-testid":"native-test","aria-describedby":"details",onmounted:|_|{},attributes:vec![onclick(|_|{})] }
        }
    });
    for value in [
        "id=\"native\"",
        "caller",
        "height:auto;align-self:stretch",
        "lang=\"ar\"",
        "tabindex=\"-1\"",
        "data-testid=\"native-test\"",
        "aria-describedby=\"details\"",
    ] {
        assert!(html.contains(value), "{html}");
    }
    assert!(!html.contains("data-astryx-feedback-id"), "{html}");
}

#[test]
fn custom_variant_names_remain_open_without_silently_becoming_subtle() {
    for invalid in ["", "bad value", "line\n", "<red>"] {
        assert!(DividerVariant::new(invalid).is_err());
    }
    fn custom() -> Element {
        let variant = DividerVariant::new("custom-rule")?;
        rsx! {Divider{variant}}
    }
    let html = render(custom);
    assert!(html.contains("data-variant=\"custom-rule\""), "{html}");
    assert!(!html.contains("data-variant=\"subtle\""), "{html}");
}

#[component]
fn ChangingDivider() -> Element {
    let is_labelled = use_signal(|| true);
    use_context_provider(|| is_labelled);
    rsx! {Divider{label:if is_labelled(){DividerLabel::from("Stable")}else{DividerLabel::Empty}}}
}

#[test]
fn label_updates_preserve_the_root_and_stable_accessible_relationship() {
    let mut dom = VirtualDom::new(ChangingDivider);
    dom.rebuild_in_place();
    let first = dioxus_ssr::render(&dom);
    let id = first
        .split("aria-labelledby=\"")
        .nth(1)
        .expect("reference")
        .split('"')
        .next()
        .expect("id")
        .to_string();
    let mut is_labelled =
        dom.in_runtime(|| consume_context_from_scope::<Signal<bool>>(ScopeId::APP).expect("state"));
    dom.in_runtime(|| is_labelled.set(false));
    dom.render_immediate_to_vec();
    dom.render_immediate_to_vec();
    assert!(!dioxus_ssr::render(&dom).contains("aria-labelledby="));
    dom.in_runtime(|| is_labelled.set(true));
    dom.render_immediate_to_vec();
    dom.render_immediate_to_vec();
    assert!(dioxus_ssr::render(&dom).contains(&format!("aria-labelledby=\"{id}\"")));
}

#[test]
fn none_name_attributes_do_not_erase_the_derived_label_reference() {
    let html = render(
        || rsx! {Divider{label:"Named",attributes:vec![Attribute::new("aria-labelledby",AttributeValue::None,None,false)]}},
    );
    assert!(
        html.contains("aria-labelledby=\"astryx-divider-label-"),
        "{html}"
    );
}
