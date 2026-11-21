use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::CommandPaletteItem;

#[test]
fn option_reflects_independent_states_and_owns_its_role() {
    let html = render_element(rsx! {
        CommandPaletteItem { id: "publish", value: "publish", is_selected: true, is_highlighted: true, is_disabled: true,
            role: "button", "aria-selected": "false", "Publish"
        }
    });
    for expected in [
        "id=\"publish\"",
        "role=\"option\"",
        "aria-selected=\"true\"",
        "aria-disabled=\"true\"",
        "data-value=\"publish\"",
        "data-highlighted=\"highlighted\"",
        "data-selected=\"selected\"",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(
        !html.contains("role=\"button\"") && !html.contains("tabindex="),
        "{html}"
    );
}
