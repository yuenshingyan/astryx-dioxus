use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::CommandPaletteList;

#[test]
fn standalone_list_owns_its_role_and_label() {
    let html = render_element(rsx! {
        CommandPaletteList { label: "Files", id: "results", class: "custom", role: "group", "aria-label": "Not the name",
            div { role: "option", "aria-selected": "false", "README" }
        }
    });
    for expected in [
        "role=\"listbox\"",
        "aria-label=\"Files\"",
        "id=\"results\"",
        "astryx-command-palette-list custom",
        "role=\"option\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(
        !html.contains("Not the name") && !html.contains("role=\"group\""),
        "{html}"
    );
}

#[test]
fn standalone_list_has_a_default_name() {
    let html = render_element(rsx! { CommandPaletteList { div { role: "option", "Open" } } });
    assert!(html.contains("aria-label=\"Commands\""), "{html}");
}
