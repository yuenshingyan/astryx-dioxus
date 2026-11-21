use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Avatar, AvatarPixelSize, AvatarShape, AvatarSize, AvatarTooltip};

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
