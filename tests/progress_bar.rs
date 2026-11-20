use astryx_dioxus::{ProgressBar, ProgressBarMark, ProgressBarState, ProgressBarVariant};
use dioxus::core::{ScopeId, consume_context_from_scope};
use dioxus::events::{onclick, onmounted};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn defaults_are_named_empty_accent_progress_with_no_extra_public_label_or_value_targets() {
    let html = render(|| rsx! {ProgressBar{label:"Upload progress"}});
    for value in [
        "astryx-progress-bar ",
        "data-variant=\"accent\"",
        "role=\"progressbar\"",
        "aria-valuenow=\"0\"",
        "aria-valuemin=\"0\"",
        "aria-valuemax=\"100\"",
        "aria-valuetext=\"0%\"",
        "Upload progress",
        "--_progress-bar-fill:0%",
    ] {
        assert!(html.contains(value), "{value}: {html}");
    }
    assert_eq!(html.matches("role=\"progressbar\"").count(), 1);
    assert!(!html.contains("astryx-progress-bar-label"));
    assert!(!html.contains("astryx-progress-bar-value"));
    assert!(!html.contains("aria-disabled="));
    assert!(!html.contains("aria-live="));
    assert!(!html.contains("tabindex="));
    assert_eq!(html.matches("astryx-progress-bar__value").count(), 0);
}

#[test]
fn values_and_max_are_finite_clamped_and_nonpositive_ranges_are_empty() {
    let html = render(|| {
        rsx! {
            for (value,max) in [(-0.0,100.0),(-10.0,100.0),(150.0,100.0),(f64::NAN,100.0),(f64::INFINITY,100.0),(3.2,5.0),(10.0,0.0),(10.0,-10.0),(10.0,f64::NAN),(10.0,f64::INFINITY)]{
                ProgressBar{label:"Normalized",state:ProgressBarState::range(value,max),has_value_label:true}
            }
        }
    });
    assert_eq!(html.matches("role=\"progressbar\"").count(), 10);
    assert_eq!(html.matches("aria-valuenow=\"0\"").count(), 8);
    assert_eq!(html.matches("aria-valuemax=\"0\"").count(), 4);
    assert!(html.contains("aria-valuenow=\"100\""));
    assert!(html.contains("aria-valuenow=\"3.2\""));
    assert!(html.contains("aria-valuetext=\"64%\""));
    assert!(!html.contains("NaN"));
    assert!(!html.contains("Infinity"));
    assert!(!html.contains("aria-valuenow=\"-"));
}

#[test]
fn indeterminate_omits_every_value_semantic_and_does_not_call_formatters_or_validate_unused_templates()
 {
    let html = render(|| {
        let unused = "not a percentage message";
        rsx! {ProgressBar{label:"Loading",state:ProgressBarState::Indeterminate,is_disabled:true,has_value_label:true,percentage_template:unused,format_value_label:move|_: (f64,f64)| -> String {panic!("indeterminate formatter called")}}}
    });
    assert!(html.contains("data-mode=\"indeterminate\""));
    assert!(html.contains("data-disabled=\"true\""));
    assert!(html.contains("role=\"progressbar\""));
    assert!(html.contains("Loading"));
    for absent in [
        "aria-valuenow=",
        "aria-valuemin=",
        "aria-valuemax=",
        "aria-valuetext=",
        "astryx-progress-bar__value",
        "astryx-progress-bar-mark",
    ] {
        assert!(!html.contains(absent), "{absent}: {html}");
    }
}

#[test]
fn custom_formatting_receives_normalized_values_and_default_messages_can_localize() {
    let html = render(|| {
        let localized = "{percent}\u{a0}٪";
        rsx! {
            ProgressBar{label:"Disk",state:ProgressBarState::range(200.0,5.0),has_value_label:true,format_value_label:move|(value,max):(f64,f64)|format!("{value} GB / {max} GB")}
            ProgressBar{label:"Localized",state:ProgressBarState::value(42.0),percentage_template:localized,has_value_label:true}
            ProgressBar{label:"Rounding",state:ProgressBarState::value(12.5)}
        }
    });
    assert_eq!(html.matches("5 GB / 5 GB").count(), 2);
    assert_eq!(html.matches("42\u{a0}٪").count(), 2);
    assert!(html.contains("aria-valuetext=\"13%\""));
}

#[test]
fn hidden_names_remain_referenced_and_disabled_changes_paint_metadata_not_aria_state() {
    let html = render(|| {
        rsx! {
            ProgressBar{label:"Hidden upload",is_label_hidden:true,state:ProgressBarState::value(25.0)}
            ProgressBar{label:"Hidden with text",is_label_hidden:true,has_value_label:true,state:ProgressBarState::value(35.0)}
            ProgressBar{label:"Canceled",is_disabled:true,state:ProgressBarState::value(60.0)}
        }
    });
    assert_eq!(html.matches("aria-labelledby=").count(), 3);
    assert!(html.contains("Hidden upload"));
    assert!(html.contains("Hidden with text"));
    assert!(
        html.contains(
            "inset-block-start:0;inset-inline-start:0;pointer-events:none;user-select:none"
        )
    );
    assert!(html.contains("astryx-progress-bar__label--hidden"));
    assert!(html.contains("data-variant=\"disabled\""));
    assert!(!html.contains("aria-disabled="));
    let names: Vec<_> = html
        .split("aria-labelledby=\"")
        .skip(1)
        .map(|part| part.split('"').next().unwrap())
        .collect();
    assert_eq!(names.len(), 3);
    assert_ne!(names[0], names[1]);
    assert_ne!(names[1], names[2]);
    for name in names {
        assert_eq!(html.matches(&format!("id=\"{name}\"")).count(), 1);
    }
}

#[test]
fn marks_are_named_independent_images_clamped_and_not_collapsed_when_positions_repeat() {
    let html = render(|| {
        rsx! {ProgressBar{label:"Goals",state:ProgressBarState::marked(50.0,100.0,vec![
            ProgressBarMark::new(-10.0,"Low"),ProgressBarMark::new(0.0,"Start"),ProgressBarMark::new(50.0,"Reached"),ProgressBarMark::new(200.0,"High"),ProgressBarMark::new(200.0,"High"),ProgressBarMark::new(f64::NAN,""),ProgressBarMark::new(f64::INFINITY,"")
        ])}}
    });
    assert_eq!(html.matches("role=\"img\"").count(), 5);
    assert_eq!(html.matches("tabindex=\"0\"").count(), 5);
    assert_eq!(html.matches("aria-label=\"High\"").count(), 2);
    assert_eq!(html.matches("data-placement=\"fill\"").count(), 3);
    assert_eq!(html.matches("data-placement=\"track\"").count(), 2);
    assert_eq!(html.matches("inset-inline-start:0%").count(), 2);
    assert_eq!(html.matches("inset-inline-start:100%").count(), 2);
    let track_end = html.find("</div></div>").unwrap();
    let first_mark = html.find("role=\"img\"").unwrap();
    assert!(
        track_end < first_mark,
        "marks must be outside the semantic track: {html}"
    );
    assert!(!html.contains("NaN"));
    assert!(!html.contains("Infinity"));
}

#[test]
fn open_variants_and_native_root_attributes_do_not_replace_owned_theme_metadata() {
    for name in ["", " ", "x y", "x]", "色"] {
        assert!(ProgressBarVariant::new(name).is_err());
    }
    let html = render(
        || rsx! {ProgressBar{label:"Native progress",variant:ProgressBarVariant::new("queued_Tier-7").unwrap(),state:ProgressBarState::value(60.0),id:"native",class:"caller",style:"width:280px;margin-inline:7px",lang:"fr",role:"group","aria-describedby":"description","data-testid":"native-progress",onmounted:|_|{},attributes:vec![onclick(|_|{}),onmounted(|_|{}),Attribute::new("data-variant","wrong",None,false),Attribute::new("data-mode","wrong",None,false)]}},
    );
    for value in [
        "id=\"native\"",
        "caller",
        "width:280px;margin-inline:7px",
        "lang=\"fr\"",
        "role=\"group\"",
        "aria-describedby=\"description\"",
        "data-testid=\"native-progress\"",
        "data-variant=\"queued_Tier-7\"",
    ] {
        assert!(html.contains(value), "{value}: {html}");
    }
    assert!(!html.contains("wrong"));
    assert_eq!(html.matches("role=\"progressbar\"").count(), 1);
    assert_eq!(html.matches("data-variant=\"queued_Tier-7\"").count(), 2);
}

#[component]
fn ChangingProgress() -> Element {
    let value = use_signal(|| 25.0);
    use_context_provider(|| value);
    rsx! {ProgressBar{id:"stable",label:"Reactive progress",state:ProgressBarState::value(value()),has_value_label:true}}
}
#[test]
fn controlled_value_updates_change_semantics_and_visual_target_together() {
    let mut dom = VirtualDom::new(ChangingProgress);
    dom.rebuild_in_place();
    let before = dioxus_ssr::render(&dom);
    let mut value =
        dom.in_runtime(|| consume_context_from_scope::<Signal<f64>>(ScopeId::APP).unwrap());
    dom.in_runtime(|| value.set(75.0));
    dom.render_immediate_to_vec();
    dom.render_immediate_to_vec();
    let after = dioxus_ssr::render(&dom);
    assert!(before.contains("aria-valuenow=\"25\""));
    assert!(before.contains("--_progress-bar-fill:25%"));
    assert!(after.contains("aria-valuenow=\"75\""));
    assert!(after.contains("--_progress-bar-fill:75%"));
    assert_eq!(after.matches("id=\"stable\"").count(), 1);
}

#[test]
fn invalid_required_text_returns_recoverable_errors() {
    #[component]
    fn Invalids() -> Element {
        let templates = [
            "no message",
            "{percent}{percent}",
            "{other}",
            "{percent}{other}",
        ];
        rsx! {
            for label in [""," "]{ErrorBoundary{handle_error:|_|rsx!{p{"invalid label"}},ProgressBar{label}}}
            ErrorBoundary{handle_error:|_|rsx!{p{"invalid mark"}},ProgressBar{label:"Valid",state:ProgressBarState::marked(50.0,100.0,vec![ProgressBarMark::new(20.0,"")])}}
            for template in templates{ErrorBoundary{handle_error:|_|rsx!{p{"invalid template"}},ProgressBar{label:"Valid",percentage_template:template}}}
            ErrorBoundary{handle_error:|_|rsx!{p{"invalid value text"}},ProgressBar{label:"Valid",format_value_label:move|_: (f64,f64)|" ".to_owned()}}
        }
    }
    let mut dom = VirtualDom::new(Invalids);
    dom.rebuild_in_place();
    dom.render_immediate_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert_eq!(html.matches("invalid label").count(), 2);
    assert_eq!(html.matches("invalid mark").count(), 1);
    assert_eq!(html.matches("invalid template").count(), 4);
    assert_eq!(html.matches("invalid value text").count(), 1);
    assert!(!html.contains("role=\"progressbar\""));
}
