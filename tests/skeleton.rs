use astryx_dioxus::{Skeleton, SkeletonLength, SkeletonRadius};
use dioxus::core::{ScopeId, consume_context_from_scope};
use dioxus::events::onclick;
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn defaults_are_one_empty_hidden_div_with_percentage_geometry_and_no_status_ownership() {
    let html = render(|| rsx! {Skeleton{}});
    assert_eq!(html.matches("<div").count(), 1, "{html}");
    assert!(html.contains("aria-hidden=\"true\""), "{html}");
    for value in [
        "--_skeleton-width:100%",
        "--_skeleton-height:100%",
        "--_skeleton-radius:var(--radius-container)",
        "--_skeleton-delay:1000ms",
    ] {
        assert!(html.contains(value), "{html}");
    }
    for absent in [
        "role=",
        "aria-busy=",
        "aria-live=",
        "tabindex=",
        "data-radius=",
        "data-index=",
        "<span",
        "<style",
        "<script",
    ] {
        assert!(!html.contains(absent), "{absent}: {html}");
    }
    assert!(html.ends_with("></div>"), "{html}");
}

#[test]
fn pixel_css_and_zero_dimensions_stagger_and_all_radius_levels_reach_geometry_not_theme_axes() {
    let html = render(|| {
        rsx! {
            for radius in [SkeletonRadius::None, SkeletonRadius::Zero, SkeletonRadius::One, SkeletonRadius::Two, SkeletonRadius::Three, SkeletonRadius::Four, SkeletonRadius::Rounded] {
                Skeleton{radius,width:12.5,height:0,index:3}
            }
            Skeleton{width:"calc(100% - 2rem)",height:"var(--row-height, 16px)",index:u32::MAX}
        }
    });
    assert_eq!(html.matches("<div").count(), 8);
    assert_eq!(html.matches("--_skeleton-width:12.5px").count(), 7);
    assert_eq!(html.matches("--_skeleton-height:0px").count(), 7);
    assert_eq!(html.matches("--_skeleton-delay:1300ms").count(), 7);
    assert_eq!(
        html.matches("--_skeleton-radius:var(--radius-container)")
            .count(),
        3
    );
    for value in [
        "--_skeleton-radius:0px",
        "var(--radius-none)",
        "var(--radius-inner)",
        "var(--radius-element)",
        "var(--radius-full)",
        "calc(100% - 2rem)",
        "var(--row-height, 16px)",
        "--_skeleton-delay:429496730500ms",
    ] {
        assert!(html.contains(value), "{value}: {html}");
    }
    assert!(!html.contains("data-radius="));
    assert!(!html.contains("data-index="));
}

#[test]
fn caller_can_expose_and_name_shape_and_forward_native_props_ref_events_and_style_overrides() {
    let html = render(|| {
        rsx! {Skeleton{
            width:200,height:20,id:"placeholder",class:"caller",style:"width:33px;opacity:0.8",role:"img",
            "aria-hidden":"false","aria-label":"Loading preview","data-testid":"native-skeleton",lang:"fr",
            onmounted:|_|{},attributes:vec![onclick(|_|{})],
        }}
    });
    for value in [
        "id=\"placeholder\"",
        "caller",
        "width:33px;opacity:0.8",
        "role=\"img\"",
        "aria-hidden=\"false\"",
        "aria-label=\"Loading preview\"",
        "data-testid=\"native-skeleton\"",
        "lang=\"fr\"",
    ] {
        assert!(html.contains(value), "{value}: {html}");
    }
    assert_eq!(html.matches("aria-hidden=").count(), 1);
    assert!(!html.contains("aria-hidden=\"true\""));
    assert!(!html.contains("aria-busy="));
}

#[test]
fn css_wide_dimensions_apply_to_real_properties_and_classification_does_not_merge_comment_tokens() {
    let html = render(|| {
        rsx! {
            Skeleton{width:" /*note*/ INHERIT ",height:"inherit"}
            Skeleton{width:"initial",height:"unset"}
            Skeleton{width:"revert",height:"revert-layer"}
            Skeleton{width:"in/**/herit",height:"auto"}
        }
    });
    for class in [
        "astryx-skeleton--width-inherit",
        "astryx-skeleton--height-inherit",
        "astryx-skeleton--width-initial",
        "astryx-skeleton--height-unset",
        "astryx-skeleton--width-revert",
        "astryx-skeleton--height-revert-layer",
    ] {
        assert_eq!(html.matches(class).count(), 1, "{class}: {html}");
    }
    assert_eq!(html.matches("class=\"astryx-skeleton ").count(), 4);
}

#[test]
fn css_wide_dimensions_distinguish_keyword_trivia_from_identifier_characters() {
    let html = render(|| {
        let width = " \\69 nherit /* trailing comment */";
        let height = "\tunset\n";
        rsx! { Skeleton { width, height } }
    });
    assert!(html.contains("astryx-skeleton--width-inherit"), "{html}");
    assert!(html.contains("astryx-skeleton--height-unset"), "{html}");

    let html = render(|| {
        rsx! {
            for (width, height) in [("\u{a0}inherit", "unset\u{a0}"), (r"\ inherit", r"unset\20"), ("\\\ninherit", "unset\u{b}")] {
                Skeleton { width, height }
            }
        }
    });
    assert_eq!(html.matches("<div").count(), 3, "{html}");
    assert!(!html.contains("astryx-skeleton--width-"), "{html}");
    assert!(!html.contains("astryx-skeleton--height-"), "{html}");
    assert!(html.contains(r"--_skeleton-width:\ inherit;"), "{html}");
    assert!(html.contains(r"--_skeleton-height:unset\20;"), "{html}");
}

#[test]
fn changed_geometry_keeps_the_root_contract_and_stagger_is_not_cached() {
    #[component]
    fn App() -> Element {
        let is_changed = use_signal(|| false);
        use_context_provider(|| is_changed);
        rsx! {Skeleton{
            id:"stable",width:if is_changed(){240}else{120},height:16,
            radius:if is_changed(){SkeletonRadius::Rounded}else{SkeletonRadius::None},
            index:if is_changed(){5}else{0},
        }}
    }
    let mut dom = VirtualDom::new(App);
    dom.rebuild_in_place();
    let before = dioxus_ssr::render(&dom);
    let mut is_changed =
        dom.in_runtime(|| consume_context_from_scope::<Signal<bool>>(ScopeId::APP).unwrap());
    dom.in_runtime(|| is_changed.set(true));
    dom.render_immediate_to_vec();
    let after = dioxus_ssr::render(&dom);
    assert!(before.contains("--_skeleton-width:120px"));
    assert!(before.contains("--_skeleton-delay:1000ms"));
    assert!(after.contains("--_skeleton-width:240px"));
    assert!(after.contains("--_skeleton-delay:1500ms"));
    assert!(after.contains("--_skeleton-radius:var(--radius-full)"));
    assert_eq!(after.matches("id=\"stable\"").count(), 1);
}

#[test]
fn invalid_pixels_and_declaration_breaking_css_return_recoverable_errors() {
    #[component]
    fn Invalids() -> Element {
        rsx! {
            for width in [SkeletonLength::Pixels(-1.0),SkeletonLength::Pixels(f64::NAN),SkeletonLength::Pixels(f64::INFINITY),"".into(),"20px;color:red".into(),"/*open".into()] {
                ErrorBoundary {handle_error:|_|rsx!{p{"invalid width"}},Skeleton{width,height:20}}
            }
            ErrorBoundary {handle_error:|_|rsx!{p{"invalid height"}},Skeleton{width:20,height:-2}}
        }
    }
    let mut dom = VirtualDom::new(Invalids);
    dom.rebuild_in_place();
    dom.render_immediate_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert_eq!(html.matches("invalid width").count(), 6, "{html}");
    assert_eq!(html.matches("invalid height").count(), 1, "{html}");
    assert!(!html.contains("astryx-skeleton"));
}
