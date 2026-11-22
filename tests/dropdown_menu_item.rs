use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::{DropdownMenuItem, DropdownMenuItemVariant};

#[test]
fn action_row_owns_menu_semantics_and_preserves_content() {
    let html = render_element(rsx! {
        DropdownMenuItem { id: "archive", label: "Archive", description: Some("Move to storage".into()),
            variant: DropdownMenuItemVariant::Destructive, is_disabled: true,
            role: "button", tabindex: "0",
        }
    });
    for expected in [
        "id=\"archive\"",
        "role=\"menuitem\"",
        "tabindex=\"-1\"",
        "aria-disabled=\"true\"",
        "data-variant=\"destructive\"",
        "Move to storage",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(!html.contains("role=\"button\"") && !html.contains("tabindex=\"0\""));
}
