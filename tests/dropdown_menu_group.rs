use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::DropdownMenuGroup;

#[test]
fn title_owns_the_group_name_and_keeps_row_order() {
    let html = render_element(rsx! { DropdownMenuGroup {
        id: "history", role: "menuitem", "aria-labelledby": "caller", title: rsx! { span { "Version history" } },
        button { "Restore" }
    } });
    let (_, labelled_by) = html.split_once("aria-labelledby=\"").unwrap();
    let heading_id = labelled_by.split('"').next().unwrap();
    assert!(html.starts_with("<div role=\"group\""), "{html}");
    assert!(
        html.contains("id=\"history\"") && !html.contains("caller") && !html.contains("menuitem")
    );
    assert!(html.contains(&format!(
        "id=\"{heading_id}\" class=\"astryx-dropdown-menu-section-heading\""
    )));
    assert!(html.find("Version history") < html.find("Restore"));
}

#[test]
fn empty_title_does_not_override_the_caller_name() {
    let html = render_element(
        rsx! { DropdownMenuGroup { title: rsx! { if false { "Empty" } }, "aria-labelledby": "caller" } },
    );
    assert!(html.contains("role=\"group\"") && html.contains("aria-labelledby=\"caller\""));
    assert!(!html.contains("astryx-dropdown-menu-section-heading"));
    let unnamed = render_element(rsx! { DropdownMenuGroup {} });
    assert_eq!(unnamed, "<div role=\"group\"></div>");
}
