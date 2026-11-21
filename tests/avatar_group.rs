use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{Avatar, AvatarGroup, AvatarGroupOverflow, AvatarShape, AvatarSize};

#[test]
fn group_overrides_member_shape_and_size_without_adding_static_tab_stops() {
    let html = render_element(rsx! { AvatarGroup {
        size: AvatarSize::Large, shape: AvatarShape::Rounded, "aria-label": "Reviewers",
        Avatar { name: "Alex", size: AvatarSize::Small }
        AvatarGroupOverflow { count: 3 }
    } });
    assert!(html.contains("role=\"group\"") && html.contains("aria-label=\"Reviewers\""));
    assert_eq!(html.matches("data-shape=\"rounded\"").count(), 3);
    assert!(html.contains("--_avatar-size:48px") && html.contains("--_avatar-overflow-size:48px"));
    assert!(!html.contains("tabindex="));
}
