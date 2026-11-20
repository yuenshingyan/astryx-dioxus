use astryx_dioxus::{Card, CardVariant, Elevation, SizeValue, SizeValueError, SpacingStep};
use dioxus::events::{onclick, onmounted};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn default_is_one_static_surface_without_semantics_or_interaction() {
    let html = render(|| rsx! {Card{}});
    assert_eq!(html.matches("<div").count(), 1);
    assert_eq!(html.matches("</div>").count(), 1);
    for expected in [
        "astryx-card ",
        "data-variant=\"default\"",
        "data-elevation=\"none\"",
        "astryx-card--elevation-none",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    for absent in [
        "role=",
        "tabindex=",
        "aria-",
        "data-astryx-card-scroll=",
        "--_card-width:",
        "--_card-padding:",
        "script",
    ] {
        assert!(!html.contains(absent), "{absent}: {html}");
    }
}

#[test]
fn thirteen_open_surfaces_and_four_closed_elevations_reflect_independently() {
    let html = render(
        || rsx! {for variant in [CardVariant::DEFAULT,CardVariant::TRANSPARENT,CardVariant::MUTED,CardVariant::BLUE,CardVariant::CYAN,CardVariant::GRAY,CardVariant::GREEN,CardVariant::ORANGE,CardVariant::PINK,CardVariant::PURPLE,CardVariant::RED,CardVariant::TEAL,CardVariant::YELLOW]{for elevation in [Elevation::None,Elevation::Low,Elevation::Med,Elevation::High]{Card{variant:variant.clone(),elevation}}}},
    );
    assert_eq!(html.matches("<div").count(), 52);
    for elevation in [
        Elevation::None,
        Elevation::Low,
        Elevation::Med,
        Elevation::High,
    ] {
        assert_eq!(
            html.matches(&format!("astryx-card--elevation-{}", elevation.as_str()))
                .count(),
            13
        );
    }
    for invalid in ["", " ", "x y", "x]", "色"] {
        assert!(CardVariant::new(invalid).is_err());
    }
    let custom =
        render(|| rsx! {Card{variant:CardVariant::new("catalog_Tier-7").unwrap(),"Custom"}});
    assert!(custom.contains("data-variant=\"catalog_Tier-7\""));
    assert!(!custom.contains("data-variant=\"default\""));
}

#[test]
fn caller_children_remain_free_form_and_text_is_escaped() {
    let html = render(
        || rsx! {Card{h3{"Title"}p{"<script>not markup</script>"}button{r#type:"button","Independent action"}}},
    );
    assert_eq!(html.matches("<div").count(), 1);
    assert!(html.contains("<h3>Title</h3>"));
    assert!(html.contains("&#60;script&#62;not markup&#60;/script&#62;"));
    assert!(!html.contains("<script>"));
    assert!(html.contains("<button"));
}

#[test]
fn sizing_props_preserve_open_css_and_caller_final_style_order() {
    let html = render(
        || rsx! {Card{width:SizeValue::from(320),height:SizeValue::from("calc(100vh - 2rem)"),max_width:SizeValue::from("var(--size, 90vw)"),min_height:SizeValue::from(24.5),class:"caller",style:"width:233px;min-width:190px","Sized"}},
    );
    for expected in [
        "--_card-width:320px;",
        "--_card-height:calc(100vh - 2rem);",
        "--_card-max-width:var(--size, 90vw);",
        "--_card-min-height:24.5px;",
        "width:233px;min-width:190px",
        "data-astryx-card-scroll=\"true\"",
        "caller",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(html.find("--_card-width:320px").unwrap() < html.find("width:233px").unwrap());
}

#[test]
fn literal_auto_and_css_wide_keywords_do_not_confuse_custom_property_inheritance() {
    let html = render(|| {
        rsx! {
         Card{width:SizeValue::from(" /* leading */ \\69 nherit "),height:SizeValue::from("INITIAL"),max_width:SizeValue::from("revert-layer"),min_height:SizeValue::from("unset")}
         Card{height:SizeValue::from(" /* keyword */ \\61 uto ")}
        }
    });
    for expected in [
        "astryx-card--width-inherit",
        "astryx-card--height-initial",
        "astryx-card--max-width-revert-layer",
        "astryx-card--min-height-unset",
    ] {
        assert!(html.contains(expected));
    }
    assert_eq!(html.matches("data-astryx-card-scroll=\"true\"").count(), 1);
    assert!(!SizeValue::from("var(--height, auto)").is_auto());
    assert!(!SizeValue::from("au/**/to").is_auto());
}

#[test]
fn all_eleven_explicit_steps_remain_distinct_from_the_theme_default() {
    let html = render(
        || rsx! {Card{}for padding in [SpacingStep::Zero,SpacingStep::Half,SpacingStep::One,SpacingStep::OneAndHalf,SpacingStep::Two,SpacingStep::Three,SpacingStep::Four,SpacingStep::Five,SpacingStep::Six,SpacingStep::Eight,SpacingStep::Ten]{Card{padding}}},
    );
    assert_eq!(html.matches("--_card-padding:").count(), 11);
    for token in ["0", "0-5", "1", "1-5", "2", "3", "4", "5", "6", "8", "10"] {
        assert!(html.contains(&format!("--_card-padding:var(--spacing-{token});")));
    }
}

#[test]
fn native_props_ref_builders_and_events_reach_the_target_without_forged_axes() {
    let html = render(
        || rsx! {Card{id:"native",class:"caller",style:"margin-inline:7px",role:"region","aria-label":"Promotion",lang:"fr",title:"Native title","data-testid":"native-card",onmounted:|_|{},attributes:vec![onclick(|_|{}),onmounted(|_|{}),Attribute::new("data-variant","wrong",None,false),Attribute::new("data-elevation","wrong",None,false)]}},
    );
    for expected in [
        "id=\"native\"",
        "caller",
        "margin-inline:7px",
        "role=\"region\"",
        "aria-label=\"Promotion\"",
        "lang=\"fr\"",
        "title=\"Native title\"",
        "data-testid=\"native-card\"",
    ] {
        assert!(html.contains(expected), "{expected}: {html}");
    }
    assert!(!html.contains("wrong"));
    assert_eq!(html.matches("<div").count(), 1);
}

#[test]
fn shared_size_validation_is_recoverable_and_keeps_browser_css_grammar_open() {
    assert_eq!(
        SizeValue::from(f64::NAN).validate(),
        Err(SizeValueError::NonFinitePixels)
    );
    assert_eq!(
        SizeValue::from(f64::INFINITY).validate(),
        Err(SizeValueError::NonFinitePixels)
    );
    assert_eq!(
        SizeValue::from(-1).validate(),
        Err(SizeValueError::NegativePixels)
    );
    for value in [
        "",
        " ",
        "/* comment */",
        "10px;color:red",
        "10px!important",
        "10px/*",
        "var(--x",
        "10px\\",
        "\0",
    ] {
        assert_eq!(
            SizeValue::from(value).validate(),
            Err(SizeValueError::InvalidCss),
            "{value:?}"
        );
    }
    for value in [
        "calc(100% - 2rem)",
        "min(40cqw, 20rem)",
        "var(--future, calc(20px + 1em))",
        "inherit",
        "revert-layer",
        "fit-content(60ch)",
        "var(--opaque, {custom;tokens})",
        "-1px",
    ] {
        assert!(SizeValue::from(value).validate().is_ok(), "{value}");
    }
}

#[test]
fn invalid_dimensions_return_errors_through_the_render_boundary() {
    #[component]
    fn Invalids() -> Element {
        rsx! {
         ErrorBoundary{handle_error:|_|rsx!{p{"invalid width"}},Card{width:SizeValue::from(f64::NAN)}}
         ErrorBoundary{handle_error:|_|rsx!{p{"invalid height"}},Card{height:SizeValue::from(-1)}}
         ErrorBoundary{handle_error:|_|rsx!{p{"invalid maximum"}},Card{max_width:SizeValue::from("10px;color:red")}}
         ErrorBoundary{handle_error:|_|rsx!{p{"invalid minimum"}},Card{min_height:SizeValue::from("var(--x")}}
        }
    }
    let mut dom = VirtualDom::new(Invalids);
    dom.rebuild_in_place();
    dom.render_immediate_to_vec();
    let html = dioxus_ssr::render(&dom);
    for text in [
        "invalid width",
        "invalid height",
        "invalid maximum",
        "invalid minimum",
    ] {
        assert!(html.contains(text));
    }
    assert!(!html.contains("astryx-card"));
}
