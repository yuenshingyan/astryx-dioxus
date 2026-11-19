use astryx_dioxus::{StatusDot, StatusDotIcon, StatusDotVariant};
use dioxus::core::{ScopeId, consume_context_from_scope};
use dioxus::events::{onclick, onkeydown, onkeyup, onmounted};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn required_tone_and_name_produce_one_childless_static_image_without_focus_or_status_machinery() {
    let html = render(|| rsx! {StatusDot{variant:StatusDotVariant::SUCCESS,label:"Online"}});
    assert_eq!(html.matches("<span").count(), 1, "{html}");
    assert!(html.contains("class=\"astryx-status-dot "), "{html}");
    assert!(html.contains("data-variant=\"success\""), "{html}");
    assert!(html.contains("role=\"img\""), "{html}");
    assert!(html.contains("aria-label=\"Online\""), "{html}");
    for absent in [
        "tabindex=",
        "aria-live=",
        "aria-busy=",
        "aria-expanded=",
        "data-astryx-pulsing=",
        "role=\"tooltip\"",
        "astryx-statusdot",
        "__icon",
    ] {
        assert!(!html.contains(absent), "{absent}: {html}");
    }
    assert!(html.ends_with("></span>"), "{html}");
}

#[test]
fn all_30_tone_pulse_and_icon_cases_keep_name_and_single_target_with_hidden_caller_artwork() {
    let html = render(|| {
        rsx! {
            for variant in [StatusDotVariant::SUCCESS,StatusDotVariant::WARNING,StatusDotVariant::ERROR,StatusDotVariant::ACCENT,StatusDotVariant::NEUTRAL] {
                for is_pulsing in [false,true] {
                    for icon in [StatusDotIcon::Empty,StatusDotIcon::from(0),StatusDotIcon::from(rsx!{svg{width:"8",height:"8",stroke:"currentColor",path{d:"M1 4L3 6L7 2"}}})] {
                        StatusDot{variant:variant.clone(),label:"Accessible status",is_pulsing,icon}
                    }
                }
            }
        }
    });
    assert_eq!(html.matches("role=\"img\"").count(), 30);
    assert_eq!(html.matches("aria-label=\"Accessible status\"").count(), 30);
    assert_eq!(html.matches("data-astryx-pulsing=\"true\"").count(), 15);
    assert_eq!(html.matches("astryx-status-dot__icon").count(), 20);
    assert_eq!(html.matches("aria-hidden=\"true\"").count(), 20);
    assert_eq!(html.matches("<svg").count(), 10);
    assert_eq!(html.matches(">0</span>").count(), 10);
    assert!(!html.contains("astryx-statusdot"));
    assert!(!html.contains("tabindex="));
}

#[test]
fn boolean_absent_and_empty_scalar_icons_do_not_add_wrappers_but_zero_and_rich_fragments_do() {
    let html = render(|| {
        rsx! {
            StatusDot{variant:StatusDotVariant::NEUTRAL,label:"A",icon:false}
            StatusDot{variant:StatusDotVariant::NEUTRAL,label:"B",icon:true}
            StatusDot{variant:StatusDotVariant::NEUTRAL,label:"C",icon:""}
            StatusDot{variant:StatusDotVariant::NEUTRAL,label:"D",icon:None::<Element>}
            StatusDot{variant:StatusDotVariant::NEUTRAL,label:"E",icon:0}
            StatusDot{variant:StatusDotVariant::NEUTRAL,label:"F",icon:rsx!{""}}
            StatusDot{variant:StatusDotVariant::NEUTRAL,label:"G",icon:u128::MAX}
        }
    });
    assert_eq!(html.matches("astryx-status-dot__icon").count(), 3, "{html}");
    assert!(html.contains(">0</span>"));
    assert!(html.contains(&u128::MAX.to_string()));
}

#[test]
fn custom_variants_are_open_validated_and_reflected_without_replacing_their_name_with_neutral() {
    for name in ["", "  ", "new tone", "x]", "x;", "色", "tone\n"] {
        assert!(StatusDotVariant::new(name).is_err(), "{name}");
    }
    let name = "queued_Tier-7";
    assert_eq!(StatusDotVariant::new(name).unwrap().as_str(), name);
    let html = render(
        || rsx! {StatusDot{variant:StatusDotVariant::new("queued_Tier-7").unwrap(),label:"Queued"}},
    );
    assert!(html.contains("data-variant=\"queued_Tier-7\""), "{html}");
    assert!(!html.contains("data-variant=\"neutral\""));
    assert!(html.contains("aria-label=\"Queued\""));
}

#[test]
fn native_semantics_replace_defaults_and_props_ref_events_styles_reach_the_actual_span() {
    let html = render(|| {
        rsx! {StatusDot{
            variant:StatusDotVariant::ERROR,label:"Original",id:"caller",class:"caller-dot",style:"margin-inline:7px",lang:"fr",
            role:"note","aria-label":"Caller-owned","aria-labelledby":"visible-status","aria-hidden":"true",tabindex:"-1","data-testid":"native-status",
            onmounted:|_|{},attributes:vec![onclick(|_|{}),onkeydown(|_|{}),onkeyup(|_|{}),onmounted(|_|{}),Attribute::new("data-variant","wrong",None,false),Attribute::new("data-astryx-tooltip-placement","forged",None,false),Attribute::new("data-astryx-hit-area","forged",None,false)],
        }}
    });
    for value in [
        "id=\"caller\"",
        "caller-dot",
        "margin-inline:7px",
        "lang=\"fr\"",
        "role=\"note\"",
        "aria-label=\"Caller-owned\"",
        "aria-labelledby=\"visible-status\"",
        "aria-hidden=\"true\"",
        "tabindex=\"-1\"",
        "data-testid=\"native-status\"",
        "data-variant=\"error\"",
    ] {
        assert!(html.contains(value), "{value}: {html}");
    }
    assert_eq!(html.matches("role=").count(), 1);
    assert_eq!(html.matches("aria-label=").count(), 1);
    assert!(!html.contains("wrong"));
    assert!(!html.contains("forged"));
    assert!(!html.contains("Original"));
}

#[test]
fn tooltip_ssr_is_a_named_pending_image_without_a_prerendered_surface_and_empty_tips_remain_plain()
{
    let html = render(|| {
        rsx! {
            StatusDot{variant:StatusDotVariant::SUCCESS,label:"Online",tooltip:"Connection verified"}
            StatusDot{variant:StatusDotVariant::ERROR,label:"Offline",tooltip:""}
            StatusDot{variant:StatusDotVariant::NEUTRAL,label:"Unknown",tooltip:"   "}
        }
    });
    assert_eq!(html.matches("role=\"img\"").count(), 3, "{html}");
    assert_eq!(html.matches("role=\"tooltip\"").count(), 0);
    assert_eq!(html.matches("data-astryx-status-tooltip-id=").count(), 1);
    assert!(!html.contains("Connection verified"));
    assert!(!html.contains("popover=\"manual\""));
    assert!(!html.contains("tabindex="));
    assert!(!html.contains("aria-expanded="));
    assert!(!html.contains("role=\"button\""));
    assert_eq!(html.matches("<span").count(), 3);
}

#[component]
fn ChangingStatus() -> Element {
    let has_changed = use_signal(|| false);
    use_context_provider(|| has_changed);
    rsx! {StatusDot{id:"stable",variant:if has_changed(){StatusDotVariant::ERROR}else{StatusDotVariant::SUCCESS},label:if has_changed(){"Offline"}else{"Online"},is_pulsing:has_changed(),icon:if has_changed(){StatusDotIcon::from("!")}else{StatusDotIcon::Empty}}}
}

#[test]
fn name_tone_pulse_and_artwork_props_update_existing_consumers() {
    let mut dom = VirtualDom::new(ChangingStatus);
    dom.rebuild_in_place();
    let before = dioxus_ssr::render(&dom);
    let mut has_changed =
        dom.in_runtime(|| consume_context_from_scope::<Signal<bool>>(ScopeId::APP).unwrap());
    dom.in_runtime(|| has_changed.set(true));
    dom.render_immediate_to_vec();
    dom.render_immediate_to_vec();
    let after = dioxus_ssr::render(&dom);
    assert!(before.contains("data-variant=\"success\""));
    assert!(before.contains("aria-label=\"Online\""));
    assert!(after.contains("data-variant=\"error\""));
    assert!(after.contains("aria-label=\"Offline\""));
    assert!(after.contains("data-astryx-pulsing=\"true\""));
    assert!(after.contains(">!</span>"));
    assert_eq!(after.matches("id=\"stable\"").count(), 1);
}

#[test]
fn empty_status_names_report_recoverable_errors_instead_of_unnamed_images() {
    #[component]
    fn Invalids() -> Element {
        rsx! {for label in ["","  ","\n"] {ErrorBoundary{handle_error:|_|rsx!{p{"Invalid status name"}},StatusDot{variant:StatusDotVariant::SUCCESS,label}}}}
    }
    let mut dom = VirtualDom::new(Invalids);
    dom.rebuild_in_place();
    dom.render_immediate_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert_eq!(html.matches("Invalid status name").count(), 3);
    assert!(!html.contains("astryx-status-dot"));
}
