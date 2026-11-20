use astryx_dioxus::{
    CardVariant, ClickableCard, Elevation, LinkProvider, LinkRenderProps, SizeValue, SpacingStep,
};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dom.render_immediate_to_vec();
    dioxus_ssr::render(&dom)
}

#[test]
fn action_has_one_native_hidden_button_and_unlabelled_surface() {
    let html = render(
        || rsx! {ClickableCard{label:"Settings",onclick:move |_|{},h3{"Settings"}button{"Independent"}}},
    );
    assert_eq!(html.matches("<div").count(), 1);
    assert_eq!(html.matches("<button").count(), 2);
    let root = html.split('>').next().unwrap();
    assert!(root.contains("astryx-card"));
    assert!(root.contains("astryx-clickable-card"));
    for absent in ["role=", "tabindex=", "aria-label="] {
        assert!(!root.contains(absent), "{html}");
    }
    assert!(html.contains("type=\"button\""));
    assert!(html.contains("aria-label=\"Settings\""));
    assert!(html.contains("data-astryx-container-action"));
    assert!(html.find("data-astryx-container-action").unwrap() < html.find("<h3").unwrap());
}

#[test]
fn navigation_wins_over_action_and_keeps_target() {
    let html = render(
        || rsx! {ClickableCard{label:"Docs",href:"/docs",target:"_blank",onclick:move |_|{},"Content"}},
    );
    assert_eq!(html.matches("<a ").count(), 1);
    assert!(!html.contains("<button"));
    assert!(html.contains("href=\"/docs\""));
    assert!(html.contains("target=\"_blank\""));
    assert!(html.contains("tabindex=0"));
}

#[test]
fn disabled_controls_keep_their_tab_stop_name_and_role_without_a_destination() {
    let html = render(
        || rsx! {ClickableCard{label:"Unavailable action",is_disabled:true}ClickableCard{label:"Unavailable link",href:"/docs",target:"_blank",is_disabled:true}},
    );
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 2);
    assert_eq!(html.matches("data-disabled=\"true\"").count(), 2);
    assert_eq!(
        html.matches("data-astryx-container-press=\"false\"")
            .count(),
        2
    );
    assert_eq!(
        html.matches("data-pressable-container=\"true\"").count(),
        2,
        "disabled inner cards must retain nested ownership"
    );
    assert!(html.contains("role=\"link\""));
    for absent in ["href=", "target=", "disabled=\"\"", "tabindex=\"-1\""] {
        assert!(!html.contains(absent), "{html}");
    }
    assert_eq!(html.matches("aria-label=").count(), 2);
}

#[test]
fn blocked_destinations_never_reach_the_native_or_provider_sink() {
    let html = render(
        || rsx! {LinkProvider{component:Callback::new(|_props:LinkRenderProps|rsx!{p{"Provider should not receive blocked destination"}}),for href in ["javascript:run()","java\nscript:run()","vbscript:run()","data: text/html,hello"] {ClickableCard{label:"Blocked",href}}}},
    );
    assert_eq!(html.matches("<a ").count(), 4);
    assert!(!html.contains("href="));
    assert!(!html.contains("Provider should"));
}

#[test]
fn accepted_provider_receives_hidden_control_contract_but_disabled_bypasses_it() {
    let html = render(
        || rsx! {LinkProvider{component:Callback::new(|props:LinkRenderProps|rsx!{a{href:props.href,target:props.target,"data-router":"true",..props.attributes,{props.children}}}),ClickableCard{label:"Enabled",href:"/docs",target:"_self"}ClickableCard{label:"Disabled",href:"/docs",is_disabled:true}}},
    );
    assert_eq!(html.matches("data-router=\"true\"").count(), 1);
    assert_eq!(html.matches("data-astryx-container-action").count(), 2);
    assert!(html.contains("aria-label=\"Enabled\""));
    assert!(html.contains("aria-label=\"Disabled\""));
    assert_eq!(html.matches("href=\"/docs\"").count(), 1);
}

#[test]
fn released_noop_composition_remains_focusable_without_new_action_policy() {
    let html = render(|| rsx! {ClickableCard{label:"No-op","Content"}});
    assert_eq!(html.matches("<button").count(), 1);
    assert!(!html.contains("aria-disabled"));
    assert!(!html.contains("tabindex=\"-1\""));
    assert!(!html.contains("href="));
}

#[test]
fn blank_accessible_labels_fail_at_public_boundary() {
    let html = render(
        || rsx! {for label in ["","  ","\n"] {ErrorBoundary{handle_error:|_|rsx!{p{"Invalid card name"}},ClickableCard{label}}}},
    );
    assert_eq!(html.matches("Invalid card name").count(), 3);
    assert!(!html.contains("<button"));
}

#[test]
fn card_surface_sizing_tones_and_final_caller_styles_are_composed() {
    let html = render(
        || rsx! {ClickableCard{label:"Custom",variant:CardVariant::new("custom_Tier-7").unwrap(),elevation:Elevation::High,padding:SpacingStep::Six,width:SizeValue::from(320),height:SizeValue::from(180),max_width:SizeValue::from("100%"),class:"caller",style:"width:233px;margin:7px",id:"caller-id",lang:"fr","data-test":"native","Content"}},
    );
    for expected in [
        "data-variant=\"custom_Tier-7\"",
        "data-elevation=\"high\"",
        "astryx-card--elevation-high",
        "--_card-width:320px;",
        "--_card-height:180px;",
        "--_card-max-width:100%;",
        "--_card-padding:var(--spacing-6);",
        "width:233px;margin:7px",
        "caller",
        "id=\"caller-id\"",
        "lang=\"fr\"",
        "data-test=\"native\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(html.find("--_card-width:320px").unwrap() < html.find("width:233px").unwrap());
}

#[test]
fn caller_content_is_escaped_and_keeps_independent_semantics() {
    let html = render(
        || rsx! {ClickableCard{label:"<Open>",input{r#type:"checkbox","aria-label":"Nested checkbox"}a{href:"/nested","Nested link"}p{"<script>not code</script>"}}},
    );
    assert!(html.contains("aria-label=\"&#60;Open&#62;\""));
    assert!(html.contains("href=\"/nested\""));
    assert!(html.contains("type=\"checkbox\""));
    assert!(!html.contains("<script>"));
}
