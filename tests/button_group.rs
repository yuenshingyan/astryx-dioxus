use astryx_dioxus::{
    ButtonGroup, ButtonGroupElevation, ButtonGroupOrientation, ButtonSizeContext, ControlSize,
    SizeProvider, use_button_environment,
};
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
