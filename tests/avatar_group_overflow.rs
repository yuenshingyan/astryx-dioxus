use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::AvatarGroupOverflow;

#[test]
fn negative_count_is_clamped_and_owned_name_wins() {
    let html = render_element(
        rsx! { AvatarGroupOverflow { count: -3, "aria-label": "Ignored", id: "overflow" } },
    );
    for expected in [
        "<span",
        "role=\"img\"",
        "aria-label=\"0 more\"",
        "data-size=\"md\"",
        "data-shape=\"circle\"",
        "id=\"overflow\"",
        "+0",
    ] {
        assert!(html.contains(expected), "{html}");
    }
    assert!(!html.contains("Ignored"));
}

#[test]
fn custom_content_keeps_count_name_and_caller_role() {
    let html = render_element(rsx! { AvatarGroupOverflow { count: 4912, role: "note", "Custom" } });
    assert!(
        html.contains("aria-label=\"4,912 more\"")
            && html.contains("role=\"note\"")
            && html.contains("Custom"),
        "{html}"
    );
    assert!(!html.contains("+4912"));
}
