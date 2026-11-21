use dioxus::prelude::*;
use dioxus_ssr::render_element;

use astryx_dioxus::CommandPaletteFooter;

#[test]
fn default_hints_use_kbd_and_accept_localized_text() {
    let html = render_element(rsx! { CommandPaletteFooter { navigate_label: "Naviguer" } });
    assert_eq!(html.matches("<kbd ").count(), 4, "{html}");
    for text in ["Naviguer", "Select", "Close"] {
        assert!(html.contains(text), "missing {text}: {html}");
    }
    let root = html.split_once('>').unwrap().0;
    assert!(
        !root.contains("role=") && !root.contains("tabindex="),
        "{root}"
    );
}

#[test]
fn custom_content_replaces_defaults_and_root_attributes_are_forwarded() {
    let html = render_element(
        rsx! { CommandPaletteFooter { id: "guidance", class: "custom", style: "gap:0", a { href: "/help", "Help" } } },
    );
    for expected in [
        "astryx-command-palette-footer custom",
        "id=\"guidance\"",
        "style=\"gap:0\"",
        "<a href=\"/help\">Help</a>",
    ] {
        assert!(html.contains(expected), "missing {expected}: {html}");
    }
    assert!(
        !html.contains("<kbd") && !html.contains("Navigate"),
        "{html}"
    );
}
