use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::CommandPaletteGroup;

#[test]
fn heading_names_group_without_repeating_in_accessibility_tree() {
    let html =
        render_element(rsx! { CommandPaletteGroup { heading: "Navigation", button { "Home" } } });
    assert!(
        html.contains("role=\"group\"") && html.contains("aria-label=\"Navigation\""),
        "{html}"
    );
    assert!(html.contains("class=\"astryx-command-palette-group-heading\" aria-hidden=\"true\">Navigation</div><button"), "{html}");
}

#[test]
fn root_keeps_caller_overrides() {
    let html = render_element(
        rsx! { CommandPaletteGroup { heading: "Actions", role: "region", "aria-label": "Shortcuts", id: "actions", class: "custom", style: "gap:0", span { "Save" } } },
    );
    for expected in [
        "role=\"region\"",
        "aria-label=\"Shortcuts\"",
        "id=\"actions\"",
        "astryx-command-palette-group custom",
        "style=\"gap:0\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert_eq!(html.matches("aria-label=").count(), 1, "{html}");
    assert!(!html.contains("role=\"group\""), "{html}");
}
