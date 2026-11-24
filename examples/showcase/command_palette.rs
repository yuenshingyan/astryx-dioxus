use dioxus::prelude::*;

use astryx_dioxus::{Button, CommandPalette, CommandPaletteOption};

fn options() -> Vec<CommandPaletteOption> {
    [
        ("home", "Home"),
        ("settings", "Settings"),
        ("profile", "Profile"),
        ("dashboard", "Dashboard"),
        ("help", "Help"),
    ]
    .into_iter()
    .map(|(value, label)| CommandPaletteOption {
        value: value.into(),
        label: label.into(),
    })
    .collect()
}

#[component]
pub(super) fn CommandPaletteExamples() -> Element {
    let mut open = use_signal(|| false);
    let mut selected = use_signal(String::new);
    rsx! {
        section { id: "command-palette-examples",
            h2 { "Command Palette" }
            CommandPalette { is_inline: true, is_open: true, on_open_change: |_| {}, options: options(), on_select: move |value| selected.set(value) }
            Button { id: "command-palette-trigger", label: "Open command palette", onclick: move |_| open.set(true) }
            CommandPalette { is_open: open(), on_open_change: move |next| open.set(next), options: options(), on_select: move |value| selected.set(value) }
            output { "{selected()}" }
        }
    }
}
