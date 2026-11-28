use dioxus::core::Runtime;
use dioxus::prelude::*;
use dioxus_ssr::{render, render_element};

use astryx_dioxus::{
    Avatar, AvatarPixelSize, AvatarShape, AvatarSize, AvatarStatusDot, AvatarTooltip,
};

#[test]
fn avatar_skips_empty_image_sources() {
    let fallback = render_element(rsx! { Avatar { src: "", fallback_src: "/fallback.png" } });
    assert_eq!(fallback.matches("<img ").count(), 1, "{fallback}");
    assert!(fallback.contains("src=\"/fallback.png\""), "{fallback}");

    let icon = render_element(rsx! { Avatar { src: "", fallback_src: "" } });
    assert!(!icon.contains("<img "), "{icon}");
    assert!(icon.contains("<svg "), "{icon}");
}

#[test]
fn static_avatar_renders_identity_and_explicit_status() {
    let html = render_element(rsx! { Avatar {
        name: "Jane Doe", alt: "Jane's profile", status_label: "Online",
        shape: AvatarShape::Rounded, size: AvatarSize::Pixels(AvatarPixelSize::Px72),
        tooltip: AvatarTooltip::None,
    } });
    for expected in [
        "role=\"img\"",
        "Jane&#39;s profile, Online",
        "data-shape=\"rounded\"",
        "data-size=\"72\"",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert!(!html.contains("tabindex="));
}

#[test]
fn anonymous_avatar_is_decorative_and_caller_name_takes_precedence() {
    let anonymous = render_element(rsx! { Avatar {} });
    assert!(
        anonymous.contains("role=\"presentation\"") && anonymous.contains("aria-hidden=\"true\"")
    );
    let named =
        render_element(rsx! { Avatar { name: "Jane Doe", "aria-labelledby": "caller-name" } });
    assert!(named.contains("aria-labelledby=\"caller-name\"") && !named.contains("aria-label="));
}

fn avatar_root<'a>(html: &'a str, id: &str) -> &'a str {
    let attr = format!(" id=\"{id}\"");
    html.split('<')
        .filter_map(|part| part.split_once('>').map(|(tag, _)| tag))
        .find(|tag| tag.contains(&attr))
        .unwrap_or_else(|| panic!("missing avatar {id}: {html}"))
}

#[test]
fn explicit_status_names_the_ssr_root_and_respects_caller_aria() {
    let html = render_element(rsx! {
        Avatar { id: "status-only", name: "  ", status_label: "Online",
            status: rsx! { AvatarStatusDot { label: "Waiting" } }
        }
        Avatar { id: "caller", name: "Jane", status_label: "Online", "aria-labelledby": "caption",
            status: rsx! { AvatarStatusDot { label: "Waiting" } }
        }
        span { id: "caption", "Caller name" }
    });
    let root = avatar_root(&html, "status-only");
    assert!(
        root.contains("role=\"img\"") && root.contains("aria-label=\"Online\""),
        "{root}"
    );
    assert!(!root.contains("aria-hidden="), "{root}");
    let root = avatar_root(&html, "caller");
    assert!(
        root.contains("aria-labelledby=\"caption\"") && !root.contains("aria-label="),
        "{root}"
    );
}

#[component]
fn WrappedStatus(status: Signal<Option<&'static str>>) -> Element {
    rsx! { span { if let Some(label) = status() { AvatarStatusDot { label } } } }
}

fn status_lifecycle() -> Element {
    let status = use_signal(|| Some("Online"));
    use_context_provider(|| status);
    rsx! {
        Avatar { id: "direct", name: "Jane", alt: "Profile",
            status: rsx! { if let Some(label) = status() { AvatarStatusDot { label } } }
        }
        Avatar { id: "wrapped", status: rsx! { WrappedStatus { status } } }
        Avatar { id: "override", "aria-label": "Caller name",
            status: rsx! { WrappedStatus { status } }
        }
    }
}

#[test]
fn reported_status_names_follow_mount_change_clear_removal_and_remount() {
    let mut dom = VirtualDom::new(status_lifecycle);
    dom.rebuild_in_place();
    let mut status = dom.in_runtime(|| {
        Runtime::current()
            .consume_context::<Signal<Option<&'static str>>>(ScopeId::APP)
            .unwrap()
    });
    for (label, expected_status) in [
        (Some("Online"), Some("Online")),
        (Some("Away"), Some("Away")),
        (Some("  "), None),
        (Some("Busy"), Some("Busy")),
        (None, None),
        (Some("Online"), Some("Online")),
    ] {
        dom.in_runtime(|| status.set(label));
        // Apply the child update, then let each of the three reporting effects
        // schedule its Avatar render. Dioxus can defer later effects when a scope becomes dirty.
        for _ in 0..4 {
            dom.render_immediate_to_vec();
        }
        let html = render(&dom);
        let direct = avatar_root(&html, "direct");
        let name =
            expected_status.map_or_else(|| "Profile".into(), |label| format!("Profile, {label}"));
        assert!(
            direct.contains(&format!("aria-label=\"{name}\"")),
            "{direct}"
        );
        let wrapped = avatar_root(&html, "wrapped");
        if let Some(label) = expected_status {
            assert!(
                wrapped.contains(&format!("aria-label=\"{label}\"")),
                "{wrapped}"
            );
            assert!(
                wrapped.contains("role=\"img\"") && !wrapped.contains("aria-hidden="),
                "{wrapped}"
            );
        } else {
            assert!(!wrapped.contains("aria-label="), "{wrapped}");
            assert!(
                wrapped.contains("role=\"presentation\"")
                    && wrapped.contains("aria-hidden=\"true\""),
                "{wrapped}"
            );
        }
        let caller = avatar_root(&html, "override");
        assert!(caller.contains("aria-label=\"Caller name\""), "{caller}");
    }
}
