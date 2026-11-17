use astryx_dioxus::{
    ButtonEnvironment, ButtonEnvironmentProvider, ButtonGroup, ButtonGroupElevation,
    ButtonGroupOrientation, ButtonSizeContext, ControlSize, Icon, IconDefaultSizeProvider,
    IconSize, SizeProvider, Spinner, SpinnerShade, SpinnerSize, use_button_environment,
};
use dioxus::core::{ScopeId, consume_context_from_scope};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[component]
fn EnvironmentProbe() -> Element {
    let environment = use_button_environment();
    let inherited_size = try_consume_context::<ButtonSizeContext>().map(|context| (context.0)());
    let size = environment.size.or(inherited_size).unwrap_or_default();
    rsx! {
        output {
            "data-size": size.as_str(),
            "data-disabled": environment.is_disabled.to_string(),
            "data-grouped": environment.is_grouped.to_string(),
            "data-orientation": environment.orientation.as_str(),
        }
    }
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

#[test]
fn icon_is_decorative_by_default_and_named_only_when_meaningful() {
    let html = render(|| rsx! { Icon { icon: rsx! { svg {} } } });
    assert!(html.contains("data-size=\"md\""), "{html}");
    assert!(html.contains("1.25rem"), "{html}");
    assert!(html.contains("aria-hidden=\"true\""), "{html}");
    assert!(!html.contains("role=\"img\""), "{html}");
    let html = render(|| {
        rsx! { Icon {
            icon: rsx! { svg {} }, label: "Success", color: "var(--color-success)"
        } }
    });
    let root = html.split('>').next().expect("root");
    assert!(root.contains("role=\"img\""), "{html}");
    assert!(root.contains("aria-label=\"Success\""), "{html}");
    assert!(!root.contains("aria-hidden="), "{html}");
    assert!(root.contains("color:var(--color-success)"), "{html}");
    let html = render(|| rsx! { Icon { icon: rsx! { svg {} }, label: "  " } });
    assert!(!html.contains("role=\"img\""), "{html}");
}

#[test]
fn icon_size_precedence_is_explicit_then_nearest_provider_then_medium() {
    let html = render(|| {
        rsx! {
            IconDefaultSizeProvider { size: IconSize::Small,
                Icon { icon: rsx! { svg {} } }
                Icon { size: IconSize::Large, icon: rsx! { svg {} } }
                IconDefaultSizeProvider { size: IconSize::ExtraSmall,
                    Icon { icon: rsx! { svg {} } }
                }
                Icon { icon: rsx! { svg {} } }
            }
        }
    });
    assert_eq!(html.matches("data-size=\"sm\"").count(), 2, "{html}");
    for value in [
        "data-size=\"lg\"",
        "data-size=\"xsm\"",
        "1rem",
        "1.5rem",
        "0.75rem",
    ] {
        assert!(html.contains(value), "missing {value}: {html}");
    }
}

#[test]
fn environment_is_optional_and_size_provider_is_headless() {
    let html = render(|| rsx! { EnvironmentProbe {} });
    for value in [
        "data-size=\"md\"",
        "data-disabled=\"false\"",
        "data-grouped=\"false\"",
        "data-orientation=\"horizontal\"",
    ] {
        assert!(html.contains(value), "missing {value}: {html}");
    }
    let html = render(|| {
        rsx! {
            SizeProvider { size: ControlSize::Large, EnvironmentProbe {} }
        }
    });
    assert!(html.starts_with("<output"), "{html}");
    assert!(html.contains("data-size=\"lg\""), "{html}");
}

#[test]
fn nearest_environment_replaces_outer_flags_and_size() {
    let html = render(|| {
        rsx! {
            ButtonEnvironmentProvider {
                environment: ButtonEnvironment {
                    size: Some(ControlSize::Large), is_disabled: true, is_grouped: true,
                    orientation: ButtonGroupOrientation::Vertical,
                },
                EnvironmentProbe {}
                ButtonEnvironmentProvider {
                    environment: ButtonEnvironment::default(),
                    SizeProvider { size: ControlSize::Small, EnvironmentProbe {} }
                }
                EnvironmentProbe {}
            }
        }
    });
    assert_eq!(html.matches("data-disabled=\"true\"").count(), 2, "{html}");
    assert_eq!(html.matches("data-grouped=\"true\"").count(), 2, "{html}");
    assert_eq!(html.matches("data-size=\"lg\"").count(), 2, "{html}");
    assert!(
        html.contains("data-size=\"sm\" data-disabled=\"false\" data-grouped=\"false\""),
        "{html}"
    );
}

#[test]
fn group_exposes_controller_hooks_and_supplies_environment() {
    let html = render(|| {
        rsx! {
            SizeProvider { size: ControlSize::Small,
                ButtonGroup {
                    label: "Editing", is_disabled: true,
                    orientation: ButtonGroupOrientation::Vertical,
                    elevation: ButtonGroupElevation::High,
                    EnvironmentProbe {}
                }
            }
        }
    });
    for value in [
        "astryx-button-group--vertical",
        "data-astryx-button-group=\"\"",
        "role=\"group\"",
        "aria-label=\"Editing\"",
        "data-elevation=\"high\"",
        "aria-disabled=\"true\"",
        "data-grouped=\"true\"",
    ] {
        assert!(html.contains(value), "missing {value}: {html}");
    }
    assert_eq!(html.matches("data-size=\"sm\"").count(), 2, "{html}");
    assert_eq!(
        html.matches("data-orientation=\"vertical\"").count(),
        2,
        "{html}"
    );
}

#[test]
fn group_defaults_and_explicit_size_override_inherited_size() {
    let html = render(|| {
        rsx! {
            SizeProvider { size: ControlSize::Large,
                ButtonGroup { label: "Actions", size: ControlSize::Small, EnvironmentProbe {} }
            }
        }
    });
    for value in ["data-elevation=\"none\"", "data-orientation=\"horizontal\""] {
        assert!(html.contains(value), "missing {value}: {html}");
    }
    assert_eq!(html.matches("data-size=\"sm\"").count(), 2, "{html}");
    assert!(!html.contains("aria-disabled="), "{html}");
}

#[test]
fn remaining_group_elevations_use_upstream_selectors() {
    let html = render(|| {
        rsx! {
            ButtonGroup { label: "Low", elevation: ButtonGroupElevation::Low, span {} }
            ButtonGroup { label: "Medium", elevation: ButtonGroupElevation::Medium, span {} }
        }
    });
    assert!(html.contains("data-elevation=\"low\""), "{html}");
    assert!(html.contains("data-elevation=\"med\""), "{html}");
}

#[component]
fn ReactiveProviders() -> Element {
    let is_changed = use_context_provider(|| Signal::new(false));
    let size = if is_changed() {
        ControlSize::Large
    } else {
        ControlSize::Small
    };
    let icon_size = if is_changed() {
        IconSize::Large
    } else {
        IconSize::Small
    };
    rsx! {
        SizeProvider { size,
            ButtonEnvironmentProvider {
                environment: ButtonEnvironment {
                    is_disabled: is_changed(),
                    ..ButtonEnvironment::default()
                },
                EnvironmentProbe {}
            }
            IconDefaultSizeProvider { size: icon_size, Icon { icon: rsx! { svg {} } } }
        }
    }
}

#[test]
fn providers_publish_prop_changes_to_existing_consumers() {
    let mut dom = VirtualDom::new(ReactiveProviders);
    dom.rebuild_in_place();
    let initial = dioxus_ssr::render(&dom);
    assert_eq!(initial.matches("data-size=\"sm\"").count(), 2, "{initial}");
    let mut is_changed = dom.in_runtime(|| {
        consume_context_from_scope::<Signal<bool>>(ScopeId::APP).expect("test state")
    });
    dom.in_runtime(|| is_changed.set(true));
    // Settle both the provider render and its dependent-consumer render wave.
    dom.render_immediate_to_vec();
    dom.render_immediate_to_vec();
    let updated = dioxus_ssr::render(&dom);
    assert_eq!(updated.matches("data-size=\"lg\"").count(), 2, "{updated}");
    assert!(updated.contains("data-disabled=\"true\""), "{updated}");
}
