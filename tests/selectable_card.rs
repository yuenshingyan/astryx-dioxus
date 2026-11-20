use astryx_dioxus::{
    CardVariant, Elevation, SelectableCard, SelectableCardError, SizeValue, SpacingStep,
};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn native_checkbox_is_first_and_root_adds_no_role_or_tab_stop() {
    let html = render(
        || rsx! { SelectableCard { label: "Option A", is_selected: false, onchange: |_| {}, span { "Content" } } },
    );
    assert_eq!(html.matches("<input").count(), 1);
    assert!(html.find("<input").unwrap() < html.find("<span").unwrap());
    for value in [
        "astryx-card ",
        "astryx-selectable-card",
        "type=\"checkbox\"",
        "aria-label=\"Option A\"",
        "data-selected=\"false\"",
        "data-disabled=\"false\"",
        "data-pressable-container=\"true\"",
    ] {
        assert!(html.contains(value), "{value}: {html}");
    }
    for absent in [
        "role=",
        "tabindex=",
        " disabled",
        " checked",
        "aria-disabled=",
    ] {
        assert!(!html.contains(absent), "{absent}: {html}");
    }
}

#[test]
fn selected_disabled_is_checked_focusable_and_reflected() {
    let html = render(
        || rsx! { SelectableCard { label: "Unavailable", is_selected: true, is_disabled: true, onchange: |_| {}, "Selected" } },
    );
    for value in [
        "checked",
        "data-selected=\"true\"",
        "data-disabled=\"true\"",
        "aria-disabled=\"true\"",
        "data-astryx-container-press=\"false\"",
    ] {
        assert!(html.contains(value), "{value}: {html}");
    }
    assert!(!html.contains(" disabled="));
    assert!(!html.contains("tabindex=\"-1\""));
}

#[test]
fn delegates_sizing_padding_variant_and_shadow_to_card() {
    let html = render(
        || rsx! { SelectableCard { label: "Sizing", is_selected: true, onchange: |_| {}, variant: CardVariant::BLUE, elevation: Elevation::High, padding: SpacingStep::Three, width: SizeValue::from(180), height: SizeValue::from(90), max_width: SizeValue::from("100%") } },
    );
    for value in [
        "data-variant=\"blue\"",
        "data-elevation=\"high\"",
        "astryx-card--elevation-high",
        "--_card-width:180px;",
        "--_card-height:90px;",
        "--_card-max-width:100%;",
        "--_card-padding:var(--spacing-3);",
        "data-astryx-card-scroll=\"true\"",
    ] {
        assert!(html.contains(value), "{value}: {html}");
    }
}

#[test]
fn forwards_caller_attributes_classes_and_style_to_same_root() {
    let html = render(
        || rsx! { SelectableCard { id: "owned", label: "Name", is_selected: false, onchange: |_| {}, class: "caller", style: "margin:7px", lang: "fr", role: "group", "data-extra": "kept" } },
    );
    let root = html.split('>').next().unwrap();
    for value in [
        "caller",
        "astryx-selectable-card",
        "id=\"owned\"",
        "lang=\"fr\"",
        "role=\"group\"",
        "data-extra=\"kept\"",
        "margin:7px",
    ] {
        assert!(root.contains(value), "{value}: {root}");
    }
}

#[test]
fn all_builtin_variants_and_elevations_keep_native_checkbox_semantics() {
    let html = render(
        || rsx! { for variant in [CardVariant::DEFAULT, CardVariant::TRANSPARENT, CardVariant::MUTED, CardVariant::BLUE, CardVariant::CYAN, CardVariant::GRAY, CardVariant::GREEN, CardVariant::ORANGE, CardVariant::PINK, CardVariant::PURPLE, CardVariant::RED, CardVariant::TEAL, CardVariant::YELLOW] { for elevation in [Elevation::None, Elevation::Low, Elevation::Med, Elevation::High] { SelectableCard { label: variant.as_str().to_owned(), variant: variant.clone(), elevation, is_selected: true, onchange: |_| {} } } } },
    );
    assert_eq!(html.matches("type=\"checkbox\"").count(), 52);
    assert_eq!(html.matches("data-selected=\"true\"").count(), 52);
    for elevation in ["none", "low", "med", "high"] {
        assert_eq!(
            html.matches(&format!("data-elevation=\"{elevation}\""))
                .count(),
            13
        );
    }
}

#[test]
fn theme_added_variant_remains_open_and_retains_identity() {
    let html = render(
        || rsx! { SelectableCard { label: "Theme", variant: CardVariant::new("catalog_Tier-7").unwrap(), is_selected: true, onchange: |_| {} } },
    );
    assert!(html.contains("data-variant=\"catalog_Tier-7\""));
    assert!(html.contains("data-selected=\"true\""));
}

#[test]
fn whitespace_only_accessible_label_is_rejected() {
    let html =
        render(|| rsx! { SelectableCard { label: " \n\t", is_selected: false, onchange: |_| {} } });
    assert!(!html.contains("type=\"checkbox\""));
    assert_eq!(
        SelectableCardError::EmptyLabel.to_string(),
        "SelectableCard label must contain non-whitespace text"
    );
}
