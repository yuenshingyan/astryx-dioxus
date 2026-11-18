use astryx_dioxus::{
    InteractiveRole, InteractiveRoleProvider, LayerContentBoundary, Link, LinkDownload,
    LinkProvider, LinkRenderProps, Text, TextColor, TextDisplay, TextSize, TextType, TextWeight,
};
use dioxus::prelude::*;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

fn router(props: LinkRenderProps) -> Element {
    let has_destination = props.href.is_some();
    rsx! { a {
        href: props.href, target: props.target, rel: props.rel,
        onclick: props.onclick, onkeydown: props.onkeydown, onmounted: props.onmounted,
        "data-router": "provider", "data-has-destination": has_destination.to_string(),
        ..props.attributes, {props.children}
    } }
}
fn explicit(props: LinkRenderProps) -> Element {
    rsx! { a { href: props.href, target: props.target, rel: props.rel, onclick: props.onclick, onkeydown: props.onkeydown, onmounted: props.onmounted, "data-router": "explicit", ..props.attributes, {props.children} } }
}

#[test]
fn defaults_use_named_native_navigation_and_text_owned_typography() {
    let html = render(|| rsx! { Link { href: "/docs", "Documentation" } });
    assert!(html.starts_with("<a "), "{html}");
    for value in [
        "href=\"/docs\"",
        "astryx-link",
        "data-color=\"accent\"",
        "astryx-link__label",
        "data-type=\"body\"",
        "Documentation",
    ] {
        assert!(html.contains(value), "missing {value}: {html}");
    }
    assert!(
        !html.contains("aria-label="),
        "content names default links: {html}"
    );
    assert!(!html.contains("target="), "{html}");
}

#[test]
fn disabled_and_action_forms_have_distinct_native_semantics() {
    let html = render(|| {
        rsx! {
            Link { href: "/disabled", is_disabled: true, "Disabled navigation" }
            Link { href: "/action", is_disabled: true, onclick: move |_| {}, "Disabled action" }
            Link { onclick: move |_| {}, "Action" }
            Link { "No destination" }
        }
    });
    assert_eq!(html.matches("<a ").count(), 1, "{html}");
    assert_eq!(html.matches("<button ").count(), 3, "{html}");
    assert_eq!(html.matches("type=\"button\"").count(), 3, "{html}");
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 2, "{html}");
    assert!(html.contains("role=\"link\" tabindex=0"), "{html}");
    assert!(!html.contains("href="), "{html}");
    assert!(
        !html.contains(" disabled="),
        "must remain focusable: {html}"
    );
}

#[test]
fn provider_explicit_native_fallback_and_absent_destination_are_preserved() {
    let html = render(|| {
        rsx! {
            LinkProvider { component: router,
                Link { href: "/provider", "Provider" }
                Link { href: "/explicit", render_link: explicit, "Explicit" }
                Link { href: "/disabled", is_disabled: true, "Disabled" }
                InteractiveRoleProvider { role: InteractiveRole::Link,
                    Link { "Inherited without destination" }
                }
            }
            Link { href: "/native", "Native" }
        }
    });
    assert_eq!(
        html.matches("data-router=\"provider\"").count(),
        2,
        "{html}"
    );
    assert_eq!(
        html.matches("data-router=\"explicit\"").count(),
        1,
        "{html}"
    );
    assert!(
        html.contains("data-has-destination=\"false\""),
        "None must reach the router: {html}"
    );
    assert!(html.contains("href=\"/native\""), "{html}");
    assert!(
        !html.contains("href=\"\""),
        "absence must not become an empty destination: {html}"
    );
}

#[test]
fn role_precedence_is_destination_then_callback_then_context_and_disabled_href_is_skipped() {
    let html = render(|| {
        rsx! {
            InteractiveRoleProvider { role: InteractiveRole::Button,
                Link { href: "/enabled", "Destination wins" }
                Link { href: "/disabled", is_disabled: true, "Owner button" }
            }
            InteractiveRoleProvider { role: InteractiveRole::Link,
                Link { onclick: move |_| {}, "Callback wins" }
                InteractiveRoleProvider { role: None, Link { "Owner cleared" } }
            }
        }
    });
    assert_eq!(html.matches("<a ").count(), 1, "{html}");
    assert_eq!(html.matches("<button ").count(), 3, "{html}");
    assert!(html.contains("href=\"/enabled\""), "{html}");
    assert!(!html.contains("href=\"/disabled\""), "{html}");
}

#[test]
fn external_and_explicit_blank_targets_merge_safety_without_erasing_caller_tokens() {
    let html = render(|| {
        rsx! {
            Link { href: "https://example.test", is_external_link: true, target: "_self", rel: "help noopener", new_tab_label: "(nouvel onglet)", "Example" }
            Link { href: "/new", target: "_blank", rel: "noreferrer", "New" }
            Link { is_external_link: true, onclick: move |_| {}, "Action not navigation" }
        }
    });
    assert_eq!(html.matches("target=\"_blank\"").count(), 2, "{html}");
    assert!(html.contains("rel=\"help noopener noreferrer\""), "{html}");
    assert!(html.contains("rel=\"noreferrer noopener\""), "{html}");
    assert_eq!(
        html.matches("astryx-link__external").count(),
        1,
        "only external navigation gets a glyph: {html}"
    );
    assert!(html.contains("data-size=\"xsm\""), "{html}");
    assert!(html.contains("(nouvel onglet)"), "{html}");
}

#[test]
fn rejected_destinations_are_inert_before_router_handoff_but_safe_custom_destinations_pass_verbatim()
 {
    let html = render(|| {
        rsx! {
            LinkProvider { component: router,
                Link { href: "java\nscript:alert(1)", onclick: move |_| {}, "Rejected" }
                Link { href: "VBScript:run()", "Rejected VB" }
                Link { href: "data: text/html,hello", "Rejected HTML" }
                Link { href: " app:route ", "Safe application protocol" }
                Link { href: "data:text/plain,hello", download: LinkDownload::Enabled, "Safe download" }
            }
        }
    });
    assert_eq!(
        html.matches("data-router=\"provider\"").count(),
        2,
        "{html}"
    );
    assert_eq!(html.matches("aria-disabled=\"true\"").count(), 3, "{html}");
    assert!(!html.contains("javascript:"), "{html}");
    assert!(
        html.contains("href=\" app:route \""),
        "accepted values are not rewritten: {html}"
    );
    assert!(html.contains("href=\"data:text/plain,hello\""), "{html}");
}

#[test]
fn native_download_referrer_and_aria_attributes_reach_the_root_and_owned_disabled_state_wins() {
    let html = render(|| {
        rsx! {
            Link { href: "/report", download: LinkDownload::FileName("report.pdf".into()), referrer_policy: "no-referrer", "aria-current": "page", id: "report", class: "caller", style: "margin:2px", "Report" }
            Link { href: "/disabled", is_disabled: true, role: "button", "aria-disabled": "false", tabindex: -1, "data-color": "wrong", "Disabled" }
        }
    });
    for value in [
        "download=\"report.pdf\"",
        "referrerpolicy=\"no-referrer\"",
        "aria-current=\"page\"",
        "id=\"report\"",
        "caller",
        "margin:2px",
        "role=\"link\"",
        "aria-disabled=\"true\"",
        "tabindex=-1",
    ] {
        assert!(html.contains(value), "missing {value}: {html}");
    }
    assert!(!html.contains("wrong"), "{html}");
    assert!(!html.contains("role=\"button\""), "{html}");
}

#[test]
fn caller_names_override_optional_label_and_tooltips_preserve_description_ids() {
    let html = render(|| {
        rsx! {
            Link { href: "/settings", label: "Optional label", "aria-label": "Caller name", tooltip: "Additional help", "aria-describedby": "existing", "Visible settings" }
            Link { href: "/same", label: "Same", tooltip: "Same", "Visible" }
        }
    });
    assert!(html.contains("aria-label=\"Caller name\""), "{html}");
    assert!(!html.contains("aria-label=\"Optional label\""), "{html}");
    assert!(html.contains("aria-describedby=\"existing "), "{html}");
    assert_eq!(
        html.matches("aria-describedby=").count(),
        1,
        "same-name hint is not repeated: {html}"
    );
    assert_eq!(html.matches("role=\"tooltip\"").count(), 2, "{html}");
}

#[test]
fn own_typography_clamp_and_outer_layer_precedence_stay_with_text() {
    let html = render(|| {
        rsx! {
            Text { max_lines: 2,
                Link { href: "/flow", "Flow with the paragraph" }
                Link { href: "/own", max_lines: 1, "Own clamp" }
                Link { href: "/block", display: TextDisplay::Block, "Own block" }
                Link { onclick: move |_| {}, "Button breaks outer flow" }
                LayerContentBoundary { Link { href: "/layer", "Layer must reset clamp" } }
            }
            Link { href: "/type", text_type: TextType::LARGE, size: TextSize::TwoExtraLarge, weight: TextWeight::Bold, color: TextColor::SECONDARY, is_standalone: true, has_underline: true, "Typed" }
        }
    });
    assert_eq!(
        html.matches("astryx-link--in-clamp").count(),
        1,
        "only the flowing anchor inherits clamp: {html}"
    );
    for value in [
        "astryx-text--size-2xl",
        "astryx-text--weight-bold",
        "astryx-link--standalone",
        "astryx-link--underline",
        "data-type=\"large\"",
        "data-color=\"secondary\"",
    ] {
        assert!(html.contains(value), "missing {value}: {html}");
    }
}
