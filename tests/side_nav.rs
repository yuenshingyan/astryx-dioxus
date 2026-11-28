use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::SideNav;

#[path = "../examples/showcase/side_nav.rs"]
mod showcase;

use showcase::SideNavExamples;

#[test]
fn omitted_native_label_keeps_the_component_label() {
    let default = render_element(rsx! { SideNav { "Items" } });
    assert!(default.starts_with("<nav "), "{default}");
    assert!(
        default.contains("aria-label=\"Side navigation\""),
        "{default}"
    );

    let unset_label: Option<String> = None;
    let custom = render_element(rsx! {
        SideNav { label: "Project navigation", "aria-label": unset_label, "Items" }
    });
    assert!(
        custom.contains("aria-label=\"Project navigation\""),
        "{custom}"
    );
}

#[test]
fn native_label_overrides_the_fallback_without_replacing_the_landmark() {
    let html = render_element(rsx! {
        SideNav { label: "Fallback", id: "project-nav", class: "custom-rail",
            style: "inline-size:18rem", role: "menu", "aria-label": "Project navigation",
            "aria-labelledby": "project-title",
            "Items"
        }
    });
    let (root, _) = html.split_once('>').expect("opening tag");
    assert!(root.starts_with("<nav "), "{root}");
    for attr in [
        "id=\"project-nav\"",
        "class=\"astryx-side-nav custom-rail\"",
        "style=\"inline-size:18rem\"",
        "aria-label=\"Project navigation\"",
        "aria-labelledby=\"project-title\"",
    ] {
        assert!(root.contains(attr), "missing {attr}: {root}");
    }
    assert_eq!(root.matches("aria-label=").count(), 1, "{root}");
    assert!(
        !root.contains("role=") && !root.contains("Fallback"),
        "{root}"
    );
}

#[test]
fn showcase_uses_a_native_name_and_keeps_the_current_destination() {
    let html = render_element(rsx! { SideNavExamples {} });
    let rail = html.split_once("<nav ").expect("navigation landmark").1;
    let (name, content) = rail.split_once('>').expect("navigation opening tag");
    assert!(name.contains("id=\"workspace-side-nav\""), "{name}");
    assert!(
        name.contains("aria-label=\"Workspace navigation\""),
        "{name}"
    );
    assert_eq!(
        content.matches("aria-current=\"page\"").count(),
        1,
        "{content}"
    );
}
