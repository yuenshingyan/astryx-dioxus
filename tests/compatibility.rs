use astryx_dioxus::{
    ButtonEnvironment, ButtonEnvironmentContext, ButtonEnvironmentProvider, ButtonGroup,
    ButtonGroupOrientation, ButtonSizeContext, ControlSize, Icon, IconDefaultSizeProvider,
    IconSize, SizeProvider, use_button_environment,
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

#[component]
fn LegacyContextProbe() -> Element {
    let context = consume_context::<ButtonEnvironmentContext>();
    let environment = (context.0)();
    rsx! { output { "data-disabled": environment.is_disabled.to_string(), "data-size": environment.size.map(ControlSize::as_str) } }
}

#[test]
fn real_groups_still_publish_the_old_public_context_for_existing_consumers() {
    let html = render(|| {
        rsx! {
            ButtonGroup { label: "Existing API", is_disabled: true, size: ControlSize::Small,
                LegacyContextProbe {}
            }
        }
    });
    assert!(
        html.contains("data-disabled=\"true\" data-size=\"sm\""),
        "{html}"
    );
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
