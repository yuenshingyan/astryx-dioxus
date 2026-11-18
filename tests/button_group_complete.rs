use astryx_dioxus::{
    Button, ButtonElevation, ButtonGroup, ButtonGroupElevation, ButtonGroupOrientation,
    ElementSize, Icon, IconButton, IconDefaultSizeProvider, IconSize, LayerContentBoundary,
    LinkProvider, LinkRenderProps, SizeProvider,
};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[derive(Clone, PartialEq, Props)]
struct GroupCaseProps {
    orientation: ButtonGroupOrientation,
    size: ElementSize,
    elevation: ButtonGroupElevation,
    is_disabled: bool,
}

fn group_case(props: GroupCaseProps) -> Element {
    rsx! {
        ButtonGroup {
            label: "Editing", orientation: props.orientation, size: props.size,
            elevation: props.elevation, is_disabled: props.is_disabled,
            Button { label: "Copy", elevation: ButtonElevation::High }
            IconButton { label: "Options", icon: rsx! { Icon { icon: rsx! { svg {} } } }, elevation: ButtonElevation::High }
        }
    }
}

#[test]
fn every_orientation_size_elevation_and_disabled_state_preserves_the_group_contract() {
    for (orientation, direction) in [
        (ButtonGroupOrientation::Horizontal, "horizontal"),
        (ButtonGroupOrientation::Vertical, "vertical"),
    ] {
        for size in [ElementSize::Small, ElementSize::Medium, ElementSize::Large] {
            for (elevation, tier) in [
                (ButtonGroupElevation::None, "none"),
                (ButtonGroupElevation::Low, "low"),
                (ButtonGroupElevation::Medium, "med"),
                (ButtonGroupElevation::High, "high"),
            ] {
                for is_disabled in [false, true] {
                    let mut dom = VirtualDom::new_with_props(
                        group_case,
                        GroupCaseProps {
                            orientation,
                            size,
                            elevation,
                            is_disabled,
                        },
                    );
                    dom.rebuild_in_place();
                    let html = dioxus_ssr::render(&dom);
                    let root = html.split('>').next().expect("test root");
                    assert!(root.contains("role=\"group\""), "{html}");
                    assert!(root.contains("aria-label=\"Editing\""), "{html}");
                    assert!(
                        root.contains(&format!("data-orientation=\"{direction}\"")),
                        "{html}"
                    );
                    assert!(
                        root.contains(&format!("data-elevation=\"{tier}\"")),
                        "{html}"
                    );
                    assert_eq!(
                        html.matches(&format!("data-size=\"{}\"", size.as_str()))
                            .count(),
                        3 + usize::from(size == ElementSize::Small),
                        "group, two buttons, and contextual Icon: {html}"
                    );
                    assert_eq!(
                        html.matches("data-astryx-grouped=\"true\"").count(),
                        2,
                        "{html}"
                    );
                    assert_eq!(
                        html.matches("data-elevation=\"none\"").count(),
                        if tier == "none" { 3 } else { 2 },
                        "{html}"
                    );
                    assert_eq!(
                        html.matches("aria-disabled=\"true\"").count(),
                        if is_disabled { 3 } else { 0 },
                        "{html}"
                    );
                    assert!(
                        !html.contains(" disabled="),
                        "disabled members must remain focusable: {html}"
                    );
                    assert!(!html.contains("aria-orientation"), "{html}");
                }
            }
        }
    }
}

#[test]
fn empty_accessible_labels_report_a_recoverable_render_error() {
    let mut dom = VirtualDom::new(|| {
        rsx! {
            ErrorBoundary {
                handle_error: move |_| rsx! { output { id: "group-validation-error", "Invalid group name" } },
                ButtonGroup { label: "   ", Button { label: "Action" } }
            }
        }
    });
    dom.rebuild_in_place();
    // The child reports its error after the boundary's initial render.
    dom.render_immediate_to_vec();
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("group-validation-error"), "{html}");
    assert!(!html.contains("role=\"group\""), "{html}");
}

#[test]
fn root_attributes_callbacks_and_owned_semantics_are_forwarded_without_conflict() {
    let html = render(|| {
        rsx! {
            ButtonGroup {
                label: "Actions", id: "group", "data-testid": "group-test", dir: "rtl",
                class: "caller-class", style: "margin:4px", is_disabled: true,
                onkeydown: move |_: KeyboardEvent| {}, onfocus: move |_: FocusEvent| {},
                onmounted: move |_: MountedEvent| {},
                attributes: vec![Attribute::new("role", "toolbar", None, false), Attribute::new("data-size", "lg", None, false), Attribute::new("aria-disabled", "false", None, false)],
                Button { label: "Action" }
            }
        }
    });
    let root = html.split('>').next().expect("test root");
    for attribute in [
        "id=\"group\"",
        "data-testid=\"group-test\"",
        "dir=\"rtl\"",
        "caller-class",
        "margin:4px",
        "role=\"group\"",
        "data-size=\"md\"",
        "aria-disabled=\"true\"",
    ] {
        assert!(root.contains(attribute), "missing {attribute}: {html}");
    }
    assert!(!root.contains("toolbar"), "{html}");
}

#[test]
fn an_enabled_group_does_not_reenable_a_disabled_child_or_disable_an_interruptible_child() {
    let html = render(|| {
        rsx! {
            ButtonGroup { label: "Availability",
                Button { label: "Disabled", is_disabled: true }
                Button { label: "Pending", is_loading: true, is_interruptible: true }
                Button { label: "Ready" }
            }
        }
    });
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 1, "{html}");
    assert_eq!(html.matches("aria-busy=\"true\"").count(), 1, "{html}");
}

#[test]
fn explicit_member_size_overrides_the_group_and_its_outer_provider() {
    let html = render(|| {
        rsx! {
            SizeProvider { size: ElementSize::Large,
                ButtonGroup { label: "Sizing", size: ElementSize::Small,
                    Button { label: "Inherited" }
                    Button { label: "Explicit", size: ElementSize::Medium }
                }
            }
        }
    });
    assert_eq!(html.matches("data-size=\"sm\"").count(), 2, "{html}");
    assert_eq!(html.matches("data-size=\"md\"").count(), 1, "{html}");
}

#[test]
fn layer_boundary_ends_surface_defaults_but_allows_new_inner_providers() {
    let html = render(|| {
        rsx! {
            ButtonGroup { label: "Outer", size: ElementSize::Small, is_disabled: true,
                Button { label: "Outer disabled" }
                LayerContentBoundary {
                    Button { label: "Layer action", elevation: ButtonElevation::High }
                    SizeProvider { size: ElementSize::Large, Button { label: "Layer sized" } }
                    ButtonGroup { label: "Inner", Button { label: "Inner action" } }
                }
            }
        }
    });
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 2, "{html}");
    assert_eq!(
        html.matches("data-astryx-grouped=\"true\"").count(),
        2,
        "outer and inner group members only: {html}"
    );
    assert_eq!(html.matches("data-elevation=\"high\"").count(), 1, "{html}");
    assert!(html.contains("data-size=\"lg\""), "{html}");
}

fn router(props: LinkRenderProps) -> Element {
    rsx! { a { href: props.href, onclick: props.onclick, onkeydown: props.onkeydown, onmounted: props.onmounted, "data-router": "kept", ..props.attributes, {props.children} } }
}

#[test]
fn layer_boundary_resets_icon_defaults_without_discarding_application_routing() {
    let html = render(|| {
        rsx! {
            LinkProvider { component: router,
                IconDefaultSizeProvider { size: IconSize::Large,
                    LayerContentBoundary {
                        Icon { icon: rsx! { svg {} } }
                        Button { label: "Link in layer", href: "/layer" }
                    }
                }
            }
        }
    });
    assert!(html.contains("data-router=\"kept\""), "{html}");
    assert!(html.contains("1.25rem"), "{html}");
    assert!(!html.contains("1.5rem"), "{html}");
}
