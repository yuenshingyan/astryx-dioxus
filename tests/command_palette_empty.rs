use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::CommandPaletteEmpty;

#[test]
fn empty_surface_preserves_caller_content_without_adding_semantics() {
    let html = render_element(rsx! {
        CommandPaletteEmpty { id: "empty", class: "custom", style: "padding:0", "data-testid": "empty-results",
            a { href: "/help", "Search help" }
        }
    });
    assert!(html.starts_with("<div "), "{html}");
    for expected in [
        "id=\"empty\"",
        "astryx-command-palette-empty custom",
        "style=\"padding:0\"",
        "data-testid=\"empty-results\"",
        "<a href=\"/help\">Search help</a>",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    for unexpected in ["role=", "aria-live=", "tabindex="] {
        assert!(
            !html.contains(unexpected),
            "unexpected {unexpected}: {html}"
        );
    }
}
