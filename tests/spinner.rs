use astryx_dioxus::{Spinner, SpinnerShade, SpinnerSize};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn spinner_defaults_are_indeterminate_svg_not_border_approximation() {
    let html = render(|| rsx! { Spinner {} });
    for attribute in [
        "data-size=\"md\"",
        "data-shade=\"default\"",
        "role=\"progressbar\"",
        "aria-label=\"Loading\"",
        "r=7",
        "stroke-width=3",
        "stroke-linecap=\"round\"",
        "data-astryx-spinner-rotor=\"\"",
    ] {
        assert!(html.contains(attribute), "missing {attribute}: {html}");
    }
    assert_eq!(html.matches("<circle").count(), 2, "{html}");
    assert!(html.contains("20px"), "{html}");
    assert!(!html.contains("aria-valuenow"), "{html}");
    assert!(!html.contains("border:"), "{html}");
    let dash = html
        .split("stroke-dasharray=\"")
        .nth(1)
        .expect("arc dash pattern")
        .split('"')
        .next()
        .expect("dash value");
    let lengths: Vec<f64> = dash
        .split_whitespace()
        .map(|part| part.parse().expect("numeric dash"))
        .collect();
    assert_eq!(lengths.len(), 2);
    assert!((lengths[0] / (lengths[0] + lengths[1]) - 0.375).abs() < 1e-12);
}

#[test]
fn spinner_sizes_and_shades_preserve_upstream_geometry_and_alpha() {
    let html = render(|| {
        rsx! {
            Spinner { size: SpinnerSize::Small, shade: SpinnerShade::Subtle }
            Spinner { size: SpinnerSize::Large, shade: SpinnerShade::OnMedia }
            Spinner { size: SpinnerSize::ExtraLarge, shade: SpinnerShade::Inherit }
        }
    });
    for value in [
        "14px",
        "24px",
        "36px",
        "r=5",
        "r=9",
        "r=14",
        "stroke-width=2",
        "stroke-width=4",
        "data-shade=\"subtle\"",
        "data-shade=\"onMedia\"",
        "data-shade=\"inherit\"",
        "stroke-opacity=0.3",
    ] {
        assert!(html.contains(value), "missing {value}: {html}");
    }
    assert!(
        html.contains(&format!("stroke-opacity={}", 77.0 / 255.0)),
        "{html}"
    );
}

#[test]
fn visible_spinner_label_names_indicator_without_duplicate_aria_label() {
    let html = render(|| rsx! { Spinner { label: "Chargement" } });
    let labelled_by = html
        .split("aria-labelledby=\"")
        .nth(1)
        .expect("label reference")
        .split('"')
        .next()
        .expect("label id");
    assert!(html.contains(&format!("id=\"{labelled_by}\"")), "{html}");
    assert!(html.contains(">Chargement</span>"), "{html}");
    assert!(!html.contains("aria-label="), "{html}");
    assert!(!html.contains("Loading"), "{html}");
}

#[test]
fn spinner_name_override_and_owner_silencing_are_explicit() {
    let html = render(|| {
        rsx! {
            Spinner { label: "Visible", accessible_label: "Accessible" }
            Spinner { is_decorative: true, accessible_label: "Hidden" }
        }
    });
    assert!(html.contains("aria-label=\"Accessible\""), "{html}");
    assert!(!html.contains("aria-labelledby="), "{html}");
    assert!(!html.contains("aria-label=\"Hidden\""), "{html}");
    assert_eq!(html.matches("role=\"progressbar\"").count(), 1, "{html}");
    assert!(html.contains("aria-hidden=\"true\""), "{html}");
}
