use astryx_dioxus::{Avatar, AvatarSize, AvatarStatusDot, AvatarStatusDotVariant};
use dioxus::prelude::*;
use dioxus_ssr::render_element;

#[path = "../examples/showcase/avatar_status_dot.rs"]
mod showcase;

use showcase::AvatarStatusDotExamples;

#[component]
fn EmptyIcon() -> Element {
    VNode::empty()
}

#[test]
fn icon_replacement_is_shallow_and_respects_avatar_size() {
    for (size, diameter, border, icon_size) in [
        (AvatarSize::Medium, 10, 1, 0),
        (AvatarSize::Large, 20, 2, 12),
        (AvatarSize::ExtraLarge, 32, 4, 18),
    ] {
        let html = render_element(rsx! {
            Avatar { size, status: rsx! {
                AvatarStatusDot { variant: AvatarStatusDotVariant::NEUTRAL,
                    icon: rsx! { svg { id: "replacement" } },
                }
            } }
        });
        assert!(
            html.contains(&format!(
                "--_avatar-dot-size:{diameter}px;--_avatar-dot-border:{border}px"
            )),
            "{html}"
        );
        assert_eq!(
            html.contains("data-shape=\"ring\""),
            icon_size == 0,
            "{html}"
        );
        assert_eq!(
            html.contains("id=\"replacement\""),
            icon_size != 0,
            "{html}"
        );
        if icon_size != 0 {
            assert!(html.contains(&format!("class=\"astryx-avatar-status-dot-icon\" aria-hidden=\"true\" style=\"width:{icon_size}px;height:{icon_size}px\"")), "{html}");
        }
    }

    let empty_text = String::new();
    for icon in [
        VNode::empty(),
        rsx! { "{empty_text}" },
        rsx! { if false { svg {} } },
    ] {
        let html = render_element(rsx! {
            Avatar { size: AvatarSize::Large, status: rsx! {
                AvatarStatusDot { variant: AvatarStatusDotVariant::ERROR, icon: icon.clone() }
            } }
        });
        assert!(html.contains("data-shape=\"minus\""), "{html}");
        assert!(!html.contains("astryx-avatar-status-dot-icon"), "{html}");
    }

    let html = render_element(rsx! {
        Avatar { size: AvatarSize::Large, status: rsx! {
            AvatarStatusDot { variant: AvatarStatusDotVariant::ERROR, icon: rsx! { EmptyIcon {} } }
        } }
    });
    assert!(!html.contains("data-shape=\"minus\""), "{html}");
    assert!(html.contains("astryx-avatar-status-dot-icon"), "{html}");
}

#[test]
fn standalone_root_preserves_public_attributes_and_variant_targets() {
    let html = render_element(rsx! {
        AvatarStatusDot { id: "presence", label: "Online", role: "note", "aria-label": "ignored",
            class: "caller-class", style: "color:rebeccapurple;", "data-variant": "ignored",
        }
    });
    for expected in [
        "id=\"presence\"",
        "role=\"img\"",
        "aria-label=\"Online\"",
        "data-variant=\"success\"",
        "astryx-avatar-status-dot caller-class",
        "--_avatar-dot-size:10px;--_avatar-dot-border:1px;color:rebeccapurple;",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert!(!html.contains("ignored"), "{html}");
    assert!(!html.contains("<svg"), "{html}");

    let variant = AvatarStatusDotVariant::new("custom-presence").unwrap();
    let html = render_element(rsx! { AvatarStatusDot { variant, label: "" } });
    assert!(html.contains("data-variant=\"custom-presence\""), "{html}");
    for absent in ["role=", "aria-label=", "data-shape=", "<svg"] {
        assert!(!html.contains(absent), "{html}");
    }
}

#[test]
fn showcase_renders_both_pinned_patterns_in_avatar_status_slots() {
    let html = render_element(rsx! { AvatarStatusDotExamples {} });
    for (id, pairs) in [
        (
            "avatar-dot-showcase",
            [
                ("Online User", "Online"),
                ("Away User", "Away"),
                ("Busy User", "Busy"),
            ],
        ),
        (
            "avatar-dot-variants",
            [
                ("Ana Silva", "Online"),
                ("Ben Okafor", "Away"),
                ("Cleo Marsh", "Do not disturb"),
            ],
        ),
    ] {
        let marker = format!("id=\"{id}\"");
        let (before, after) = html.split_once(&marker).unwrap();
        let row_start = before.rsplit_once("<div ").unwrap().1;
        let row = after.split_once("</h3>").unwrap().0;
        assert!(
            row_start.contains("astryx-stack--align-center"),
            "{row_start}"
        );
        assert!(
            row_start.contains("--_stack-gap:var(--spacing-4)"),
            "{row_start}"
        );
        for ((name, label), variant) in pairs.into_iter().zip(["success", "neutral", "error"]) {
            let name = format!("aria-label=\"{name}\"");
            let avatar = row.split_once(&name).unwrap().1;
            let avatar = avatar
                .split_once("class=\"astryx-avatar astryx-avatar--public")
                .map_or(avatar, |(slot, _)| slot);
            for expected in [
                "class=\"astryx-avatar__status\"".to_string(),
                format!("aria-label=\"{label}\""),
                format!("data-variant=\"{variant}\""),
                "--_avatar-dot-size:32px;--_avatar-dot-border:4px".to_string(),
            ] {
                assert!(avatar.contains(&expected), "{avatar}");
            }
        }
    }
}
