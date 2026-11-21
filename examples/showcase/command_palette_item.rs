use dioxus::prelude::*;

use astryx_dioxus::{CommandPaletteItem, CommandPaletteList, Kbd};

#[component]
pub(super) fn CommandPaletteItemExamples() -> Element {
    let mut picked = use_signal(String::new);
    let on_select = EventHandler::new(move |value| picked.set(value));
    rsx! {
        section { id: "command-palette-item-examples", "data-astryx-theme": "neutral",
            h2 { "CommandPaletteItem" }
            p { "Standalone pointer examples. Keyboard navigation is owned by the parent palette." }
            h3 { "Basic" }
            CommandPaletteList { style: "max-width:360px",
                CommandPaletteItem { value: "new-file", on_select, "New File" }
                CommandPaletteItem { value: "open-recent", on_select, is_highlighted: true, "Open Recent" }
                CommandPaletteItem { value: "save-all", on_select, "Save All" }
                CommandPaletteItem { value: "publish", on_select, is_disabled: true, "Publish (disabled)" }
            }
            h3 { "Showcase" }
            CommandPaletteList { style: "max-width:360px",
                for (value, label, keys) in [
                    ("save", "Save File", "mod+s"),
                    ("find", "Find in Files", "mod+shift+f"),
                    ("palette", "Command Palette", "mod+shift+p"),
                    ("terminal", "Toggle Terminal", "ctrl+`"),
                    ("sidebar", "Toggle Sidebar", "mod+b"),
                ] {
                    CommandPaletteItem { key: "{value}", value, on_select, is_selected: picked() == value,
                        span { style: "flex:1", "{label}" }
                        Kbd { keys }
                    }
                }
            }
            p { id: "command-palette-item-picked", "{picked}" }
        }
    }
}
