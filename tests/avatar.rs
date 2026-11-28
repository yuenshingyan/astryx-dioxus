use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Avatar, AvatarPixelSize, AvatarShape, AvatarSize, AvatarTooltip};

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
