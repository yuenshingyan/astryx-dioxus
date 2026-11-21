use dioxus::prelude::*;

use astryx_dioxus::{CommandPaletteFooter, Text, TextColor, TextType};

#[component]
pub(super) fn CommandPaletteFooterExamples() -> Element {
    rsx! {
        section { id: "command-palette-footer-examples", "data-astryx-theme": "neutral",
            h2 { "CommandPaletteFooter" }
            h3 { "Basic" }
            CommandPaletteFooter { id: "command-footer-default", style: "max-width:240px" }
            h3 { "Showcase" }
            CommandPaletteFooter { id: "command-footer-custom",
                Text { text_type: TextType::SUPPORTING, color: TextColor::SECONDARY,
                    "Tip: Press ⌘K anywhere to open the command palette"
                }
            }
        }
    }
}
