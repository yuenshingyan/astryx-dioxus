use astryx_dioxus::{
    Button, ButtonElevation, ButtonEnvironment, ButtonEnvironmentProvider, ButtonGroup,
    ButtonLinkProps, ButtonLinkProvider, ButtonSize, ControlSize, Icon, SizeProvider,
};
use dioxus::core::consume_context_from_scope;
use dioxus::events::{onmousemove, onwheel};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

fn first_tag(html: &str) -> &str {
    html.split_once('>').expect("root element").0
}

fn custom_link(props: ButtonLinkProps) -> Element {
    rsx! {
        a {
            href: props.href, target: props.target, rel: props.rel,
            onclick: props.onclick, onkeydown: props.onkeydown, onmounted: props.onmounted,
            "data-router": "first",
            ..props.attributes,
            {props.children}
        }
    }
}

fn alternate_link(props: ButtonLinkProps) -> Element {
    rsx! { a { href: props.href, "data-router": "second", ..props.attributes, {props.children} } }
}

#[test]
fn tooltip_descriptions_preserve_existing_ids_and_do_not_repeat_plain_names() {
    let html = render(
        || rsx! { Button { label: "Save", tooltip: "Save changes", "aria-describedby": "existing" } },
    );
    assert!(
        first_tag(&html).contains("aria-describedby=\"existing astryx-button-"),
        "{html}"
    );
    assert!(html.contains("role=\"tooltip\""), "{html}");
    assert!(html.contains("popover=\"manual\""), "{html}");
    let same = render(
        || rsx! { Button { label: "Save", tooltip: " Save ", "aria-describedby": "existing" } },
    );
    assert!(
        first_tag(&same).contains("aria-describedby=\"existing\""),
        "{same}"
    );
    assert_eq!(same.matches("role=\"tooltip\"").count(), 1, "{same}");
}

#[test]
fn empty_tooltips_do_not_create_empty_accessibility_descriptions() {
    let html = render(|| rsx! { Button { label: "Save", tooltip: "   " } });
    assert!(!html.contains("role=\"tooltip\""), "{html}");
    assert!(!first_tag(&html).contains("aria-describedby"), "{html}");
}

#[test]
fn raw_attributes_cannot_override_owned_pre_mount_form_guards() {
    let html = render(|| {
        rsx! {
            Button {
                label: "Disabled", is_disabled: true,
                attributes: vec![Attribute::new("type", "submit", None, false)],
            }
        }
    });
    assert!(first_tag(&html).contains("type=\"button\""), "{html}");
    assert!(!first_tag(&html).contains("type=\"submit\""), "{html}");
}

#[test]
fn labelledby_keeps_tooltip_description_when_name_is_external() {
    let html = render(
        || rsx! { Button { label: "Save", tooltip: "Save", "aria-labelledby": "external-name" } },
    );
    assert!(first_tag(&html).contains("aria-describedby="), "{html}");
    assert!(
        first_tag(&html).contains("aria-labelledby=\"external-name\""),
        "{html}"
    );
}

#[test]
fn button_inherits_size_but_explicit_size_wins() {
    let html = render(|| {
        rsx! {
            SizeProvider { size: ControlSize::Small,
                Button { label: "Inherited" }
                Button { label: "Explicit", size: ButtonSize::Large }
            }
        }
    });
    assert!(html.contains("data-size=\"sm\""), "{html}");
    assert!(html.contains("data-size=\"lg\""), "{html}");
}

#[test]
fn nearest_size_provider_overrides_the_group_default() {
    let html = render(|| {
        rsx! {
            ButtonGroup { label: "Actions", size: ControlSize::Small,
                SizeProvider { size: ControlSize::Large,
                    Button { label: "Nearest provider" }
                }
            }
        }
    });
    assert!(html.contains("data-size=\"lg\""), "{html}");
}

#[test]
fn group_and_input_environments_reach_button_behavior_and_paint() {
    let html = render(|| {
        rsx! {
            ButtonGroup { label: "Actions", is_disabled: true, size: ControlSize::Small,
                Button { label: "Grouped", elevation: ButtonElevation::High }
            }
            ButtonEnvironmentProvider {
                environment: ButtonEnvironment { is_disabled: true, ..Default::default() },
                Button { label: "Addon", href: "/example" }
            }
        }
    });
    assert!(html.contains("data-astryx-grouped=\"true\""), "{html}");
    assert!(html.contains("data-elevation=\"none\""), "{html}");
    assert!(!html.contains("data-elevation=\"high\""), "{html}");
    assert_eq!(html.matches("<button ").count(), 2, "{html}");
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 3, "{html}");
    assert!(!html.contains(" href="), "{html}");
}

#[test]
fn nested_input_environment_cannot_enable_a_disabled_group_member() {
    let html = render(|| {
        rsx! {
            ButtonGroup { label: "Disabled group", is_disabled: true,
                ButtonEnvironmentProvider { environment: ButtonEnvironment::default(),
                    Button { label: "Still disabled" }
                }
            }
        }
    });
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 2, "{html}");
    assert!(html.contains("data-astryx-grouped=\"true\""), "{html}");
}

#[test]
fn button_uses_shared_svg_spinner_and_contextual_icons() {
    let html = render(|| {
        rsx! {
            Button { label: "Loading", is_loading: true, icon: rsx! { Icon { icon: rsx! { svg {} } } } }
        }
    });
    assert!(html.contains("astryx-spinner__arc"), "{html}");
    assert!(html.contains("data-shade=\"inherit\""), "{html}");
    assert!(html.contains("stroke-linecap=\"round\""), "{html}");
    assert!(html.contains("data-size=\"sm\""), "{html}");
    assert!(
        !html.contains("role=\"progressbar\""),
        "owner announces busy state: {html}"
    );
}

#[test]
fn link_provider_explicit_override_and_disabled_fallback_keep_root_contract() {
    let html = render(|| {
        rsx! {
            ButtonLinkProvider { render: custom_link,
                Button { label: "Provider", href: "/one", tooltip: "Description" }
                Button { label: "Explicit", href: "/two", render_link: alternate_link }
                Button { label: "Disabled", href: "/three", is_disabled: true }
            }
        }
    });
    assert!(html.contains("data-router=\"first\""), "{html}");
    assert!(html.contains("data-router=\"second\""), "{html}");
    assert_eq!(html.matches("data-astryx-button-id=").count(), 3, "{html}");
    assert_eq!(html.matches("<button ").count(), 1, "{html}");
    assert!(!html.contains("/three"), "{html}");
}

#[component]
fn ChangingLinkProvider() -> Element {
    let is_changed = use_context_provider(|| Signal::new(false));
    let first = use_callback(custom_link);
    let second = use_callback(alternate_link);
    rsx! {
        ButtonLinkProvider { render: if is_changed() { second } else { first },
            Button { label: "Router", href: "/example" }
        }
    }
}

#[test]
fn link_provider_updates_existing_consumers() {
    let mut dom = VirtualDom::new(ChangingLinkProvider);
    dom.rebuild_in_place();
    assert!(dioxus_ssr::render(&dom).contains("data-router=\"first\""));
    let mut is_changed = dom.in_runtime(|| {
        consume_context_from_scope::<Signal<bool>>(ScopeId::APP).expect("test state")
    });
    dom.in_runtime(|| is_changed.set(true));
    dom.render_immediate_to_vec();
    dom.render_immediate_to_vec();
    assert!(dioxus_ssr::render(&dom).contains("data-router=\"second\""));
}

#[test]
fn additional_native_events_can_be_forwarded_without_inventing_wrapper_apis() {
    let html = render(|| {
        rsx! {
            Button {
                label: "Native events",
                attributes: vec![onwheel(move |_| {}), onmousemove(move |_| {})],
            }
        }
    });
    assert!(html.contains("Native events"), "{html}");
}

#[test]
fn root_focus_pointer_and_keyboard_callbacks_are_public_props() {
    let html = render(|| {
        rsx! {
            Button {
                label: "Callbacks", onfocus: move |_| {}, onblur: move |_| {},
                onpointerdown: move |_| {}, onpointerup: move |_| {},
                onpointercancel: move |_| {}, onkeyup: move |_| {},
            }
        }
    });
    assert!(html.contains("Callbacks"), "{html}");
}
