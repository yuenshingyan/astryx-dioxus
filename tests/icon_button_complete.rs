use astryx_dioxus::{
    ButtonElevation, ButtonSize, ButtonType, ButtonVariant, Icon, IconButton, IconSize,
    LinkProvider, LinkRenderProps, SizeProvider,
};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
struct IconCaseProps {
    variant: ButtonVariant,
    size: ButtonSize,
    elevation: ButtonElevation,
    is_disabled: bool,
    is_loading: bool,
    is_interruptible: bool,
}

fn icon_case(props: IconCaseProps) -> Element {
    rsx! {
        IconButton {
            label: "Delete conversation", variant: props.variant, size: props.size,
            elevation: props.elevation, is_disabled: props.is_disabled, is_loading: props.is_loading,
            is_interruptible: props.is_interruptible, loading_label: "Deleting",
            icon: rsx! { Icon { icon: rsx! { svg { circle { cx: "12", cy: "12", r: "8" } } } } },
        }
    }
}

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn all_384_variant_size_elevation_and_availability_combinations_preserve_icon_only_contract() {
    for (variant, name) in [
        (ButtonVariant::Primary, "primary"),
        (ButtonVariant::Secondary, "secondary"),
        (ButtonVariant::Ghost, "ghost"),
        (ButtonVariant::Destructive, "destructive"),
    ] {
        for size in [ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large] {
            for (elevation, tier) in [
                (ButtonElevation::None, "none"),
                (ButtonElevation::Low, "low"),
                (ButtonElevation::Medium, "med"),
                (ButtonElevation::High, "high"),
            ] {
                for is_disabled in [false, true] {
                    for is_loading in [false, true] {
                        for is_interruptible in [false, true] {
                            let mut dom = VirtualDom::new_with_props(
                                icon_case,
                                IconCaseProps {
                                    variant,
                                    size,
                                    elevation,
                                    is_disabled,
                                    is_loading,
                                    is_interruptible,
                                },
                            );
                            dom.rebuild_in_place();
                            let html = dioxus_ssr::render(&dom);
                            let root = html.split('>').next().expect("test root");
                            assert_eq!(html.matches("<button ").count(), 1, "{html}");
                            for expected in [
                                "aria-label=\"Delete conversation\"".to_string(),
                                "data-icon-only=\"true\"".to_string(),
                                format!("data-variant=\"{name}\""),
                                format!("data-size=\"{}\"", size.as_str()),
                                format!("data-elevation=\"{tier}\""),
                            ] {
                                assert!(root.contains(&expected), "missing {expected}: {html}");
                            }
                            assert_eq!(root.contains("aria-busy=\"true\""), is_loading, "{html}");
                            assert_eq!(
                                root.contains("aria-disabled=\"true\""),
                                is_disabled || (is_loading && !is_interruptible),
                                "{html}"
                            );
                            assert!(!root.contains(" disabled="), "must stay focusable: {html}");
                            assert!(!html.contains("astryx-button__label"), "{html}");
                            assert!(!html.contains("astryx-button__end"), "{html}");
                            assert_eq!(
                                html.matches("Delete conversation").count(),
                                1,
                                "label is only the name: {html}"
                            );
                            let icon = html
                                .split("class=\"astryx-icon ")
                                .nth(1)
                                .expect("test Icon")
                                .split('>')
                                .next()
                                .expect("Icon root");
                            assert!(
                                icon.contains(if size == ButtonSize::Large {
                                    "data-size=\"md\""
                                } else {
                                    "data-size=\"sm\""
                                }),
                                "{html}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn label_alone_does_not_invent_a_tooltip_and_raw_attributes_cannot_change_content_mode() {
    let html = render(|| {
        rsx! {
            IconButton { label: "Settings", icon: rsx! { svg {} }, "data-icon-only": "false", "data-size": "wrong", "data-variant": "wrong" }
        }
    });
    assert!(!html.contains("role=\"tooltip\""), "{html}");
    assert!(!html.contains("aria-describedby"), "{html}");
    assert!(html.contains("data-icon-only=\"true\""), "{html}");
    assert!(!html.contains("wrong"), "{html}");
    assert!(!html.contains("astryx-button__label"), "{html}");
}

#[test]
fn tooltip_descriptions_keep_names_and_existing_description_ids() {
    let html = render(|| {
        rsx! {
            IconButton { label: "Search", tooltip: "Find a conversation", "aria-describedby": "search-help", icon: rsx! { svg {} } }
            IconButton { label: "More options", tooltip: "More options", icon: rsx! { svg {} } }
        }
    });
    assert!(html.contains("aria-label=\"Search\""), "{html}");
    assert!(html.contains("aria-describedby=\"search-help "), "{html}");
    assert_eq!(
        html.matches("aria-describedby=").count(),
        1,
        "identical tooltip must not repeat name: {html}"
    );
    assert_eq!(html.matches("role=\"tooltip\"").count(), 2, "{html}");
}

#[test]
fn explicit_button_and_icon_sizes_override_context_and_width_is_not_forced_square() {
    let html = render(|| {
        rsx! {
            SizeProvider { size: ButtonSize::Small,
                IconButton { label: "Inherited", icon: rsx! { Icon { icon: rsx! { svg {} } } } }
                IconButton { label: "Override", size: ButtonSize::Large, width: "96px", class: "caller-class", style: "outline-offset:3px", icon: rsx! { Icon { size: IconSize::Large, icon: rsx! { svg {} } } } }
            }
        }
    });
    assert!(html.contains("data-size=\"sm\""), "{html}");
    assert_eq!(
        html.matches("data-size=\"lg\"").count(),
        2,
        "button and explicit Icon: {html}"
    );
    assert!(html.contains("width: 96px"), "{html}");
    assert!(html.contains("caller-class"), "{html}");
    assert!(html.contains("outline-offset:3px"), "{html}");
}

fn router(props: LinkRenderProps) -> Element {
    rsx! { a { href: props.href, target: props.target, rel: props.rel, onclick: props.onclick, onkeydown: props.onkeydown, onmounted: props.onmounted, "data-adapter": "icon-router", ..props.attributes, {props.children} } }
}

#[test]
fn links_use_shared_or_explicit_renderer_and_block_invalid_or_disabled_destinations() {
    let html = render(|| {
        rsx! {
            LinkProvider { component: router,
                IconButton { label: "Provider", href: "/provider", target: "_blank", rel: "help", icon: rsx! { svg {} } }
                IconButton { label: "Disabled", href: "/disabled", is_disabled: true, icon: rsx! { svg {} } }
                IconButton { label: "Unsafe", href: "javascript:alert(1)", icon: rsx! { svg {} } }
            }
            IconButton { label: "Explicit", href: "/explicit", render_link: router, icon: rsx! { svg {} } }
        }
    });
    assert_eq!(
        html.matches("data-adapter=\"icon-router\"").count(),
        2,
        "{html}"
    );
    assert!(html.contains("target=\"_blank\""), "{html}");
    assert!(html.contains("rel=\"help\""), "{html}");
    assert!(!html.contains("href=\"/disabled\""), "{html}");
    assert!(!html.contains("javascript:"), "{html}");
    assert_eq!(html.matches("<button ").count(), 2, "{html}");
}

#[test]
fn form_types_and_fields_keep_pre_mount_activation_guards() {
    let html = render(|| {
        rsx! {
            IconButton { label: "Submit", button_type: ButtonType::Submit, name: "action", value: "save", form: "editor", icon: rsx! { svg {} } }
            IconButton { label: "Reset", button_type: ButtonType::Reset, icon: rsx! { svg {} } }
            IconButton { label: "Blocked", button_type: ButtonType::Submit, is_loading: true, icon: rsx! { svg {} } }
        }
    });
    for expected in [
        "type=\"submit\"",
        "type=\"reset\"",
        "type=\"button\"",
        "name=\"action\"",
        "value=\"save\"",
        "form=\"editor\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
}
